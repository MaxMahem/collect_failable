/// An error indicating that a duplicate value was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq)]
#[error("Value collision")]
pub struct ValueCollision<T> {
    /// The value that caused the collision.
    pub value: T,
}
