use std::mem::MaybeUninit;

use fluent_result::into::IntoResult;

use crate::{impls::r#unsafe::DisarmError, CapacityMismatch, CollectionError, TryFromIterator};

/// Create an array of size `N` from an iterator, failing if the iterator produces fewer or more items than `N`.
impl<const N: usize, T, I> TryFromIterator<T, I> for [T; N]
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<T, I::IntoIter, Vec<T>, CapacityMismatch>;

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
) -> Result<(), CollectionError<T, I, Vec<T>, CapacityMismatch>> {
    match (array.len(), iter.size_hint()) {
        (len, hint @ (min, _)) if min > len => bounds_error(iter, len, hint),
        (len, hint @ (_, Some(max))) if max < len => bounds_error(iter, len, hint),
        (len, _) => {
            let mut guard = super::SliceGuard::new(array);
            guard.extend(iter.by_ref());
            match iter.next() {
                Some(reject) => overflow_error(len, guard.drain(), reject, iter),
                None => guard.disarm().map_err(|DisarmError { error, items }| CollectionError::new(iter, items, None, error)),
            }
        }
    }
}

fn bounds_error<T, I: Iterator<Item = T>>(
    iter: I,
    len: usize,
    hint: (usize, Option<usize>),
) -> Result<(), CollectionError<T, I, Vec<T>, CapacityMismatch>> {
    CollectionError::new(iter, Vec::new(), None, CapacityMismatch::bounds(len..=len, hint)).into_err()
}

fn overflow_error<T, I: Iterator<Item = T>>(
    len: usize,
    items: Vec<T>,
    reject: T,
    iter: I,
) -> Result<(), CollectionError<T, I, Vec<T>, CapacityMismatch>> {
    CollectionError::new(iter, items, Some(reject), CapacityMismatch::overflow(len..=len)).into_err()
}
