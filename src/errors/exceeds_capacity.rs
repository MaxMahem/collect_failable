#[cfg(doc)]
use arrayvec::ArrayVec;

use arrayvec::CapacityError;

/// An error produced when the iterator produces more items than the [`ArrayVec`]'s capacity.
#[derive(Debug, PartialEq, Eq, thiserror::Error, derive_more::Constructor)]
#[error("Too many items to fit the array, Capacity: {capacity}, necessary: {necessary}")]
pub struct ExceedsCapacity {
    /// The capacity of the [`ArrayVec`].
    pub capacity: usize,
    /// The number of items in the iterator.
    pub necessary: usize,
}

impl From<ExceedsCapacity> for CapacityError<()> {
    fn from(_: ExceedsCapacity) -> Self {
        CapacityError::new(())
    }
}
