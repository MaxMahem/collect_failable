use std::mem::MaybeUninit;

use fluent_result::ThenErr;
use size_guess::SizeGuess;

use crate::{ItemCountMismatch, TryFromIterator};

/// Create an array of size `N` from an iterator, failing if the iterator produces fewer or more items than `N`.
impl<const N: usize, T> TryFromIterator<T> for [T; N] {
    type Error = ItemCountMismatch;

    /// Create an array from an iterator, failing if the iterator produces fewer or more items than `N`.
    ///
    /// # Examples
    ///
    /// ```
    /// use collect_failable::{TryCollectEx, ItemCountMismatch};
    ///
    /// // while `TryFromIterator` can be used directly, typically `TryCollectEx` is preferred
    /// let data = 1..=3;
    /// let array: [_; 3] = data.into_iter().try_collect_ex().expect("should be ok");
    /// assert_eq!(array, [1, 2, 3]);
    ///
    /// // an iterator with too many or too few items will fail.
    /// let data = 1..=2; // too few
    /// let err = data.into_iter().try_collect_ex::<[u32; 3]>().expect_err("should be err");
    /// assert_eq!(err, ItemCountMismatch { expected: 3, actual: 2 });
    ///
    /// let data = 1..=4; // too many
    /// let err = data.into_iter().try_collect_ex::<[u32; 3]>().expect_err("should be err");
    /// assert_eq!(err, ItemCountMismatch { expected: 3, actual: 4 });
    /// ```
    fn try_from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Result<Self, Self::Error> {
        let into_iter = into_iter.into_iter();

        // size guess is treated as a reliable lower bound. It might have more elements than the guess, but it should
        // not have fewer elements than the guess.
        let size_guess = into_iter.size_guess();
        (size_guess > N).then_err(ItemCountMismatch::new(N, size_guess))?;

        let mut array = [const { MaybeUninit::uninit() }; N];
        match try_from_iterator_erased(into_iter.into_iter(), &mut array, N) {
            Err(ItemCountMismatch { expected, actual }) => {
                // SAFETY: elements up to `actual` are initialized
                array.iter_mut().take(actual).for_each(|element| unsafe { element.assume_init_drop() });
                Err(ItemCountMismatch::new(expected, actual))
            }
            // SAFETY: all elements are initialized
            Ok(()) => Ok(unsafe { std::mem::transmute_copy(&array) }), // TODO: Use array_assume_init once stable
        }
    }
}

/// Internal implementation of [`TryFromIterator`] for arrays of any size. Implemented via this
/// helper to avoid monomorphization for every different array size.
fn try_from_iterator_erased<T, I: Iterator<Item = T>>(
    mut iter: I,
    array: &mut [MaybeUninit<T>],
    expected: usize,
) -> Result<(), ItemCountMismatch> {
    array.iter_mut().enumerate().try_for_each(|(index, element)| {
        iter.next().ok_or(ItemCountMismatch::new(expected, index)).map(|value| _ = element.write(value))
    })?;
    iter.next().map_or(Ok(()), |_| Err(ItemCountMismatch::new(expected, expected + 1)))
}
