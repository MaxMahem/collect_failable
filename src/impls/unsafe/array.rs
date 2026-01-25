use crate::errors::{CapacityError, CollectionError};
use crate::{FixedCap, RemainingCap, SizeHint, TryFromIterator};

use super::PartialArray;

impl<const N: usize, T> RemainingCap for [T; N] {
    /// Always returns [`SizeHint::ZERO`], since arrays are fixed-size.
    fn remaining_cap(&self) -> SizeHint {
        SizeHint::ZERO
    }
}

impl<const N: usize, T> FixedCap for [T; N] {
    const CAP: SizeHint = SizeHint::exact(N);
}

/// Create an array of size `N` from an iterator, failing if the iterator produces fewer or more items than `N`.
impl<const N: usize, T, I> TryFromIterator<I> for [T; N]
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, PartialArray<T, N>, CapacityError<T>>;

    /// Create an array from an [`IntoIterator`], failing if the [`IntoIterator::IntoIter`]
    /// produces fewer or more items than `N`.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError`] if the [`IntoIterator::IntoIter`] produces more or fewer items
    /// than `N`. All items from the iterator are preserved in the error, and can be retrieved using
    /// [`CollectionError::into_iter`].
    ///
    /// # Panics
    ///
    /// This method panics if the iterator's [`size_hint`](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::CapacityError;
    /// use collect_failable::TryFromIterator;
    ///
    /// let array = <[_; 3]>::try_from_iter(1..=3).expect("should succeed");
    /// assert_eq!(array, [1, 2, 3]);
    ///
    /// let too_few_err = <[u32; 3]>::try_from_iter(1..=2).expect_err("should fail, too few items");
    /// assert_eq!(too_few_err.into_iter().collect::<Vec<_>>(), vec![1, 2], "err should contain all items");
    ///
    /// let too_many_err = <[u32; 3]>::try_from_iter(1..=4).expect_err("should fail, too many items");
    /// assert_eq!(too_many_err.into_iter().collect::<Vec<_>>(), vec![1, 2, 3, 4], "err should contain all items");
    /// ```
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut into_iter = into_iter.into_iter();

        let mut guard = PartialArray::new();
        match guard.try_extend_basic(&mut into_iter) {
            Ok(()) => guard.try_into_array().map_err(|(g, e)| CollectionError::new(into_iter, g, e)),
            Err(error) => Err(CollectionError::new(into_iter, guard, error)),
        }
    }
}
