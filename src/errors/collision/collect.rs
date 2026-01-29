use crate::errors::CollectError;
use crate::errors::collision::error::Collision;

/// Specialization of [`CollectError`] for [`Collision`].
///
/// This type is used when collection fails due to a collision. Such as a
/// duplicate key in a map or set. The [`Collision`] will contain the item
/// that collided during collection.
///
/// # Type Parameters
///
/// - `I`: The type of the [Iterator] that was used to collect the values.
/// - `C`: The type of the collection that was used to collect the values.
///
/// # Data Recovery
///
/// If `C` implements [`IntoIterator`], this type implements [`IntoIterator`],
/// as well, allowing the data in the original iterator to be reconstructed from
/// [`CollectError::iter`], [`CollectError::collected`], and the colliding item.
///
/// # Examples
///
/// ```rust
/// # use collect_failable::errors::CollectError;
/// # use collect_failable::errors::collision::Collision;
/// # use std::collections::HashSet;
/// let error = CollectError::<_, HashSet<_>, _>::collision(1..=1, HashSet::from([1]), 1);
///
/// let values = error.into_iter().collect::<Vec<_>>();
///
/// assert_eq!(values.len(), 3, "Should have 3 values");
/// assert!(values.iter().all(|v| v == &1), "Should only contain 1");
/// ```
impl<I: Iterator, C> CollectError<I, C, Collision<I::Item>> {
    /// Creates a new [`CollectError`] with a [`Collision`] error, for collection failures
    /// due to a collision.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after the collision occurred
    /// * `collected` - The values that were collected before the collision
    /// * `item` - The item that caused the collision
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::CollectError;
    /// # use collect_failable::errors::collision::Collision;
    /// # use std::collections::HashSet;
    /// let error = CollectError::<_, HashSet<_>, _>::collision(1..=3, HashSet::from([1, 2]), 3);
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, HashSet::from([1, 2]));
    /// assert_eq!(error.error.item, 3);
    /// ```
    #[must_use]
    #[inline]
    pub fn collision(iter: I, collected: C, item: I::Item) -> Self {
        Self::new(iter, collected, Collision::new(item))
    }
}
