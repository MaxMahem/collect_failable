use alloc::vec::Vec;
use core::mem::MaybeUninit;
use tap::TryConv;

use fluent_result::into::IntoResult;

use crate::errors::{CapacityError, CollectionError};
use crate::{MaxSize, RemainingSize, SizeHint, TryFromIterator};

impl<const N: usize, T> RemainingSize for [T; N] {
    /// Always returns `SizeHint::ZERO`, since arrays are fixed-size.
    fn remaining_size(&self) -> SizeHint {
        SizeHint::ZERO
    }
}

impl<const N: usize, T> MaxSize for [T; N] {
    /// Always returns `SizeHint::exact(N)`, since arrays are fixed-size.
    const MAX_SIZE: SizeHint = SizeHint::exact(N);
}

/// Create an array of size `N` from an iterator, failing if the iterator produces fewer or more items than `N`.
impl<const N: usize, T, I> TryFromIterator<I> for [T; N]
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, Vec<T>, CapacityError<T>>;

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
    #[inline]
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut array = [const { MaybeUninit::uninit() }; N];
        try_from_iterator_erased(into_iter.into_iter(), &mut array)
            // SAFETY: all elements are initialized on success
            .map(|()| unsafe { core::mem::transmute_copy(&array) }) // TODO: Use array_assume_init once stable
    }
}

/// Internal implementation of [`TryFromIterator`] for arrays of any size. Implemented via this
/// helper to avoid monomorphization for every different array size.
///
/// Assumes that all elements in the slice are unitialized
fn try_from_iterator_erased<T, I: Iterator<Item = T>>(
    mut iter: I,
    slice: &mut [MaybeUninit<T>],
) -> Result<(), CollectionError<I, Vec<T>, CapacityError<T>>> {
    match (SizeHint::exact(slice.len()), iter.size_hint().try_conv::<SizeHint>().expect("invalid size hint")) {
        (capacity, hint) if hint.disjoint(capacity) => CollectionError::bounds(iter, capacity).into_err(),
        (capacity, _) => {
            let mut guard = super::SliceGuard::new(slice);
            guard.extend(iter.by_ref());
            match iter.next() {
                Some(reject) => CollectionError::overflow(iter, guard.drain(), reject, capacity).into_err(),
                None => guard.disarm().map_err(|items| CollectionError::<_, Vec<T>, _>::underflow(iter, items, capacity)),
            }
        }
    }
}
