use alloc::vec::Vec;

use crate::{Capacity, SizeHint};

impl<T> Capacity for Vec<T> {
    fn capacity_hint(&self) -> SizeHint {
        SizeHint::UNIVERSAL
    }
}
