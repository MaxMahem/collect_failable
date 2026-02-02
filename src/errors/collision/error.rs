use crate::errors::ErrorItemProvider;

/// Error type for when a single item cannot be added to a collection due to a collision.
///
/// This is commonly used with map and set collection where duplicate keys/values are not allowed.
///
/// # Type Parameters
///
/// - `T`: The type of the item that caused the collision.
///
/// # Examples
///
/// ```rust
/// # use collect_failable::errors::collision::Collision;
/// let error = Collision::new(1);
/// assert_eq!(error.item, 1);
/// ```
#[derive(Debug, PartialEq, Eq, thiserror::Error, derive_more::Constructor)]
#[error("item collision")]
pub struct Collision<T> {
    /// The item that could not be inserted due to a collision.
    pub item: T,
}

impl<T> ErrorItemProvider for Collision<T> {
    type Item = T;

    fn into_item(self) -> Option<Self::Item> {
        Some(self.item)
    }

    fn item(&self) -> Option<&Self::Item> {
        Some(&self.item)
    }
}
