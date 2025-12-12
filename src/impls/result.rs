use fluent_result::bool::Then;

use crate::{ResultIterError, TryFromIterator};

/// Iterator adaptor that extracts [`Ok`] values from a [`Result`] [`Iterator`],
/// storing the first encountered [`Err`] for later retrieval.
#[derive(Debug)]
pub struct ExtractErr<'a, I, E> {
    iter: I,
    error: &'a mut Option<E>,
}

/// No more items hint for `ExtractErr`.
const NO_MORE_ITEMS: (usize, Option<usize>) = (0, Some(0));

/// Implements Iterator for `ExtractErr`, yielding Ok values and capturing the first Err.
///
/// Once an Err is encountered, the iterator terminates and stores the error.
impl<I, T, E> Iterator for ExtractErr<'_, I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        //self.error.borrow().is_some().then_none()?;
        self.error.is_some().then_none()?;

        match self.iter.next() {
            None => None,
            Some(Ok(v)) => Some(v),
            Some(Err(e)) => {
                //*self.error.borrow_mut() = Some(e);
                *self.error = Some(e);
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        //match self.error.borrow().is_some() {
        match self.error.is_some() {
            true => NO_MORE_ITEMS,
            false => (0, self.iter.size_hint().1),
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
    C: TryFromIterator<T, ExtractErr<'static, I::IntoIter, E>>,
{
    type Error = ResultIterError<E, C, C::Error>;

    /// Converts an iterator of `Result<A, E>` into a `Result<V, E>`.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
    /// ```
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter_error = None;
        // SAFETY: We transmute the lifetime here to work around the self-referential structure.
        // This is safe because:
        // 1. `iter_error` is created before `extractor` and lives until the end of the function
        // 2. `extractor` is consumed by `C::try_from_iter` and never used again
        // 3. We only access `iter_error` after `extractor` has been consumed
        let iter_error_ref: &'static mut Option<E> = unsafe { std::mem::transmute(&mut iter_error) };
        let extractor = ExtractErr { iter: into_iter.into_iter(), error: iter_error_ref };

        let try_from_result = C::try_from_iter(extractor);

        match (iter_error.take(), try_from_result) {
            (None, Ok(v)) => Ok(Ok(v)),
            (None, Err(e)) => Ok(Err(e)),
            (Some(iter_err), coll_result) => Err(ResultIterError::new(iter_err, coll_result)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [Result<i32, i32>; 4] = [Ok(1), Err(3), Ok(3), Ok(4)];

    #[test]
    fn extract_err_zero_size_after_err() {
        let mut error = None;
        let mut extractor = ExtractErr { iter: TEST_DATA.into_iter(), error: &mut error };

        extractor.next();
        extractor.next();
        extractor.next();

        assert_eq!(extractor.size_hint(), (0, Some(0)));
    }

    #[test]
    fn extract_err_forward_hint() {
        let mut error = None;
        let mut extractor = ExtractErr { iter: TEST_DATA.into_iter(), error: &mut error };

        extractor.next();

        assert_eq!(extractor.size_hint(), (0, Some(3)));
    }
}
