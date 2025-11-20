/// Trait for failably extending a collection from an iterator.
pub trait TryExtend<T> {
    /// Error type returned by [`TryExtend::try_extend`].
    type Error;

    /// Tries to extend the collection with the contents of the iterator.
    ///
    /// Implementors are required to provide a strong error guarantee. On a failure, the original
    /// collection should not be modified. This may require allocations and/or a less optimal
    /// algorithm.
    ///
    /// For a non-safe version, see [`TryExtend::try_extend`].
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
    /// let result = map.try_extend_safe([(2, 4), (2, 5)]);
    ///
    /// // result is an error with the colliding key
    /// assert_eq!(result, Err(KeyCollision { key: 2 }));
    ///
    /// // map is unchanged
    /// assert_eq!(map, HashMap::from([(1, 2)]));
    ///
    /// // works like `extend` if there are no collisions
    /// let mut map = HashMap::from([(1, 2)]);
    /// let result = map.try_extend_safe([(2, 3)]);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
    /// ```
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>;

    /// Tries to extend the collection with the contents of the iterator.
    ///
    /// Unlike [`TryExtend::try_extend_safe`], this method does not provide a strong error guarantee.
    /// If the method returns an error, the collection may be modified. However, it should still be
    /// in a valid state, and the specific extension that caused the error should not take effect.
    ///
    /// For a safe version, see [`TryExtend::try_extend_safe`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::{TryExtend, KeyCollision};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// let err = map.try_extend([(2, 3), (1, 3)]).expect_err("should be err");
    ///
    /// assert_eq!(err, KeyCollision { key: 1 });
    ///
    /// // map may be modified, but colliding value should not be changed
    /// assert_eq!(map[&1], 2);
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        self.try_extend_safe(iter)
    }
}
