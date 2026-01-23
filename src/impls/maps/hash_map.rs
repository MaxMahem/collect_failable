use core::hash::{BuildHasher, Hash};
use std::collections::HashMap;

crate::impls::macros::impl_try_from_iter_via_try_extend_one! (
    type: HashMap<K, V> where [K: Eq + Hash, V] of (K, V);
    ctor: |iter| Self::with_capacity(iter.size_hint().0)
);

crate::impls::macros::impl_try_extend_via_try_extend_one! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Clone] of (K, V);
    reserve: |iter, map| map.reserve(iter.size_hint().0);
    build_empty_collection: |map: &mut HashMap<K, V, S>| { <HashMap<K, V, S>>::with_hasher(map.hasher().clone()) }
);

crate::impls::macros::impl_try_extend_safe_for_colliding_type! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Clone] of (K, V);
    build_staging: |map: &mut HashMap<K, V, S>, iter| HashMap::with_capacity_and_hasher(iter.size_hint().0, map.hasher().clone());
    contains: |map, (key, _)| map.contains_key(key)
);

crate::impls::macros::impl_try_extend_one_for_colliding_type!(
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher] of (K, V);
    contains: |map, (key, _)| map.contains_key(key);
    insert: |map, (key, value)| { map.insert(key, value); }
);
