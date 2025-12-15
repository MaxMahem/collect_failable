use std::mem::MaybeUninit;

use fluent_result::into::IntoResult;

use crate::CapacityMismatch;

/// A guard that ensures that all elements in a slice are initialized
pub struct SliceGuard<'a, T> {
    slice: &'a mut [MaybeUninit<T>],
    initialized: usize,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, derive_more::Constructor)]
pub struct DisarmError<T> {
    #[source]
    pub error: CapacityMismatch,
    pub items: Vec<T>,
}

impl<'a, T> SliceGuard<'a, T> {
    /// Creates a new guard for the given slice
    ///
    /// Assumes that all elements in `slice` are unitialized. Passing in initialized elements is
    /// safe, but may result in a memory leak, as their destructors may not be called.
    pub const fn new(slice: &'a mut [MaybeUninit<T>]) -> Self {
        Self { slice, initialized: 0 }
    }

    /// Disarms the guard, returning an error if the slice is not fully initialized.
    pub fn disarm(self) -> Result<(), DisarmError<T>> {
        match (self.initialized, self.slice.len()) {
            (init, len) if init != len => DisarmError::new(CapacityMismatch::underflow(len..=len, init), self.drain()).into_err(),
            _ => Ok(() = std::mem::forget(self)),
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
        std::mem::forget(self);
        collection
    }
}

impl<T> Extend<T> for SliceGuard<'_, T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        std::iter::zip(&mut self.slice[self.initialized..], iter).for_each(|(slot, value)| {
            slot.write(value);
            self.initialized += 1;
        });
    }
}

impl<T> Drop for SliceGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: elements up to `initialized` are initialized
        self.slice[..self.initialized].iter_mut().for_each(|elem| unsafe { elem.assume_init_drop() });
    }
}
