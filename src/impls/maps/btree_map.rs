use alloc::collections::BTreeMap;

use fluent_result::expect::dbg::ExpectNone;

crate::impls::macros::impl_try_from_iter_via_try_extend_one! (
    type: BTreeMap<K, V> where [K: Ord, V] of (K, V);
    ctor: |_| Self::new()
);

crate::impls::macros::impl_try_extend_via_try_extend_one! (
    type: BTreeMap<K, V> where [K: Ord, V] of (K, V);
    reserve: |_, _| {}
);

crate::impls::macros::impl_try_extend_safe_for_colliding_type! (
    type: BTreeMap<K, V> where [K: Ord, V] of (K, V);
    build_staging: |_, _| BTreeMap::new();
    contains: |map, (key, _)| map.contains_key(key)
);

crate::impls::macros::impl_try_extend_one_for_colliding_type!(
    type: BTreeMap<K, V> where [K: Ord, V] of (K, V);
    contains: |map, (key, _)| map.contains_key(key);
    insert: |map, (key, value)| map.insert(key, value).expect_none("should not be in map")
);
