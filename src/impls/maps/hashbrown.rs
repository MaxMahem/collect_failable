use core::hash::{BuildHasher, Hash};

use hashbrown::HashMap;
use hashbrown::hash_map::RawEntryMut;

use crate::errors::{CollectionError, Collision};

use crate::{TryExtendOne, TryExtendSafe};

crate::impls::macros::impl_try_from_iter_via_try_extend_one! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Default] of (K, V);
    ctor: |iter| HashMap::with_capacity_and_hasher(iter.size_hint().0, S::default())
);

crate::impls::macros::impl_try_extend_via_try_extend_one! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Clone] of (K, V);
    reserve: |map, iter| map.reserve(iter.size_hint().0);
    build_empty: |map| { <HashMap<K, V, S>>::with_hasher(map.hasher().clone()) }
);

impl<K: Eq + Hash, V, S: BuildHasher + Clone, I> TryExtendSafe<I> for HashMap<K, V, S>
where
    I: IntoIterator<Item = (K, V)>,
{
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();

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

    fn try_extend_one(&mut self, (key, value): Self::Item) -> Result<(), Self::Error> {
        let hash = self.hasher().hash_one(&key);
        match self.raw_entry_mut().from_hash(hash, |k| k == &key) {
            RawEntryMut::Occupied(_) => Err(Collision::new((key, value))),
            RawEntryMut::Vacant(entry) => {
                entry.insert_hashed_nocheck(hash, key, value);
                Ok(())
            }
        }
    }
}
