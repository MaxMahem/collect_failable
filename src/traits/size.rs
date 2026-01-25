use crate::SizeHint;

/// A trait for types with a dynamic item capacity, expressed as a [`SizeHint`].
///
/// This is the remaining capacity of the collection,
/// and may change if the collection is modified.
pub trait RemainingCap {
    /// Returns the remaining capacity of this collection as a [`SizeHint`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::{RemainingCap, SizeHint};
    /// # use arrayvec::ArrayVec;
    /// let vec = ArrayVec::<i32, 4>::new();
    /// assert_eq!(vec.remaining_cap(), SizeHint::at_most(4));
    /// ```
    fn remaining_cap(&self) -> SizeHint;
}

/// A trait for types with a static item capacity, expressed as a [`SizeHint`].
///
/// This is the static capacity of the collection when empty, and it should never change.
pub trait FixedCap: RemainingCap {
    /// The static capacity of this collection as a [`SizeHint`].
    const CAP: SizeHint;
}
