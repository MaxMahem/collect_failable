use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

use fluent_result::bool::Then;
use itertools::Either;
use tap::Pipe;

use crate::errors::ResultCollectionError;
use crate::TryFromIterator;

/// Iterator adaptor that extracts [`Ok`] values from a [`Result`] [`Iterator`],
/// storing the first encountered [`Err`] and remaining iterator for later retrieval.
///
/// This type is not user constructable.
#[subdef::subdef(derive(Debug))]
#[allow(clippy::type_complexity)]
pub struct ResultIter<I: Iterator, E>(
    [Rc<RefCell<ResultIterState<Either<I, iter::Empty<I::Item>>, E>>>; {
        struct ResultIterState<Iter, E> {
            iter: Iter,
            error: Option<E>,
        }
    }],
);

impl<I, E, T> ResultIter<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    const EMPTY_STATE: ResultIterState<Either<I, iter::Empty<I::Item>>, E> =
        ResultIterState { iter: Either::Right(iter::empty()), error: None };

    /// Creates a new `ExtractErr` wrapping the given iterator.
    fn new(iter: I) -> Self {
        ResultIterState { iter: Either::Left(iter), error: None }.pipe(RefCell::new).pipe(Rc::new).pipe(ResultIter)
    }

    /// Takes and returns the inner state, replacing it with an empty state.
    fn take_inner(&self) -> ResultIterState<Either<I, iter::Empty<I::Item>>, E> {
        std::mem::replace(&mut *self.0.borrow_mut(), Self::EMPTY_STATE)
    }

    /// Creates a new handle to the same shared iterator state.
    /// Both handles will share the same underlying state.
    fn share(&self) -> Self {
        self.0.clone().pipe(ResultIter)
    }
}

/// Implements Iterator for `ExtractErr`, yielding Ok values and capturing the first Err.
///
/// Once an Err is encountered, the iterator terminates and stores the error along with the remaining iterator.
impl<I, T, E> Iterator for ResultIter<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.0.borrow_mut();
        state.error.is_some().then_none()?;
        match state.iter.next() {
            None => None,
            Some(Ok(v)) => Some(v),
            Some(Err(e)) => {
                state.error = Some(e);
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        const NO_MORE_ITEMS: (usize, Option<usize>) = (0, Some(0));

        match &*self.0.borrow() {
            ResultIterState { error: Some(_), .. } => NO_MORE_ITEMS,
            ResultIterState { error: None, iter } => (0, iter.size_hint().1),
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
impl<I, T, E, C> TryFromIterator<I> for Result<C, C::Error>
where
    I: IntoIterator<Item = Result<T, E>>,
    C: TryFromIterator<ResultIter<I::IntoIter, E>>,
{
    type Error = ResultCollectionError<E, C, C::Error, Either<I::IntoIter, iter::Empty<Result<T, E>>>>;

    /// Converts an [`IntoIterator`] of [`Result<T, E>`] into a `Result<C, C::Error>`.
    ///
    /// # Return Value
    ///
    /// There are three possible states this function can return:
    ///
    /// - `Ok(Ok(C))`: The [`IntoIterator`] completed successfully and the container (`C`) was successfully constructed.
    /// - `Ok(Err(C::Error))`: The [`IntoIterator`] completed successfully,
    ///   but the container (`C`) construction failed with `C::Error`.
    /// - `Err(ResultCollectionError)`: The [`IntoIterator`] encountered an [`Err<E>`] before completion,
    ///   producing a [`ResultCollectionError`].
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
    /// ```
    ///
    /// ## Error Handling
    ///
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
    /// In the case of an error during iteration, which produces a [`ResultCollectionError`], the iterator
    /// and the collection result can be recovered. If the container type and error type are both [`IntoIterator`]
    /// (which all implementations from this crate are), you can use the [`IntoIterator`] implementation of
    /// [`ResultCollectionError`] to recover the data consumed by the iterator in either case.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", error_recovery_example, [])]
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
    /// ```
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let extractor = ResultIter::new(into_iter.into_iter());

        let try_from_result = extractor.share().pipe(C::try_from_iter);

        match (extractor.take_inner(), try_from_result) {
            (ResultIterState { error: None, .. }, Ok(v)) => Ok(Ok(v)), // iter without err, and successful collect
            (ResultIterState { error: None, .. }, Err(e)) => Ok(Err(e)), // iter without err, but collect failed
            // errored during iter, collect may have succeeded or failed.
            (ResultIterState { error: Some(error), iter }, result) => Err(ResultCollectionError::new(error, result, iter)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [Result<i32, i32>; 4] = [Ok(1), Err(3), Ok(3), Ok(4)];

    #[test]
    fn extract_err_zero_size_after_err() {
        let mut extractor = ResultIter::new(TEST_DATA.into_iter());

        extractor.next(); // Ok(1)
        extractor.next(); // Err(3) - stops here
        extractor.next(); // Should return None

        assert_eq!(extractor.size_hint(), (0, Some(0)));
    }

    #[test]
    fn extract_err_forward_hint() {
        let mut extractor = ResultIter::new(TEST_DATA.into_iter());

        extractor.next(); // Ok(1)

        assert_eq!(extractor.size_hint(), (0, Some(3)));
    }
}
