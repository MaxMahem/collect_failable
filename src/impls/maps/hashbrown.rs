use core::hash::{BuildHasher, Hash};

use hashbrown::hash_map::RawEntryMut;
use hashbrown::HashMap;

use crate::errors::{CollectionError, Collision};
use crate::impls::try_extend_basic;
use crate::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

impl<K: Eq + Hash, V, I> TryFromIterator<I> for HashMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionError<I::IntoIter, Self, Collision<(K, V)>>;

    /// Converts `iter` into a [`HashMap`], failing if a key would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_hint().0;
        let mut map = Self::with_capacity(size_guess);

        match try_extend_basic(&mut map, &mut iter) {
            Ok(()) => Ok(map),
            Err(err) => Err(CollectionError::new(iter, map, err)),
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Clone, I> TryExtend<I> for HashMap<K, V, S>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionError<I::IntoIter, Self, Collision<(K, V)>>;

    /// Extends the map with `iter`, failing if a key would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        try_extend_basic(self, &mut iter).map_err(|err| CollectionError::new(iter, Self::with_hasher(self.hasher().clone()), err))
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Clone, I> TryExtendSafe<I> for HashMap<K, V, S>
where
    I: IntoIterator<Item = (K, V)>,
{
    /// Extends the map with `iter`, erroring if a key would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();

        // uses the same hasher as the main map in order to allow the has only have to be computed once
        let staging_map = Self::with_capacity_and_hasher(iter.size_hint().0, self.hasher().clone());

        iter.try_fold(staging_map, |mut staging_map, (key, value)| {
            let shared_hash = staging_map.hasher().hash_one(&key);

            // check for an entry in the staging map
            match staging_map.raw_entry_mut().from_hash(shared_hash, |k| k == &key) {
                RawEntryMut::Occupied(_) => Err((staging_map, (key, value))),

                // check for an entry in the main map
                RawEntryMut::Vacant(staging_entry) => match self.raw_entry().from_hash(shared_hash, |k| k == &key) {
                    Some(_) => Err((staging_map, (key, value))),
                    None => {
                        staging_entry.insert_hashed_nocheck(shared_hash, key, value);
                        Ok(staging_map)
                    }
                },
            }
        })
        .map(|staging_map| self.extend(staging_map))
        .map_err(|(staging_map, kvp)| CollectionError::collision(iter, staging_map, kvp))
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> TryExtendOne for HashMap<K, V, S> {
    type Item = (K, V);
    type Error = Collision<(K, V)>;

    fn try_extend_one(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        let hash = self.hasher().hash_one(&item.0);
        match self.raw_entry_mut().from_hash(hash, |k| k == &item.0) {
            RawEntryMut::Occupied(_) => Err(Collision::new(item)),
            RawEntryMut::Vacant(entry) => {
                entry.insert_hashed_nocheck(hash, item.0, item.1);
                Ok(())
            }
        }
    }
}
