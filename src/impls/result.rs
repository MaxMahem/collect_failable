use alloc::rc::Rc;
use core::cell::RefCell;

use size_hinter::SizeHint;
use tap::Pipe;

use crate::errors::ResultCollectionError;
use crate::TryFromIterator;

/// Iterator adaptor that extracts [`Ok`] values from a [`Result`] [`Iterator`],
/// storing the first encountered [`Err`] and remaining iterator for later retrieval.
///
/// This type is not user constructable.
#[subdef::subdef(derive(Debug))]
pub struct ResultIter<I: Iterator, E>(
    [Rc<RefCell<IterState<I, E>>>; {
        enum IterState<Iter, E> {
            Active(Iter),
            Taken,
            Errored { error: E, remaining: Iter },
        }
    }],
);

impl<I, E, T> ResultIter<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    const EMPTY_STATE: IterState<I, E> = IterState::Taken;

    /// Creates a new `ExtractErr` wrapping the given iterator.
    fn new(iter: I) -> Self {
        IterState::Active(iter).pipe(RefCell::new).pipe(Rc::new).pipe(ResultIter)
    }

    /// Takes and returns the inner state, replacing it with an empty state.
    fn take_inner(&self) -> IterState<I, E> {
        core::mem::replace(&mut *self.0.borrow_mut(), Self::EMPTY_STATE)
    }

    /// Creates a new handle to the same shared iterator state.
    /// Both handles will share the same underlying state.
    fn share(&self) -> Self {
        self.0.clone().pipe(ResultIter)
    }
}

impl<Iter: Iterator, E> IterState<Iter, E> {
    /// Advances the iterator, returning the next value if available, or an error if encountered.
    /// while also updating the state to reflect the result of the advance.
    fn advance<T>(self) -> (Self, Option<T>)
    where
        Iter: Iterator<Item = Result<T, E>>,
    {
        match self {
            Self::Active(mut iter) => match iter.next() {
                Some(Ok(v)) => (Self::Active(iter), Some(v)),
                Some(Err(e)) => (Self::Errored { error: e, remaining: iter }, None),
                None => (Self::Active(iter), None),
            },
            state @ (Self::Errored { .. } | Self::Taken) => (state, None),
        }
    }
}

/// Implements Iterator for `ResultIter`, yielding [`Ok`] values and capturing the first [`Err`].
///
/// Once an [`Err`] is encountered, the iterator terminates and stores the error along with the remaining iterator.
impl<I, T, E> Iterator for ResultIter<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = self.0.borrow_mut();
        let (new_state, item) = core::mem::replace(&mut *state, Self::EMPTY_STATE).advance();
        *state = new_state;
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &*self.0.borrow() {
            IterState::Errored { .. } | IterState::Taken => SizeHint::ZERO.into(),
            IterState::Active(iter) => (0, iter.size_hint().1),
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
    type Error = ResultCollectionError<E, C, C::Error, I::IntoIter>;

    /// Converts an [`IntoIterator`] of [`Result<T, E>`] into a `Result<C, C::Error>`.
    ///
    /// # Return Value
    ///
    /// There are three possible states this function can return:
    ///
    /// - `Ok(Ok(C))`: The [`IntoIterator`] completed successfully and the container (`C`) was
    ///   successfully constructed.
    /// - `Ok(Err(C::Error))`: The [`IntoIterator`] completed successfully, but the container (`C`)
    ///   construction failed with `C::Error`.
    /// - `Err(ResultCollectionError)`: The [`IntoIterator`] encountered an [`Err<E>`] before
    ///   completion, producing a [`ResultCollectionError`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::{TryCollectEx, TryFromIterator};
    /// use std::collections::HashSet;
    ///
    /// let all_ok: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    /// match Result::<HashSet<i32>, _>::try_from_iter(all_ok) {
    ///     Ok(Ok(set)) => assert_eq!(set, HashSet::from([1, 2, 3]), "set should contain all items"),
    ///     Ok(Err(_)) => panic!("should succeed"),
    ///     Err(_) => panic!("should succeed"),
    /// };
    ///
    /// let iter_err: Vec<Result<i32, &str>> = vec![Ok(1), Err("oops"), Ok(3)];
    /// match Result::<HashSet<i32>, _>::try_from_iter(iter_err) {
    ///     Ok(Ok(_)) => panic!("should fail"),
    ///     Ok(Err(_)) => panic!("iterating values should fail"),
    ///     Err(e) => assert_eq!(e.error, "oops", "error should be the first error encountered"),
    /// };
    ///
    /// let collection_err: Vec<Result<i32, &str>> = vec![Ok(1), Ok(1), Ok(3)];
    /// match Result::<HashSet<i32>, _>::try_from_iter(collection_err) {
    ///     Ok(Ok(_)) => panic!("should fail"),
    ///     Ok(Err(e)) => assert_eq!(e.item, 1, "collision should be on value 1"),
    ///     Err(_) => panic!("iterating values should succeed"),
    /// };
    /// ```
    ///
    /// ## Error Handling
    ///
    /// Handling the nested `Result<Result<C, E>, C::Error>` may be cumbersome. Several crates may
    /// offer means of flattening the error type. Alternatively, if both error types are
    /// convertable to the return type of the local scope using [`From`], two instances of the `??`
    /// operator can be used to flatten the error.
    ///
    /// ```rust
    /// use collect_failable::TryCollectEx;
    /// use std::collections::HashSet;
    /// use std::error::Error;
    ///
    /// // Define a simple error type that implements Error
    /// #[derive(Debug, thiserror::Error)]
    /// #[error("parse error")]
    /// struct ParseError;
    ///
    /// // Most errors can be converted to Box<dyn std::error::Error> using From
    /// fn process_data(data: Vec<Result<i32, ParseError>>) -> Result<HashSet<i32>, Box<dyn Error + 'static>> {
    ///     let set = data.into_iter().try_collect_ex::<Result<HashSet<i32>, _>>()??;
    ///     Ok(set)
    /// }
    ///
    /// let success = process_data(vec![Ok(1), Ok(2), Ok(3)]).expect("should succeed");
    /// assert_eq!(success, HashSet::from([1, 2, 3]), "set should contain all items");
    ///
    /// let iter_err = process_data(vec![Ok(1), Err(ParseError), Ok(3)]).expect_err("should fail on parse error");
    /// assert_eq!(iter_err.to_string(), "Iterator error: parse error");
    ///
    /// let collect_err = process_data(vec![Ok(1), Ok(1), Ok(3)]).expect_err("should fail on collision");
    /// assert_eq!(collect_err.to_string(), "Collection collision");
    /// ```
    ///
    /// ## Recovering Data
    ///
    /// In the case of an error during iteration, which produces a [`ResultCollectionError`], which
    /// provides access to:
    /// - [`error`](ResultCollectionError::error): The error that was encountered
    /// - [`result`](ResultCollectionError::result): The partial collection result (`Ok` or `Err`).
    /// - [`iter`](ResultCollectionError::iter): The remaining unconsumed items from the original `Iterator`.
    ///
    /// ```rust
    /// use collect_failable::TryCollectEx;
    /// use std::collections::HashSet;
    ///
    /// // Collect items until an error is encountered
    /// let data = vec![Ok(1), Ok(2), Ok(3), Err("invalid"), Ok(5)];
    /// let result: Result<Result<HashSet<_>, _>, _> = data.into_iter().try_collect_ex();
    /// let err = result.expect_err("should fail on invalid item");
    ///
    /// assert_eq!(err.error, "invalid", "error should be the iteration error");
    /// let collected = err.result.as_ref().expect("partial collection should succeed");
    /// assert_eq!(collected, &HashSet::from([1, 2, 3]), "partial collection should contain first 3 items");
    /// assert_eq!(err.iter.size_hint(), (1, Some(1)), "remaining iterator should have 1 item");
    /// let next = err.into_data().iter.next();
    /// assert_eq!(next, Some(Ok(5)), "remaining iterator should contain the unconsumed item");
    /// ```
    ///
    /// When both the success type (`C`) and error type (`CErr`) implement [`IntoIterator`], which
    /// all implementations from this crate do, you can use the [`either`](either) crate (or
    /// similar) to uniformly recover all partially collected data, regardless of whether the
    /// collection succeeded or failed. In other words it lets you go easily from a
    /// [`ResultCollectionError::result`] value to an [Iterator<Item = T>].
    ///
    /// ```rust
    /// use collect_failable::TryCollectEx;
    /// use collect_failable::errors::ResultCollectionErrorData;
    /// use std::collections::HashSet;
    /// use either::Either;
    /// use tap::Conv;
    ///
    /// let data = vec![Ok(1), Ok(2), Ok(3), Err("invalid"), Ok(5)];
    /// let result: Result<Result<HashSet<_>, _>, _> = data.into_iter().try_collect_ex();
    /// let err = result.expect_err("should fail on invalid item");
    ///
    /// // Use Either to uniformly handle both Ok and Err cases
    /// let collected_items: Vec<_> = err
    ///     .into_data()
    ///     .result
    ///     .conv::<Either<_, _>>()
    ///     .into_iter()
    ///     .collect();
    ///
    /// // Regardless of whether collection succeeded or failed, we get the items
    /// assert_eq!(collected_items.len(), 3);
    /// assert!(collected_items.contains(&1));
    /// assert!(collected_items.contains(&2));
    /// assert!(collected_items.contains(&3));
    /// ```
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let extractor = ResultIter::new(into_iter.into_iter());

        let try_from_result = extractor.share().pipe(C::try_from_iter);

        match (extractor.take_inner(), try_from_result) {
            (IterState::Active(_), Ok(v)) => Ok(Ok(v)),
            (IterState::Active(_), Err(e)) => Ok(Err(e)),
            (IterState::Errored { error, remaining }, result) => Err(ResultCollectionError::new(error, result, remaining)),
            (IterState::Taken, _) => unreachable!("take_inner called multiple times"),
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
