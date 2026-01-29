use crate::errors::types::SizeHint;

/// A trait for types with a dynamic item capacity, expressed as a [`SizeHint`].
///
/// This is the remaining capacity of the collection,
/// and may change if the collection is modified.
pub trait RemainingCap {
    /// Returns the remaining capacity of this collection as a [`SizeHint`].
    fn remaining_cap(&self) -> SizeHint;
}

/// A trait for types with a static item capacity, expressed as a [`SizeHint`].
///
/// This is the static capacity of the collection when empty, and it should never change.
pub trait FixedCap: RemainingCap {
    /// The static capacity of this collection as a [`SizeHint`].
    const CAP: SizeHint;
}
