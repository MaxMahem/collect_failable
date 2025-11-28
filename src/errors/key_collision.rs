/// An error indicating that a duplicate key was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq)]
#[error("Key collision")]
pub struct KeyCollision<K> {
    /// The key that caused the collision.
    pub key: K,
}
