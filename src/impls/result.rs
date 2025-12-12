use std::sync::{Arc, Mutex};

use fluent_result::into::IntoOption;
use tap::Pipe;

use crate::{ResultIterError, TryFromIterator};

/// Iterator adaptor that extracts [`Ok`] values from a [`Result`] [`Iterator`],
/// storing the first encountered [`Err`] and remaining iterator for later retrieval.
#[subdef::subdef]
#[derive(Debug)]
pub struct ExtractErr<I, E>(
    [Arc<Mutex<Option<ExtractErrState<I, E>>>>; {
        #[derive(Debug)]
        struct ExtractErrState<I, E> {
            iter: I,
            state: [ErrState<E>; {
                #[derive(Debug)]
                enum ErrState<E> {
                    Active,
                    Errored(E),
                }
            }],
        }
    }],
);

/// No more items hint for `ExtractErr`.
const NO_MORE_ITEMS: (usize, Option<usize>) = (0, Some(0));

/// Implements Iterator for `ExtractErr`, yielding Ok values and capturing the first Err.
///
/// Once an Err is encountered, the iterator terminates and stores the error along with the remaining iterator.
impl<I, T, E> Iterator for ExtractErr<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.lock().unwrap().as_mut()? {
            ExtractErrState { state: ErrState::Errored(_), .. } => None,
            ExtractErrState { ref mut iter, ref mut state } => match iter.next() {
                None => None,
                Some(Ok(v)) => Some(v),
                Some(Err(e)) => {
                    *state = ErrState::Errored(e);
                    None
                }
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0.lock().unwrap().as_ref() {
            Some(ExtractErrState { state: ErrState::Errored(_), .. }) | None => NO_MORE_ITEMS,
            Some(ExtractErrState { iter, .. }) => (0, iter.size_hint().1),
        }
    }
}

/// Converts an iterator of [`Result<T, E>`] into a [`Result<C, E>`], where `C` implements [`TryFromIterator<T>`].
///
/// That is, given a iterator that yields [`Result<T, E>`], this implementation will collect all [`Ok`] values
/// into a container `C` that implements [`TryFromIterator<T>`], short-circuiting on the first [`Err`] encountered.
/// If an [`Err`] is found, it is returned immediately. If all values are [`Ok`], but the inner collection fails to
/// construct, that error is propagated.
///
/// # Type Parameters
///
/// - `T`: The type of the values in the iterator.
/// - `E`: The error type returned by the fallible extension methods.
/// - `C`: The type of the container to be constructed.
///
/// # Return Value
///
/// Returns a nested [`Result<Result<C, C::Error>, ResultIterError<E, C, C::Error>>`](ResultIterError).
/// The outer `Result` represents the result of iteration. Any [`Err`] value encountered during
/// iteration is stored in the returned [`ResultIterError`], which will also contain the results of
/// attempting to construct the container with the results of the partial iteration, which could be
/// [`Ok`] or [`Err`], depending on the container's [`TryFromIterator`] implementation.
///
/// The inner [`Result`] represents the result of the container construction.
///
/// Put another way, there are three possible states this function can return:
///
/// - `Ok(Ok(_))`: The iterator completed successfully and the container was successfully constructed.
/// - `Ok(Err(_))`: The iterator completed successfully, but the container construction failed.
/// - `Err(_)`: The iterator encountered an error before completion.
///
/// # Examples
///
/// ```rust
#[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
/// ```
/// Handling the nested `Result<Result<C, E>, C::Error>` may be cumbersome. Consider using
/// `fluent_result::nested::FlattenErr::flatten_err` or `fluent_result::nested::BoxErr::box_err` (from the
/// `fluent_result` crate) to flatten the error type for more ergonomic handling. Alternatively, if both error types
/// are convertable to the return type of the scope using [`From`], you can simply use two instances of the `??`
/// operator.
///
/// ```rust
#[doc = include_doc::function_body!("tests/doc/result.rs", double_question_mark_example, [])]
/// ```
///
/// ## Recovering Data
///
/// If the container type and error type are both [`IntoIterator`], you can use the [`IntoIterator`]
/// implementation to recover the data consumed by the iterator.
///
/// ```rust
#[doc = include_doc::function_body!("tests/doc/result.rs", error_recovery_example, [])]
/// ```
impl<T, I, E, C> TryFromIterator<Result<T, E>, I> for Result<C, C::Error>
where
    I: IntoIterator<Item = Result<T, E>>,
    C: TryFromIterator<T, ExtractErr<I::IntoIter, E>>,
{
    type Error = ResultIterError<E, C, C::Error, I::IntoIter>;

    /// Converts an iterator of `Result<A, E>` into a `Result<V, E>`.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
    /// ```
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let state = ExtractErrState { iter: into_iter.into_iter(), state: ErrState::Active }
            .into_some()
            .pipe(Mutex::new)
            .pipe(Arc::new);

        let extractor = ExtractErr(state.clone());

        let try_from_result = C::try_from_iter(extractor);

        let result = {
            match (state.lock().unwrap().take(), try_from_result) {
                (None, _) => unreachable!("state already extracted!?"),
                (Some(ExtractErrState { state: ErrState::Active, .. }), Ok(v)) => Ok(Ok(v)), // iter without err, and succesfull collect
                (Some(ExtractErrState { state: ErrState::Active, .. }), Err(e)) => Ok(Err(e)), // iter without err, but collect failed
                (Some(ExtractErrState { state: ErrState::Errored(error), iter }), collect_result) => {
                    // errored during iter,
                    Err(ResultIterError::new(error, collect_result, iter)) // collect may have succeded or failed.
                }
            }
        };
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [Result<i32, i32>; 4] = [Ok(1), Err(3), Ok(3), Ok(4)];

    #[test]
    fn extract_err_zero_size_after_err() {
        let mut extractor = ExtractErr(Arc::new(Mutex::new(Some(ExtractErrState {
            iter: TEST_DATA.into_iter(),
            state: ErrState::Active,
        }))));

        extractor.next(); // Ok(1)
        extractor.next(); // Err(3) - stops here
        extractor.next(); // Should return None

        assert_eq!(extractor.size_hint(), (0, Some(0)));
    }

    #[test]
    fn extract_err_forward_hint() {
        let mut extractor = ExtractErr(Arc::new(Mutex::new(Some(ExtractErrState {
            iter: TEST_DATA.into_iter(),
            state: ErrState::Active,
        }))));

        extractor.next(); // Ok(1)

        assert_eq!(extractor.size_hint(), (0, Some(3)));
    }
}
