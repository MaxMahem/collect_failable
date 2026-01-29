use core::iter::FusedIterator;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::Deref;

use fluent_result::into::IntoResult;
use tap::Pipe;

use crate::errors::capacity::{CapacityError, FixedCap, RemainingCap};
use crate::errors::types::SizeHint;

/// A possibly partially initialized array that results from the failed collection of an array.
#[derive(Debug)]
pub struct PartialArray<T, const N: usize> {
    /// The array elements. Elements `array[..back]` are initialized,
    /// Elements `array[back..]` are uninitialized
    array: [MaybeUninit<T>; N],
    /// Index of the first uninitialized element/length of the initialized array
    back: usize,
}

impl<T, const N: usize> PartialArray<T, N> {
    /// Creates a new guard for the given slice.
    ///
    /// # Note
    ///
    /// This is generally not needed for normal usage, but can be useful for testing [`CollectError`](crate::errors::CollectError)s.
    #[must_use]
    #[doc(hidden)]
    pub const fn new() -> Self {
        Self { array: [const { MaybeUninit::uninit() }; N], back: 0 }
    }

    /// Pushes an item into the partial array, returning it if the array is full.
    ///
    /// # Errors
    ///
    /// Returns the item if the array is full.
    #[doc(hidden)]
    pub const fn try_push(&mut self, item: T) -> Result<(), T> {
        match self.back >= N {
            true => Err(item),
            false => {
                self.array[self.back].write(item);
                self.back += 1;
                Ok(())
            }
        }
    }
}

#[doc(hidden)]
impl<T, const N: usize> Default for PartialArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> PartialArray<T, N> {
    fn initialized(&self) -> &[MaybeUninit<T>] {
        &self.array[..self.back]
    }

    fn initialized_mut(&mut self) -> &mut [MaybeUninit<T>] {
        &mut self.array[..self.back]
    }
}

impl<T, const N: usize> Deref for PartialArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // SAFETY: initialized elements are initialized
        self.initialized().pipe_ref(|slice| unsafe { slice.assume_init_ref() })
    }
}

impl<T, const N: usize> RemainingCap for PartialArray<T, N> {
    fn remaining_cap(&self) -> SizeHint {
        SizeHint::at_most(N - self.back)
    }
}

impl<T, const N: usize> FixedCap for PartialArray<T, N> {
    const CAP: SizeHint = SizeHint::exact(N);
}

impl<T, U, const N: usize> PartialEq<[U]> for PartialArray<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        self[..] == other[..]
    }
}

impl<T, const N: usize> Drop for PartialArray<T, N> {
    fn drop(&mut self) {
        // SAFETY: initialized elements are initialized
        self.initialized_mut().pipe_ref_mut(|slice| unsafe { slice.assume_init_drop() });
    }
}

/// Error returned when trying to convert a [`PartialArray`] into a full array.
#[derive(Debug, thiserror::Error)]
#[error("Not enough elements to fill the array: {error}")]
pub struct IntoArrayError<T, const N: usize> {
    /// The [`PartialArray`] that failed to convert.
    pub partial_array: PartialArray<T, N>,
    /// The error showing why the array was not full.
    #[source]
    pub error: CapacityError<T>,
}

impl<T, const N: usize> IntoArrayError<T, N> {
    const fn new(partial_array: PartialArray<T, N>) -> Self {
        let error = CapacityError::collect_underflow::<[T; N]>(partial_array.back);
        Self { partial_array, error }
    }
}

/// Tries to convert the [`PartialArray`] into a full array `[T; N]`.
///
/// This is only possible in an overflow error case, where the iterator was too long.
impl<T, const N: usize> TryFrom<PartialArray<T, N>> for [T; N] {
    type Error = IntoArrayError<T, N>;

    /// Performs the conversion.
    ///
    /// # Errors
    ///
    /// Returns an [`IntoArrayError`] if the array is not fully initialized.
    fn try_from(partial_array: PartialArray<T, N>) -> Result<Self, Self::Error> {
        match partial_array.back == N {
            false => IntoArrayError::new(partial_array).into_err(),
            true => ManuallyDrop::new(partial_array)
                .array
                .as_ptr()
                .cast::<[T; N]>()
                // SAFETY: all elements are initialized, read exactly once and never again
                .pipe(|ptr| unsafe { core::ptr::read(ptr) })
                .into_ok(),
        }
    }
}

impl<T, const N: usize> IntoIterator for PartialArray<T, N> {
    type Item = T;
    type IntoIter = Drain<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Drain { partial_array: ManuallyDrop::new(self), next: 0 }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a PartialArray<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self[..].iter()
    }
}

/// An iterator that moves out of a [`PartialArray`].
#[derive(Debug)]
pub struct Drain<T, const N: usize> {
    partial_array: ManuallyDrop<PartialArray<T, N>>,
    next: usize,
}

impl<T, const N: usize> Iterator for Drain<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.partial_array.get(self.next).map(|item| {
            self.next += 1;
            // SAFETY: item is read once and never again
            unsafe { core::ptr::read(item) }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        SizeHint::exact(self.partial_array.back - self.next).into()
    }
}

impl<T, const N: usize> ExactSizeIterator for Drain<T, N> {}
impl<T, const N: usize> FusedIterator for Drain<T, N> {}

impl<T, const N: usize> Drop for Drain<T, N> {
    fn drop(&mut self) {
        let back = self.partial_array.back;

        self.partial_array.array[self.next..back]
            // SAFETY: elements between `next` and `back` are owned and initialized
            .pipe_ref_mut(|slice| unsafe { slice.assume_init_drop() });
    }
}
