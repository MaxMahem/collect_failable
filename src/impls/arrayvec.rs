use alloc::vec::Vec;

use arrayvec::{ArrayVec, CapacityError};
use fluent_result::into::IntoResult;
use size_hinter::SizeHint;

use crate::errors::{CapacityMismatch, CollectionError};
use crate::{TryExtend, TryExtendSafe, TryFromIterator};

/// Tries to create an [`ArrayVec`] from an iterator.
///
/// This implementation will return an error if the iterator produces more items than the [`ArrayVec`]'s capacity.
///
/// # Examples
///
/// ```rust
/// # use arrayvec::ArrayVec;
/// # use collect_failable::TryFromIterator;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let array: ArrayVec<i32, 4> = ArrayVec::try_from_iter(1..=3)?;
/// assert_eq!(array.as_slice(), &[1, 2, 3], "array should contain all items");
///
/// let err = ArrayVec::<i32, 3>::try_from_iter(1..=4).expect_err("should fail with too many items");
/// assert_eq!(err.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4], "error should contain all items");
/// # Ok(())
/// # }
/// ```
impl<T, I, const N: usize> TryFromIterator<I> for ArrayVec<T, N>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, Self, CapacityMismatch>;

    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();

        match iter.size_hint() {
            (min, _) if min > N => CollectionError::bounds(iter, SizeHint::at_most(N)).into_err(),
            (_, Some(max)) if max <= N => iter.collect::<Self>().into_ok(),
            _ => iter
                .try_fold(Self::new(), |mut array, item| match array.try_push(item) {
                    Ok(()) => Ok(array),
                    Err(capacity_err) => Err((array, capacity_err.element())),
                })
                .map_err(|(array, reject)| CollectionError::overflow(iter, array, reject, SizeHint::at_most(N))),
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
    /// # use arrayvec::ArrayVec;
    /// use collect_failable::{TryCollectEx, TryExtend};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let mut array: ArrayVec<i32, 4> = (1..=2).try_collect_ex()?;
    /// array.try_extend([3])?;
    /// assert_eq!(*array, [1, 2, 3], "array should contain 3 items");
    ///
    /// let err = array.try_extend([4, 5]).expect_err("should fail with too many items");
    /// // `array` may be modified, err should contain all items not inserted
    /// let all_items = array.into_iter().chain(err).collect::<Vec<_>>();
    /// assert_eq!(all_items, [1, 2, 3, 4, 5], "no items should be lost");
    /// # Ok(())
    /// # }
    /// ```
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        match (iter.size_hint(), self.remaining_capacity()) {
            ((min, _), remain) if min > remain => CollectionError::bounds(iter, SizeHint::at_most(remain)).into_err(),
            (_, remain) => iter
                .try_for_each(|item| self.try_push(item))
                .map_err(|err| CollectionError::overflow(iter, Vec::new(), err.element(), SizeHint::at_most(remain))),
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
    /// # use arrayvec::ArrayVec;
    /// # use collect_failable::{TryCollectEx, TryExtendSafe};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let mut array: ArrayVec<i32, 4> = (1..=2).try_collect_ex()?;
    /// array.try_extend_safe([3])?;
    /// assert_eq!(*array, [1, 2, 3], "array should contain 3 items");
    ///
    /// let err = array.try_extend_safe([4, 5]).expect_err("should fail with too many items");
    /// assert_eq!(*array, [1, 2, 3], "array should be unchanged on error");
    ///
    /// let collected: Vec<i32> = err.into_iter().collect();
    /// assert_eq!(collected, [4, 5], "error should contain all unconsumed items");
    /// # Ok(())
    /// # }
    /// ```
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        match (iter.size_hint(), self.remaining_capacity(), self.len()) {
            ((min, _), remain, _) if min > remain => CollectionError::bounds(iter, SizeHint::at_most(remain)).into_err(),
            (_, remain, len) => iter.try_for_each(|item| self.try_push(item)).map_err(|err| {
                CollectionError::overflow(iter, self.drain(len..).collect(), err.element(), SizeHint::at_most(remain))
            }),
        }
    }
}

impl<T, const N: usize> crate::TryExtendOne for ArrayVec<T, N> {
    type Item = T;
    type Error = CapacityError<T>;

    /// Forwards directly to [`ArrayVec::try_push`].
    fn try_extend_one(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        self.try_push(item)
    }
}
