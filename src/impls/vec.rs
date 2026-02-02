use crate::errors::capacity::{FixedCap, RemainingCap};
use crate::errors::types::SizeHint;
use alloc::vec::Vec;

impl<T> RemainingCap for Vec<T> {
    /// Returns [`SizeHint::unbounded(0)`](SizeHint::unbounded) because [`Vec`]
    /// can grow indefinitely.
    fn remaining_cap(&self) -> SizeHint {
        SizeHint::unbounded(0)
    }
}

impl<T> FixedCap for Vec<T> {
    /// Returns [`SizeHint::unbounded(0)`](SizeHint::unbounded) because [`Vec`]
    /// has no fixed capacity.
    const CAP: SizeHint = SizeHint::unbounded(0);
}
