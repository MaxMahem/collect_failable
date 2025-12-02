use std::hash::{BuildHasher, Hash};

use fluent_result::into::IntoResult;
use indexmap::map::Entry;
use indexmap::IndexMap;
use size_guess::SizeGuess;
use tap::Pipe;

use crate::utils::FoldMut;
use crate::{KeyCollision, TryExtend, TryExtendSafe, TryFromIterator};

/// Converts an iterator of key-value pairs into a [`IndexMap`], failing if a key would collide.
impl<K: Eq + Hash, V> TryFromIterator<(K, V)> for IndexMap<K, V> {
    type Error = KeyCollision<K>;

    /// Converts an iterator of key-value pairs into a hash-map, failing if a key would collide.
    ///
    /// In the case of a collision, the key held by [`KeyCollision`] is the first key that was seen
    /// during iteration. This may be relevant for keys that compare the same but still have
    /// different values.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter<I>(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
        I: IntoIterator<Item = (K, V)>,
    {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_guess();

        iter.try_fold_mut(IndexMap::with_capacity(size_guess), |map, (k, v)| match map.entry(k) {
            Entry::Occupied(entry) => entry.shift_remove_entry().0.pipe(KeyCollision::new).into_err(),
            Entry::Vacant(entry) => Ok(_ = entry.insert(v)),
        })
    }
}

/// Appends an iterator of key-value pairs to the map, failing if a key would collide.
impl<K: Eq + Hash, V, S: BuildHasher> TryExtend<(K, V)> for IndexMap<K, V, S> {
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
        let mut iter = iter.into_iter();
        self.reserve(iter.size_guess());

        iter.try_for_each(|(key, value)| match self.contains_key(&key) {
            true => KeyCollision::new(key).into_err(),
            false => Ok(_ = self.insert(key, value)),
        })
    }
}

/// Appends an iterator of key-value pairs to the map with a strong error guarantee.
impl<K: Eq + Hash, V, S: BuildHasher> TryExtendSafe<(K, V)> for IndexMap<K, V, S> {
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
        let mut iter = iter.into_iter();
        let size_guess = iter.size_guess();

        iter.try_fold_mut(IndexMap::with_capacity(size_guess), |map, (key, value)| match self.contains_key(&key) {
            true => KeyCollision::new(key).into_err(),
            false => match map.entry(key) {
                Entry::Occupied(entry) => entry.swap_remove_entry().0.pipe(KeyCollision::new).into_err(),
                Entry::Vacant(entry) => Ok(_ = entry.insert(value)),
            },
        })
        .map(|insert_map| self.extend(insert_map))
    }
}
