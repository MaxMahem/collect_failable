#[cfg(doc)]
use arrayvec::ArrayVec;

#[cfg(feature = "arrayvec")]
use arrayvec::CapacityError;

/// An error indicating that a duplicate key was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq, PartialOrd, Ord)]
#[error("Key collision")]
pub struct KeyCollision<K> {
    /// The key that caused the collision.
    pub key: K,
}

/// An error indicating that a duplicate value was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq, PartialOrd, Ord)]
#[error("Value collision")]
pub struct ValueCollision<T> {
    /// The value that caused the collision.
    pub value: T,
}

/// An error indicating that the number of items in the iterator did not match the expected count.
#[cfg(feature = "unsafe")]
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq, PartialOrd, Ord)]
#[error("Incorrect number of items to fill the array, expected {expected}, got {actual}")]
pub struct ItemCountMismatch {
    /// The expected number of items.
    pub expected: usize,
    /// The actual number of items.
    pub actual: usize,
}

/// An error type that can be one of two errors.
///
/// This is used when an operation can fail with one of two possible errors, for example when
/// unzipping an iterator into two collections, where either collection might fail to extend.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum OneOf2<ErrA, ErrB> {
    /// The operation failed with the first error.
    #[error(transparent)]
    A(ErrA),
    /// The operation failed with the second error.
    #[error(transparent)]
    B(ErrB),
}

/// An error produced when the iterator produces more items than the [`ArrayVec`]'s capacity.
#[cfg(feature = "arrayvec")]
#[derive(Debug, PartialEq, Eq, thiserror::Error, derive_more::Constructor)]
#[error("Too many items to fit the array, Capacity: {capacity}, necessary: {necessary}")]
pub struct ExceedsCapacity {
    /// The capacity of the [`ArrayVec`].
    pub capacity: usize,
    /// The number of items in the iterator.
    pub necessary: usize,
}

#[cfg(feature = "arrayvec")]
impl From<ExceedsCapacity> for CapacityError<()> {
    fn from(_: ExceedsCapacity) -> Self {
        CapacityError::new(())
    }
}
