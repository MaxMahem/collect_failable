use crate::SizeHint;

/// A trait for types with a known capacity that can be expressed as a [`SizeHint`].
///
/// This trait is primarily used to simplify the implementation of `CollectionError`
/// helper constructors, but can be implemented for custom types as well.
pub trait Capacity {
    /// Returns the capacity of this collection as a [`SizeHint`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::{Capacity, SizeHint};
    ///
    /// let vec = Vec::<i32>::new();
    /// assert_eq!(vec.capacity_hint(), SizeHint::UNIVERSAL);
    /// ```
    fn capacity_hint(&self) -> SizeHint;
}
