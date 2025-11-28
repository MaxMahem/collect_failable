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
