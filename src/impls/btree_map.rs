use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use fluent_result::IntoResult;
use tap::Pipe;

use crate::{KeyCollision, TryExtend, TryFromIterator};

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
        let mut iter = into_iter.into_iter();
        iter.try_fold(BTreeMap::new(), |mut map, (k, v)| match map.entry(k) {
            Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
            Entry::Vacant(entry) => {
                entry.insert(v);
                Ok(map)
            }
        })
    }
}

impl<K: Ord, V> TryExtend<(K, V)> for BTreeMap<K, V> {
    type Error = KeyCollision<K>;

    /// Appends an iterator of key-value pairs to the map, failing if a key would collide.
    ///
    /// This implementation provides a strong error guarantee. If the method returns an error, the
    /// map is not modified.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let iter = iter.into_iter();

        let mut insert_map = BTreeMap::new();

        for (key, value) in iter {
            match self.contains_key(&key) {
                false => match insert_map.entry(key) {
                    #[rustfmt::skip]
                    Entry::Occupied(entry) => return entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
                    Entry::Vacant(entry) => _ = entry.insert(value),
                },
                true => return Err(KeyCollision::new(key)),
            }
        }

        self.extend(insert_map);

        Ok(())
    }
}
