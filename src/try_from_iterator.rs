/// Allows converting an iterator into a container that may fail to be constructed.
pub trait TryFromIterator<T> {
    /// The error that may occur when converting the iterator into the container.
    type Error;

    /// Converts an iterator into a container that may fail to be constructed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use collect_failable::TryFromIterator;
    ///
    /// let err = HashMap::try_from_iter([(1, 2), (1, 3)]).expect_err("should be err");
    /// assert_eq!(err.key, 1);
    /// ```
    fn try_from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// An error indicating that a duplicate key was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor)]
#[error("Key collision: {key}")]
pub struct KeyCollision<K> {
    /// The key that caused the collision.
    pub key: K,
}
