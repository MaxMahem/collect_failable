use arrayvec::ArrayVec;
use fluent_result::into::IntoResult;

use crate::{CapacityMismatch, CollectionError, TryExtend, TryExtendSafe, TryFromIterator};

/// Tries to create an [`ArrayVec`] from an iterator.
///
/// This implementation will return an error if the iterator produces more items than the [`ArrayVec`]'s capacity.
///
/// # Examples
///
/// ```rust
#[doc = include_doc::function_body!("tests/doc/arrayvec.rs", try_from_iter_arrayvec_example, [])]
/// ```
impl<T, I, const N: usize> TryFromIterator<I> for ArrayVec<T, N>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, Self, CapacityMismatch>;

    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();

        match iter.size_hint() {
            (min, _) if min > N => CollectionError::bounds(iter, 0..=N).into_err(),
            (_, Some(max)) if max <= N => iter.collect::<Self>().into_ok(),
            _ => iter
                .try_fold(Self::new(), |mut array, item| match array.try_push(item) {
                    Ok(()) => Ok(array),
                    Err(capacity_err) => Err((array, capacity_err.element())),
                })
                .map_err(|(array, reject)| CollectionError::overflow(iter, array, reject, 0..=N)),
        }
    }
}

/// Extends an [`ArrayVec`] with an iterator, failing if the iterator produces more items than the [`ArrayVec`]'s
/// remaining capacity.
impl<T, const N: usize, I> TryExtend<I> for ArrayVec<T, N>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, Vec<T>, CapacityMismatch>;

    /// Appends an iterator to the [`ArrayVec`], failing if the iterator produces more items than the [`ArrayVec`]'s
    /// remaining capacity.
    ///
    /// This method provides a basic error guarantee. If the method returns an error, the [`ArrayVec`] is valid, but may
    /// be modified.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/arrayvec.rs", try_extend_arrayvec_example, [])]
    /// ```
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        match (iter.size_hint(), self.remaining_capacity()) {
            ((min, _), remain) if min > remain => CollectionError::bounds(iter, 0..=remain).into_err(),
            (_, remaining) => iter
                .try_for_each(|item| self.try_push(item))
                .map_err(|err| CollectionError::overflow(iter, Vec::new(), err.element(), 0..=remaining)),
        }
    }
}

/// Extends an [`ArrayVec`] with strong error guarantee.
impl<T, const N: usize, I> TryExtendSafe<I> for ArrayVec<T, N>
where
    I: IntoIterator<Item = T>,
{
    /// Appends an iterator to the [`ArrayVec`], failing if the iterator produces more items than the [`ArrayVec`]'s
    /// remaining capacity.
    ///
    /// This method provides a strong error guarantee. If the method returns an error, the [`ArrayVec`] is not modified.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/arrayvec.rs", try_extend_safe_arrayvec_example, [])]
    /// ```
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        match (iter.size_hint(), self.remaining_capacity(), self.len()) {
            ((min, _), remain, _) if min > remain => CollectionError::bounds(iter, 0..=remain).into_err(),
            (_, remaining, len) => iter
                .try_for_each(|item| self.try_push(item))
                .map_err(|err| CollectionError::overflow(iter, self.drain(len..).collect(), err.element(), 0..=remaining)),
        }
    }
}
