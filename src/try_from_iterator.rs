/// Allows converting an iterator into a container that may fail to be constructed.
pub trait TryFromIterator<T> {
    /// The error that may occur when converting the iterator into the container.
    type Error;

    /// Converts an iterator into a container that may fail to be constructed.
    ///
    /// # Example
    ///
    /// Provided implementations for `HashMap`, `BTreeMap`, `hashbrown::HashMap`, and
    /// `indexmap::IndexMap` all work similarly, and provide the first instance of a
    /// colliding key within the [`KeyCollision`] error.
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use collect_failable::{TryFromIterator, KeyCollision};
    ///
    /// let result = HashMap::try_from_iter([(1, 2), (1, 3)]);
    /// assert_eq!(result, Err(KeyCollision { key: 1 }));
    ///
    /// let result = HashMap::try_from_iter([(1, 2), (2, 3)]);
    /// assert_eq!(result, Ok(HashMap::from([(1, 2), (2, 3)])));
    /// ```
    fn try_from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// An error indicating that a duplicate key was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq, PartialOrd, Ord)]
#[error("Key collision: {key}")]
pub struct KeyCollision<K> {
    /// The key that caused the collision.
    pub key: K,
}

/// An error indicating that a duplicate value was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueCollision<T> {
    /// The value that caused the collision.
    pub value: T,
}
