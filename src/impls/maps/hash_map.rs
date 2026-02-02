use core::hash::{BuildHasher, Hash};
use std::collections::HashMap;

use fluent_result::expect::dbg::ExpectNone;

crate::impls::macros::impl_try_from_iter_via_try_extend_one! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Default] of (K, V);
    ctor: |iter| HashMap::with_capacity_and_hasher(iter.size_hint().0, S::default())
);

crate::impls::macros::impl_try_extend_via_try_extend_one! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Clone] of (K, V);
    reserve: |map, iter| map.reserve(iter.size_hint().0)
);

crate::impls::macros::impl_try_extend_safe_for_colliding_type! (
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher + Clone] of (K, V);
    build_staging: |iter, map| HashMap::with_capacity_and_hasher(iter.size_hint().0, map.hasher().clone());
    contains: |map, (key, _)| map.contains_key(key)
);

crate::impls::macros::impl_try_extend_one_for_colliding_type!(
    type: HashMap<K, V, S> where [K: Eq + Hash, V, S: BuildHasher] of (K, V);
    contains: |map, (key, _)| map.contains_key(key);
    insert: |map, (key, value)| map.insert(key, value).expect_none("should not be in map")
);
