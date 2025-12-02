use std::mem::MaybeUninit;

use crate::{ItemCountMismatch, TryExtend, TryFromIterator};

/// Create an array of size `N` from an iterator, failing if the iterator produces fewer or more items than `N`.
impl<const N: usize, T> TryFromIterator<T> for [T; N] {
    type Error = ItemCountMismatch;

    /// Create an array from an iterator, failing if the iterator produces fewer or more items than `N`.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/array.rs", try_from_iter_array_example, [])]
    /// ```
    fn try_from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Result<Self, Self::Error> {
        let mut array = [const { MaybeUninit::uninit() }; N];
        try_from_iterator_erased(into_iter.into_iter(), &mut array)?;
        // SAFETY: all elements are initialized
        Ok(unsafe { std::mem::transmute_copy(&array) }) // TODO: Use array_assume_init once stable
    }
}

/// Internal implementation of [`TryFromIterator`] for arrays of any size. Implemented via this
/// helper to avoid monomorphization for every different array size.
///
/// # Safety
///
/// Assumes that all elements in the slice are unitialized
fn try_from_iterator_erased<T, I: Iterator<Item = T>>(
    mut iter: I,
    array: &mut [MaybeUninit<T>],
) -> Result<(), ItemCountMismatch> {
    match (array.len(), iter.size_hint()) {
        (len, (min, _)) if min > len => Err(ItemCountMismatch::new(len, min)),
        (len, (_, Some(max))) if max < len => Err(ItemCountMismatch::new(len, max)),
        _ => {
            let mut guard = super::InitGuard::new(array);
            guard.try_extend(&mut iter)?;
            guard.try_disarm()
        }
    }
}
