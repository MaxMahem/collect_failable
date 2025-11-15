/// Trait for failably extending a collection from an iterator.
pub trait TryExtend<T> {
    /// Error type returned by [`TryExtend::try_extend`].
    type Error;

    /// Tries to extend the collection with the contents of the iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::{TryExtend, NonUniqueKey};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// let result = map.try_extend([(1, 3)]);
    ///
    /// assert!(result.is_err());
    ///
    /// // map is unchanged
    /// assert_eq!(1, map.len());
    /// assert_eq!(map.get(&1), Some(&2));
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// let result = map.try_extend([(2, 3)]);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(2, map.len());
    /// assert_eq!(map.get(&1), Some(&2));
    /// assert_eq!(map.get(&2), Some(&3));
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>;
}

/// A key was not unique in a [`TryExtend`] operation.
#[derive(Debug, thiserror::Error)]
#[error("One or more keys were not unique")]
pub struct NonUniqueKey;
