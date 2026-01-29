use arrayvec::ArrayVec;

use tap::Pipe;

use crate::TryExtendOne;
use crate::errors::capacity::{CapacityError, FixedCap, RemainingCap};
use crate::errors::types::SizeHint;
use crate::errors::{CollectError, ExtendError};
use crate::utils::{EnsureEmpty, NotEmpty};
use crate::{TryExtend, TryExtendSafe, TryFromIterator};

impl<T, const N: usize> RemainingCap for ArrayVec<T, N> {
    fn remaining_cap(&self) -> SizeHint {
        self.remaining_capacity().pipe(SizeHint::at_most)
    }
}

impl<T, const N: usize> FixedCap for ArrayVec<T, N> {
    const CAP: SizeHint = SizeHint::at_most(N);
}

/// Tries to create an [`ArrayVec`] from an [`IntoIterator`].
///
/// # Errors
///
/// This implementation will return a [`CollectError`] if the iterator produces more items than
/// the [`ArrayVec`]'s capacity.
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
    type Error = CollectError<I::IntoIter, Self, CapacityError<T>>;

    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let iter = into_iter.into_iter();

        CollectError::ensure_fits_in(iter).and_then(|mut iter| {
            let array_vec = iter.by_ref().take(N).collect::<Self>();

            match iter.ensure_empty() {
                Ok(()) => Ok(array_vec),
                Err(NotEmpty { iter, item }) => Err(CollectError::collect_overflow(iter, array_vec, item)),
            }
        })
    }
}

/// Extends an [`ArrayVec`] with an iterator, failing if the iterator produces
/// more items than the [`ArrayVec`]'s remaining capacity.
impl<T, const N: usize, I> TryExtend<I> for ArrayVec<T, N>
where
    I: IntoIterator<Item = T>,
{
    type Error = ExtendError<I::IntoIter, CapacityError<T>>;

    /// Appends `iter` to the [`ArrayVec`], failing if `iter` produces more items than
    /// [`ArrayVec::remaining_capacity`].
    ///
    /// # Errors
    ///
    /// Returns an [`ExtendError`] if `iter` produces more items than [`ArrayVec::remaining_capacity`]. The [`CapacityError::capacity`] in that error will reflect
    /// the capacity of the [`ArrayVec`] after any mutations. This method provides a **basic error
    /// guarantee**. If the method returns an error, the `ArrayVec` is valid, but may be modified.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [`size_hint`](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use arrayvec::ArrayVec;
    /// # use collect_failable::{TryCollectEx, TryExtend};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut array: ArrayVec<i32, 4> = (1..=2).try_collect_ex()?;
    /// array.try_extend([3])?;
    /// assert_eq!(*array, [1, 2, 3], "array should contain 3 items");
    ///
    /// let err = array.try_extend([4, 5]).expect_err("should fail with too many items");
    /// let all_items = array.into_iter().chain(err).collect::<Vec<_>>();
    /// assert_eq!(all_items, [1, 2, 3, 4, 5], "no items should be lost");
    /// # Ok(())
    /// # }
    /// ```
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let iter = iter.into_iter();

        ExtendError::ensure_fits_into(iter, self)
            .and_then(|mut iter| iter.try_for_each(|item| self.try_extend_one(item)).map_err(|err| ExtendError::new(iter, err)))
    }
}

/// Extends an [`ArrayVec`] with strong error guarantee.
impl<T, const N: usize, I> TryExtendSafe<I> for ArrayVec<T, N>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectError<I::IntoIter, Self, CapacityError<T>>;
    /// Appends `iter` to the [`ArrayVec`], failing if `iter` produces more items than
    /// [`ArrayVec::remaining_capacity`].
    ///
    /// # Errors
    ///
    /// Returns a [`CollectError`] if `iter` produces more items than [`ArrayVec::remaining_capacity`].
    /// This method provides a **strong error guarantee**. In the case of an error, the [`ArrayVec`]
    /// is not modified.
    ///
    /// # Panics
    ///
    /// Panics if the `iter`'s [`size_hint`](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use arrayvec::ArrayVec;
    /// # use collect_failable::{TryCollectEx, TryExtendSafe};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let iter = iter.into_iter();

        CollectError::ensure_fits_into(iter, self).and_then(|mut iter| {
            let len = self.len();
            let cap = self.remaining_cap();

            iter.try_for_each(|item| self.try_push(item))
                .map_err(|err| CollectError::overflow(iter, self.drain(len..).collect(), err.element(), cap))
        })
    }
}

impl<T, const N: usize> crate::TryExtendOne for ArrayVec<T, N> {
    type Item = T;
    type Error = CapacityError<T>;

    /// Forwards directly to [`ArrayVec::try_push`].
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityError`] if the [`ArrayVec`] is full.
    fn try_extend_one(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        self.try_push(item).map_err(Into::into)
    }
}
