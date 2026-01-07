use std::mem::MaybeUninit;

use fluent_result::into::IntoResult;
use size_hinter::SizeHint;

use crate::errors::{CapacityMismatch, CollectionError};
use crate::impls::r#unsafe::DisarmError;
use crate::TryFromIterator;

/// Create an array of size `N` from an iterator, failing if the iterator produces fewer or more items than `N`.
impl<const N: usize, T, I> TryFromIterator<I> for [T; N]
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, Vec<T>, CapacityMismatch>;

    /// Create an array from an iterator.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError`] if the iterator produces more or fewer items than `N`.
    /// All items from the iterator are preserved in the error, and can be retrieved using
    /// [`CollectionError::into_iter`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::CapacityMismatch;
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
            .map(|()| unsafe { std::mem::transmute_copy(&array) }) // TODO: Use array_assume_init once stable
    }
}

/// Internal implementation of [`TryFromIterator`] for arrays of any size. Implemented via this
/// helper to avoid monomorphization for every different array size.
///
/// Assumes that all elements in the slice are unitialized
fn try_from_iterator_erased<T, I: Iterator<Item = T>>(
    mut iter: I,
    array: &mut [MaybeUninit<T>],
) -> Result<(), CollectionError<I, Vec<T>, CapacityMismatch>> {
    match (array.len(), iter.size_hint()) {
        (len, hint @ (min, _)) if min > len => CollectionError::bounds(iter, SizeHint::exact(len)).into_err(),
        (len, hint @ (_, Some(max))) if max < len => CollectionError::bounds(iter, SizeHint::exact(len)).into_err(),
        (len, _) => {
            let mut guard = super::SliceGuard::new(array);
            guard.extend(iter.by_ref());
            match iter.next() {
                Some(reject) => CollectionError::overflow(iter, guard.drain(), reject, SizeHint::exact(len)).into_err(),
                None => guard.disarm().map_err(|DisarmError { error, items }| CollectionError::new(iter, items, None, error)),
            }
        }
    }
}
