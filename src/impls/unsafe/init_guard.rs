use std::mem::MaybeUninit;

use fluent_result::bool::Then;

use crate::{ItemCountMismatch, TryExtend};

/// A guard that ensures that all elements in a slice are initialized
pub struct InitGuard<'a, T> {
    slice: &'a mut [MaybeUninit<T>],
    initialized: usize,
}

impl<'a, T> InitGuard<'a, T> {
    /// Creates a new guard for the given slice
    ///
    /// # Safety
    ///
    /// Assumes that all elements in the slice are unitialized
    pub fn new(slice: &'a mut [MaybeUninit<T>]) -> Self {
        Self { slice, initialized: 0 }
    }

    /// tries to disarm the guard, returning an error if the slice is not fully initialized.
    pub fn try_disarm(self) -> Result<(), ItemCountMismatch> {
        match (self.initialized, self.slice.len()) {
            (initialized, len) if initialized < len => Err(ItemCountMismatch::new(len, initialized)),
            _ => {
                std::mem::forget(self);
                Ok(())
            }
        }
    }
}

impl<T> TryExtend<T> for InitGuard<'_, T> {
    type Error = ItemCountMismatch;

    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let initial = self.initialized;

        let mut iter = iter.into_iter().fuse();
        std::iter::zip(&mut self.slice[self.initialized..], iter.by_ref()).for_each(|(slot, value)| {
            slot.write(value);
            self.initialized += 1;
        });

        iter.next().map_or(Ok(()), |_| {
            self.initialized = initial;
            Err(ItemCountMismatch::new(self.slice.len(), self.slice.len() + 1))
        })
    }

    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter().fuse();
        std::iter::zip(&mut self.slice[self.initialized..], iter.by_ref()).for_each(|(slot, value)| {
            slot.write(value);
            self.initialized += 1;
        });
        iter.next().is_some().then_err(ItemCountMismatch::new(self.slice.len(), self.initialized + 1))
    }
}

impl<T> Drop for InitGuard<'_, T> {
    fn drop(&mut self) {
        // SAFETY: elements up to `initialized` are initialized
        self.slice[..self.initialized].iter_mut().for_each(|elem| unsafe { elem.assume_init_drop() });
    }
}
