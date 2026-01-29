use fluent_result::into::IntoResult;

use crate::TryFromIterator;
use crate::errors::CollectError;
use crate::errors::capacity::CapacityError;
use crate::errors::capacity::{FixedCap, RemainingCap};
use crate::errors::types::SizeHint;
use crate::impls::r#unsafe::IntoArrayError;

use super::PartialArray;

impl<const N: usize, T> RemainingCap for [T; N] {
    /// Always returns [`SizeHint::ZERO`].
    fn remaining_cap(&self) -> SizeHint {
        SizeHint::ZERO
    }
}

impl<const N: usize, T> FixedCap for [T; N] {
    /// Always returns [`SizeHint::exact(N)`](SizeHint::exact).
    const CAP: SizeHint = SizeHint::exact(N);
}

/// Create an array of size `N` from an iterator,
/// failing if the iterator does not produce exactly `N` items.
impl<const N: usize, T, I> TryFromIterator<I> for [T; N]
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectError<I::IntoIter, PartialArray<T, N>, CapacityError<T>>;

    /// Create an array from `iter`, failing if `iter` does not produce exactly `N` items.
    ///
    /// # Errors
    ///
    /// Returns [`CollectError`] if `iter` does not produce exactly `N` items.
    /// All items from `iter` are preserved in the error, and can be retrieved using
    /// [`CollectError::into_iter`].
    ///
    /// # Panics
    ///
    /// Panics if the `iter`'s [`size_hint`](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::capacity::CapacityError;
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
        let mut iter = into_iter.into_iter();
        let mut partial_array = PartialArray::new();

        match CapacityError::ensure_fits_in::<[T; N], _>(&iter) {
            Err(err) => CollectError::new(iter, partial_array, err).into_err(),
            Ok(()) => match iter.try_for_each(|item| partial_array.try_push(item)) {
                Err(item) => CollectError::collect_overflow(iter, partial_array, item).into_err(),
                Ok(()) => partial_array
                    .try_into()
                    .map_err(|IntoArrayError { partial_array, error }| CollectError::new(iter, partial_array, error)),
            },
        }
    }
}
