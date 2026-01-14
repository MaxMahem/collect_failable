#[cfg(doc)]
use crate::TryFromIterator;
#[cfg(doc)]
use std::collections::HashMap;

/// Trait for extending an existing collection from an iterator with fallible operations.
///
/// This trait is similar to [`Extend`], but allows implementor to uphold a container's invariant
/// during construction with a **basic error guarantee**. On an error, the collection may be
/// modified, but will be in a valid state. The specific extension that triggers the error must
/// not be inserted.
///
/// For a stronger error guarantee where the collection is unchanged on error, see
/// [`TryExtendSafe`].
///
/// Implementations may rely on [`Iterator::size_hint`] providing reliable bounds for the number of
/// elements in the iterator in order to optimize their implementations. An iterator that violates
/// the bounds returned by [`Iterator::size_hint`] may cause panics, produce incorrect results, or
/// produce a result that violates container constraints, but must not result in undefined behavior.
pub trait TryExtend<I: IntoIterator> {
    /// Error type returned by the fallible extension methods.
    type Error;

    /// Tries to extends the collection providing a **basic error guarantee**.
    ///
    /// On failure, the collection may be partially modified, but it must remain valid.
    /// The specific extension that triggers the error must not be inserted.
    ///
    /// For strong guarantee needs, see [`TryExtendSafe::try_extend_safe`].
    ///
    /// # Errors
    ///
    /// Returns [`TryExtend::Error`] if a failure occurs while extending the collection.
    ///
    /// # Examples
    ///
    /// The provided [`HashMap`] implementation errors if a key collision occurs during extension.
    ///
    /// ```rust
    /// use collect_failable::TryExtend;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// let err = map.try_extend([(2, 3), (3, 4), (1, 5)]).expect_err("should be err");
    ///
    /// assert_eq!(err.error.item, (1, 5));
    ///
    /// // map may be modified, but colliding value should not be changed
    /// assert_eq!(map[&1], 2);
    /// assert_eq!(map[&2], 3);
    /// assert_eq!(map[&3], 4);
    /// ```
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error>;
}

/// Trait for extending a collection with a **strong error guarantee**.
///
/// This trait extends [`TryExtend`] by providing a method that guarantees the collection
/// remains unchanged if an error occurs during extension.
///
/// Not all types can implement this trait. For example, tuples of collections cannot
/// provide this guarantee because if the second collection fails to extend, the first
/// may have already been modified.
///
/// Like with [`TryExtend`], implementors may rely on [`Iterator::size_hint`] providing reliable
/// bounds for the number of elements in the iterator in order to optimize their implementations.
/// An iterator that violates the bounds returned by [`Iterator::size_hint`] may cause panics,
/// produce incorrect results, or produce a result that violates container constraints, but must
/// not result in undefined behavior.
pub trait TryExtendSafe<I: IntoIterator>: TryExtend<I> {
    /// Tries to extends the collection providing a **strong error guarantee**.
    ///
    /// On failure, the collection must remain unchanged. Implementors may need to buffer
    /// elements or use a more defensive algorithm to satisfy this guarantee.
    ///
    /// For a faster basic-guarantee alternative, see [`TryExtend::try_extend`].
    ///
    /// # Errors
    ///
    /// Returns [`TryExtend::Error`] if a failure occurs while extending the collection.
    ///
    /// # Examples
    ///
    /// The provided [`HashMap`] implementation errors if a key collision occurs during extension.
    ///
    /// ```rust
    /// use collect_failable::TryExtendSafe;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::from([(1, 2), (2, 3)]);
    /// let err = map.try_extend_safe([(3, 4), (1, 5), (4, 6)]).expect_err("should collide");
    ///
    /// assert_eq!(err.error.item, (1, 5), "item should be the colliding item");
    ///
    /// let iterated_items: Vec<_> = err.into_iter().collect();
    /// // iterator can be reconstructed. Order is not guranteed for hashmap
    /// assert_eq!(iterated_items.len(), 3, "length should be unchanged");
    /// assert!(
    ///     iterated_items.contains(&(3, 4)) &&
    ///     iterated_items.contains(&(1, 5)) &&
    ///     iterated_items.contains(&(4, 6)),
    ///     "all items should be present"
    /// );
    ///
    /// assert_eq!(map, HashMap::from([(1, 2), (2, 3)]), "map should be unchanged");
    /// ```
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error>;
}

/// Extension trait providing convenience method for extending a collection with a single item.
///
/// Unlike [`TryExtend`], this trait works with individual items rather than iterators, it always
/// should provide a strong error guarantee, guaranteeing that the collection remains unchanged on error.
pub trait TryExtendOne {
    /// The type of item that can be extended into the collection.
    type Item;

    /// Error type returned by [`try_extend_one`](TryExtendOne::try_extend_one).
    type Error;

    /// Tries to extend the collection with a single item.
    ///
    /// This method provides a **strong error guarantee**: on failure, the collection
    /// remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the extension fails (e.g., due to capacity limits or
    /// key collisions).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::TryExtendOne;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// map.try_extend_one((2, 3)).expect("should succeed");
    /// assert_eq!(map.get(&2), Some(&3), "value should be inserted");
    ///
    /// // Collision error
    /// let err = map.try_extend_one((1, 5)).expect_err("should collide");
    /// assert_eq!(err.item, (1, 5), "item should be the colliding item");
    /// assert_eq!(map.get(&1), Some(&2), "original value should be unchanged");
    /// ```
    fn try_extend_one(&mut self, item: Self::Item) -> Result<(), Self::Error>;
}
