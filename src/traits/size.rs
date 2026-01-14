use crate::SizeHint;

/// A trait for types with a dynamic item capacity, expressed as a [`SizeHint`].
///
/// This is the remaining capacity of the collection, and may change if the collection is modified.
pub trait RemainingSize {
    /// Returns the remaining capacity of this collection as a [`SizeHint`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::{RemainingSize, SizeHint};
    ///
    /// let vec = Vec::<i32>::new();
    /// assert_eq!(vec.remaining_size(), SizeHint::UNIVERSAL);
    /// ```
    fn remaining_size(&self) -> SizeHint;
}

/// A trait for types with a static item capacity, expressed as a [`SizeHint`].
///
/// This is the static, maximum capacity of the collection, and it should never change.
pub trait MaxSize: RemainingSize {
    /// The static capacity of this collection as a [`SizeHint`].
    const MAX_SIZE: SizeHint;
}
