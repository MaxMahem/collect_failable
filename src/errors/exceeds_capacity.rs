#[cfg(doc)]
use arrayvec::ArrayVec;

use arrayvec::CapacityError;

/// An error produced when the iterator produces more items than the [`ArrayVec`]'s capacity.
#[derive(Debug, PartialEq, Eq, thiserror::Error, derive_more::Constructor)]
#[error("Too many items to fit the array, Capacity: {capacity}, required: {required}")]
pub struct ExceedsCapacity {
    /// The capacity of the [`ArrayVec`].
    pub capacity: usize,
    /// The lower bound of the number of items required to contain the iterator.
    pub required: usize,
}

impl From<ExceedsCapacity> for CapacityError<()> {
    fn from(_: ExceedsCapacity) -> Self {
        CapacityError::new(())
    }
}
