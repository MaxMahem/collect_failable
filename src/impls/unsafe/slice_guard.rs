use alloc::vec::Vec;
use core::mem::MaybeUninit;
use size_hinter::SizeHint;

use crate::errors::CapacityError;

/// A guard that ensures that all elements in a slice are initialized
pub struct SliceGuard<'a, T> {
    slice: &'a mut [MaybeUninit<T>],
    initialized: usize,
}

impl<'a, T> SliceGuard<'a, T> {
    /// Creates a new guard for the given slice
    ///
    /// Assumes all elements in `slice` are unitialized. Passing in initialized elements is
    /// safe, but may result in a memory leak, as their destructors may not be called.
    pub const fn new(slice: &'a mut [MaybeUninit<T>]) -> Self {
        Self { slice, initialized: 0 }
    }

    /// Disarms the guard, returning an error if the slice is not fully initialized.
    pub fn disarm(self) -> Result<(), (Vec<T>, CapacityError<T>)> {
        match (self.initialized, self.slice.len()) {
            (init, len) if init != len => Err((self.drain(), CapacityError::underflow(SizeHint::exact(len), init))),
            _ => Ok(() = core::mem::forget(self)),
        }
    }

    /// Drain the initialized items from the slice into a vec
    pub fn drain(self) -> Vec<T> {
        let collection = self.slice[..self.initialized]
            .iter_mut()
            // SAFETY: elements up to `initialized` are initialized
            // The slice is then consumed and so cannot be read through this type again
            .map(|i| unsafe { i.assume_init_read() })
            .collect();
        core::mem::forget(self);
        collection
    }
}

impl<T> Extend<T> for SliceGuard<'_, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        core::iter::zip(&mut self.slice[self.initialized..], iter).for_each(|(slot, value)| {
            slot.write(value);
            self.initialized += 1;
        });
    }
}

impl<T> Drop for SliceGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: elements up to `initialized` are initialized
        unsafe { self.slice[..self.initialized].assume_init_drop() };
    }
}
