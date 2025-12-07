use std::hash::{BuildHasher, Hash};

use fluent_result::into::IntoResult;
use hashbrown::hash_map::{Entry, EntryRef, RawEntryMut};
use hashbrown::HashMap;
use size_guess::SizeGuess;
use tap::Pipe;

use crate::utils::FoldMut;
use crate::{CollectionCollision, KeyCollision, TryExtend, TryExtendSafe, TryFromIterator};

/// Converts an iterator of key-value pairs into a [`HashMap`], failing if a key would collide.
impl<K: Eq + Hash, V, I> TryFromIterator<(K, V), I> for HashMap<K, V> 
where
    I: IntoIterator<Item = (K, V)>
{
    type Error = CollectionCollision<(K, V), I::IntoIter, HashMap<K, V>>;

    /// Converts an iterator of key-value pairs into a [`HashMap`] failing if a key would collide.
    /// 
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_hint().0;

        iter.try_fold(HashMap::with_capacity(size_guess), |mut map, (key, value)| {
            let hash = map.hasher().hash_one(&key);
            match map.raw_entry_mut().from_hash(hash, |k| k == &key) {
                RawEntryMut::Occupied(_) => Err((map, (key, value))),
                RawEntryMut::Vacant(entry) => {
                    entry.insert_hashed_nocheck(hash, key, value);
                    Ok(map)
                }
            }
        })
        .map_err(|(map, kvp)| CollectionCollision::new(iter, map, kvp))
    }
}

/// Appends an iterator of key-value pairs to the map, failing if a key would collide.
impl<K: Eq + Hash, V, S: BuildHasher> TryExtend<(K, V)> for HashMap<K, V, S> {
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
impl<K: Eq + Hash, V, S: BuildHasher> TryExtendSafe<(K, V)> for HashMap<K, V, S> {
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

        iter.try_fold_mut(HashMap::with_capacity(size_guess), |map, (key, value)| match self.entry_ref(&key) {
            EntryRef::Vacant(_) => match map.entry(key) {
                Entry::Vacant(entry) => Ok(_ = entry.insert(value)),
                Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
            },
            EntryRef::Occupied(_) => KeyCollision::new(key).into_err(),
        })
        .map(|map| self.extend(map))
    }
}
