use std::hash::{BuildHasher, Hash};

use fluent_result::IntoResult;
use indexmap::map::Entry;
use indexmap::IndexMap;
use size_guess::SizeGuess;
use tap::Pipe;

use crate::{KeyCollision, TryExtend, TryFromIterator};

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

        iter.try_fold(
            IndexMap::with_capacity(size_guess),
            |mut map, (k, v)| match map.entry(k) {
                Entry::Occupied(entry) => entry
                    .shift_remove_entry()
                    .0
                    .pipe(KeyCollision::new)
                    .into_err(),
                Entry::Vacant(entry) => {
                    entry.insert(v);
                    Ok(map)
                }
            },
        )
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> TryExtend<(K, V)> for IndexMap<K, V, S> {
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
        let size_guess = iter.size_guess();

        let mut insert_map = IndexMap::with_capacity(size_guess);

        for (key, value) in iter {
            match self.contains_key(&key) {
                false => match insert_map.entry(key) {
                    #[rustfmt::skip]
                    Entry::Occupied(entry) => return entry.swap_remove_entry().0.pipe(KeyCollision::new).into_err(),
                    Entry::Vacant(entry) => _ = entry.insert(value),
                },
                true => return Err(KeyCollision::new(key)),
            }
        }

        self.extend(insert_map);

        Ok(())
    }
}
