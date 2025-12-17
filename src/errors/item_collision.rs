/// Error type for when a single item cannot be added to a collection due to a collision.

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("item collision")]
pub struct ItemCollision<T> {
    /// The item that could not be inserted due to a collision.
    pub item: T,
}

impl<T> ItemCollision<T> {
    /// Creates a new `ItemCollision` error with the rejected item.
    #[must_use]
    pub const fn new(item: T) -> Self {
        Self { item }
    }
}
