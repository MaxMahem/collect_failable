/// Trait for failably extending a collection from an iterator.
pub trait TryExtend<T> {
    /// Error type returned by [`TryExtend::try_extend`].
    type Error;

    /// Tries to extend the collection with the contents of the iterator.
    ///
    /// # Examples
    ///
    /// Provided implementations of `TryExtend` for map types, `HashMap`, `BTreeMap`,
    /// `hashbrown::HashMap`, and `indexmap::IndexMap`, all work similiarly and provide strong
    /// error guarantees. If the method returns an error, the map is not modified.
    ///
    /// ```rust
    /// use collect_failable::{TryExtend, KeyCollision};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// let result = map.try_extend([(1, 3)]);
    ///
    /// assert_eq!(result, Err(KeyCollision { key: 1 }));
    ///
    /// // map is unchanged
    /// assert_eq!(map, HashMap::from([(1, 2)]));
    ///
    /// // collisions within the iterator itself are also detected
    /// let result = map.try_extend([(2, 4), (2, 5)]);
    ///
    /// // result is an error with the colliding key
    /// assert_eq!(result, Err(KeyCollision { key: 2 }));
    ///
    /// // map is unchanged
    /// assert_eq!(map, HashMap::from([(1, 2)]));
    ///
    /// // works like `extend` if there are no collisions
    /// let mut map = HashMap::from([(1, 2)]);
    /// let result = map.try_extend([(2, 3)]);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>;
}
