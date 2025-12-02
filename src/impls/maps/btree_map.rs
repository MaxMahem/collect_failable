use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use fluent_result::into::IntoResult;
use tap::Pipe;

use crate::utils::FoldMut;
use crate::{KeyCollision, TryExtend, TryExtendSafe, TryFromIterator};

/// Converts an iterator of key-value pairs into a [`BTreeMap`], failing if a key would collide.
impl<K: Ord, V> TryFromIterator<(K, V)> for BTreeMap<K, V> {
    type Error = KeyCollision<K>;

    /// Converts an iterator of key-value pairs into a [`BTreeMap`], failing if a key would collide.
    ///
    /// Note: In the case of a collision, technically the key returned by [`KeyCollision`] is the
    /// first key that was seen during iteration, not the second key that collided.
    ///
    /// In the case of a collision, the key held by [`KeyCollision`] is the first key that was seen
    /// during iteration. This may be relevant for keys that compare the same but still have
    /// different values.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter<I>(into_iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        into_iter.into_iter().try_fold_mut(BTreeMap::new(), |map, (key, value)| match map.entry(key) {
            Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
            Entry::Vacant(entry) => Ok(_ = entry.insert(value)),
        })
    }
}

/// Appends an iterator of key-value pairs to the map, failing if a key would collide.
impl<K: Ord, V> TryExtend<(K, V)> for BTreeMap<K, V> {
    type Error = KeyCollision<K>;

    /// Appends an iterator of key-value pairs to the map, failing if a key would collide.
    ///
    /// This implementation provides a basic error guarantee. If the method returns an error, the
    /// map may be modified. However, it will still be in a valid state, and the specific
    /// collision that caused the error will not take effect.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter().try_for_each(|(key, value)| match self.contains_key(&key) {
            true => KeyCollision::new(key).into_err(),
            false => Ok(_ = self.insert(key, value)),
        })
    }
}

/// Appends an iterator of key-value pairs to the map with a strong error guarantee.
impl<K: Ord, V> TryExtendSafe<(K, V)> for BTreeMap<K, V> {
    /// Appends an iterator of key-value pairs to the map, failing if a key would collide.
    ///
    /// This implementation provides a strong error guarantee. If the method returns an error, the
    /// map is not modified.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter()
            .try_fold_mut(BTreeMap::new(), |map, (key, value)| match self.contains_key(&key) {
                true => KeyCollision::new(key).into_err(),
                false => match map.entry(key) {
                    Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
                    Entry::Vacant(entry) => Ok(_ = entry.insert(value)),
                },
            })
            .map(|map| self.extend(map))
    }
}
