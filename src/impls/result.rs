use std::iter;
use std::sync::{Arc, Mutex};

use fluent_result::bool::Then;
use itertools::Either;
use tap::Pipe;

use crate::{ResultCollectionError, TryFromIterator};

type ArcMutex<T> = Arc<Mutex<T>>;
type IterOrEmpty<I, T> = Either<I, iter::Empty<T>>;

/// Iterator adaptor that extracts [`Ok`] values from a [`Result`] [`Iterator`],
/// storing the first encountered [`Err`] and remaining iterator for later retrieval.
#[subdef::subdef(derive(Debug))]
pub struct ExtractErr<I: Iterator, E>(
    [ArcMutex<ExtractErrState<IterOrEmpty<I, I::Item>, E>>; {
        struct ExtractErrState<Iter, E> {
            iter: Iter,
            error: Option<E>,
        }
    }],
);

impl<Iter: Iterator, E> ExtractErrState<Iter, E> {
    const EMPTY_STATE: ExtractErrState<IterOrEmpty<Iter, Iter::Item>, E> =
        ExtractErrState { iter: Either::Right(iter::empty()), error: None };
}

impl<I, E, T> Default for ExtractErrState<IterOrEmpty<I, I::Item>, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    fn default() -> Self {
        ExtractErrState::EMPTY_STATE
    }
}

impl<I, E, T> ExtractErr<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    /// Creates a new `ExtractErr` wrapping the given iterator.
    fn new(iter: I) -> Self {
        ExtractErrState { iter: Either::Left(iter), error: None }.pipe(Mutex::new).pipe(Arc::new).pipe(ExtractErr)
    }

    /// Takes and returns the inner state, replacing it with an empty state.
    fn take_inner(&self) -> ExtractErrState<IterOrEmpty<I, I::Item>, E> {
        std::mem::take(&mut *self.0.lock().expect("Mutex should not be poisened"))
    }
}

impl<I, E, T> Clone for ExtractErr<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    fn clone(&self) -> Self {
        self.0.clone().pipe(ExtractErr)
    }
}

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
        let mut guard = self.0.lock().expect("Mutex should not be poisened");
        guard.error.is_some().then_none()?;
        match guard.iter.next() {
            None => None,
            Some(Ok(v)) => Some(v),
            Some(Err(e)) => {
                guard.error = Some(e);
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &*self.0.lock().expect("Mutex should not be poisened") {
            ExtractErrState { error: Some(_), .. } => NO_MORE_ITEMS,
            ExtractErrState { error: None, iter } => (0, iter.size_hint().1),
        }
    }
}

/// Converts an iterator of [`Result<T, E>`] into a [`Result<C, E>`], where `C` implements [`TryFromIterator<T>`].
///
/// That is, given a iterator that yields [`Result<T, E>`], this implementation will collect all [`Ok`] values
/// into a container `C` that implements [`TryFromIterator<T>`], short-circuiting on the first [`Err`] encountered.
/// If all iterator values are [`Ok`], then the outer [`Result`] will be [`Ok`], and the value of the inner [`Result`]
/// will be the result of the container construction.
///
/// # Type Parameters
///
/// - `I`: The type of the [`IntoIterator`], must produce [`Result<T, E>`].
/// - `T`: The type of [`Ok`] values in the `I::IntoIter` [`Iterator`].
/// - `E`: The type of [`Err`] values in the `I::IntoIter` [`Iterator`].
/// - `C`: The type of the container to be constructed, must implement [`TryFromIterator<T>`].
/// - `C::Error`: The [`TryFromIterator::Error`] from `C`'s [`TryFromIterator`] implementation.
///
/// # Return Value
///
/// There are three possible states this function can return:
///
/// - `Ok(Ok(_))`: The iterator completed successfully and the container was successfully constructed.
/// - `Ok(Err(_))`: The iterator completed successfully, but the container construction failed.
/// - `Err(_)`: The iterator encountered an error before completion.
///
/// The outer [`Result`] error type is [`ResultIterError`]. In the event of an error during iteration
/// (the last case above), this type will contain the error encountered, the remaining iterator, and the
/// result of the container construction over the values successfully collected (which could succeed or
/// fail, depending on the container type).
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
impl<I, T, E, C> TryFromIterator<Result<T, E>, I> for Result<C, C::Error>
where
    I: IntoIterator<Item = Result<T, E>>,
    C: TryFromIterator<T, ExtractErr<I::IntoIter, E>>,
{
    type Error = ResultCollectionError<E, C, C::Error, Either<I::IntoIter, iter::Empty<Result<T, E>>>>;

    /// Converts an iterator of `Result<A, E>` into a `Result<V, E>`.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
    /// ```
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let extractor = ExtractErr::new(into_iter.into_iter());

        let try_from_result = extractor.clone().pipe(C::try_from_iter);

        match (extractor.take_inner(), try_from_result) {
            (ExtractErrState { error: None, .. }, Ok(v)) => Ok(Ok(v)), // iter without err, and successful collect
            (ExtractErrState { error: None, .. }, Err(e)) => Ok(Err(e)), // iter without err, but collect failed
            // errored during iter, collect may have succeeded or failed.
            (ExtractErrState { error: Some(error), iter }, result) => {
                Err(ResultCollectionError::new(error, result, iter))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [Result<i32, i32>; 4] = [Ok(1), Err(3), Ok(3), Ok(4)];

    #[test]
    fn extract_err_zero_size_after_err() {
        let mut extractor = ExtractErr::new(TEST_DATA.into_iter());

        extractor.next(); // Ok(1)
        extractor.next(); // Err(3) - stops here
        extractor.next(); // Should return None

        assert_eq!(extractor.size_hint(), (0, Some(0)));
    }

    #[test]
    fn extract_err_forward_hint() {
        let mut extractor = ExtractErr::new(TEST_DATA.into_iter());

        extractor.next(); // Ok(1)

        assert_eq!(extractor.size_hint(), (0, Some(3)));
    }
}
