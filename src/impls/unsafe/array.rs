use std::mem::MaybeUninit;

use fluent_result::into::IntoResult;

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
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/array.rs", try_from_iter_array_example, [])]
    /// ```
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
        (len, hint @ (min, _)) if min > len => CollectionError::bounds(iter, len..=len).into_err(),
        (len, hint @ (_, Some(max))) if max < len => CollectionError::bounds(iter, len..=len).into_err(),
        (len, _) => {
            let mut guard = super::SliceGuard::new(array);
            guard.extend(iter.by_ref());
            match iter.next() {
                Some(reject) => CollectionError::overflow(iter, guard.drain(), reject, len..=len).into_err(),
                None => guard.disarm().map_err(|DisarmError { error, items }| CollectionError::new(iter, items, None, error)),
            }
        }
    }
}
