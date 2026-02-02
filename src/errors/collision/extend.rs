use crate::errors::ExtendError;
use crate::errors::collision::error::Collision;

/// Specialization of [`ExtendError`] for [`Collision`] errors.
///
/// This is used when collecting into a collection that checks for duplicate items,
/// such as [`HashSet`](std::collections::HashSet) or [`HashMap`](std::collections::HashMap).
///
/// # Type Parameters
///
/// * `I` - The [`Iterator`] that produced the colliding item
impl<I: Iterator> ExtendError<I, Collision<I::Item>> {
    /// Creates a new [`ExtendError`] with a [`Collision`] error, for extension
    /// failures due to a collision.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after the collision occurred
    /// * `item` - The item that caused the collision
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::ExtendError;
    /// # use collect_failable::errors::collision::Collision;
    /// let error = ExtendError::collision(1..=3, 4);
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.error.item, 4);
    /// ```
    #[must_use]
    #[inline]
    pub fn collision(iter: I, item: I::Item) -> Self {
        Self::new(iter, Collision::new(item))
    }
}
