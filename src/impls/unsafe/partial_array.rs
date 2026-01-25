use core::iter::FusedIterator;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::Deref;
use size_hinter::SizeHint;
use tap::TryConv;

use crate::FixedCap;
use crate::errors::CapacityError;

/// A guard that ensures that all elements in a slice are initialized
#[derive(Debug)]
pub struct PartialArray<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    /// Index of the first uninitialized element/length of the initialized array
    back: usize,
}

impl<T, const N: usize> PartialArray<T, N> {
    /// Creates a new guard for the given slice
    pub(crate) const fn new() -> Self {
        Self { array: [const { MaybeUninit::uninit() }; N], back: 0 }
    }

    /// Disarms the guard, returning an error if the slice is not fully initialized.
    pub(crate) const fn try_into_array(self) -> Result<[T; N], (Self, CapacityError<T>)> {
        match (self.back, N) {
            (init, len) if init != len => Err((self, CapacityError::underflow(SizeHint::exact(len), init))),
            _ => {
                let array = unsafe { core::ptr::read(self.array.as_ptr().cast::<[T; N]>()) };
                core::mem::forget(self);
                Ok(array)
            }
        }
    }

    pub(crate) fn try_extend_basic(&mut self, iter: &mut impl Iterator<Item = T>) -> Result<(), CapacityError<T>> {
        let hint = iter.size_hint().try_conv::<SizeHint>().expect("invalid size hint");
        let capacity = <[T; N]>::CAP;

        match SizeHint::disjoint(hint, capacity) {
            true => Err(CapacityError::bounds(capacity, hint)),
            false => iter.try_for_each(|item| self.try_extend_one(item)),
        }
    }

    const fn try_extend_one(&mut self, item: T) -> Result<(), CapacityError<T>> {
        match self.back >= N {
            true => Err(CapacityError::overflowed(item)),
            false => {
                self.array[self.back].write(item);
                self.back += 1;
                Ok(())
            }
        }
    }
}

impl<T, const N: usize> Deref for PartialArray<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { self.array[..self.back].assume_init_ref() }
    }
}

impl<T, U, const N: usize> PartialEq<[U]> for PartialArray<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        self[..] == other[..]
    }
}

impl<T, const N: usize> IntoIterator for PartialArray<T, N> {
    type Item = T;
    type IntoIter = Drain<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Drain { guard: ManuallyDrop::new(self), next: 0 }
    }
}

impl<T, const N: usize> Drop for PartialArray<T, N> {
    fn drop(&mut self) {
        // SAFETY: elements up to `initialized` are initialized
        unsafe { self.array[..self.back].assume_init_drop() };
    }
}

/// An iterator that moves out of a [`PartialArray`].
#[derive(Debug)]
pub struct Drain<T, const N: usize> {
    guard: ManuallyDrop<PartialArray<T, N>>,
    next: usize,
}

impl<T, const N: usize> Iterator for Drain<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.next < self.guard.back).then(|| {
            let item = unsafe { self.guard.array[self.next].assume_init_read() };
            self.next += 1;
            item
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.guard.back - self.next;
        (len, Some(len))
    }
}

impl<T, const N: usize> ExactSizeIterator for Drain<T, N> {}
impl<T, const N: usize> FusedIterator for Drain<T, N> {}

impl<T, const N: usize> Drop for Drain<T, N> {
    fn drop(&mut self) {
        let back = self.guard.back;
        // SAFETY: elements between `next` and `back` are initialized
        (self.next < back).then(|| unsafe { self.guard.array[self.next..back].assume_init_drop() });
    }
}
