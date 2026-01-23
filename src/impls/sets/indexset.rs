use core::hash::{BuildHasher, Hash};

use indexmap::IndexSet;

crate::impls::macros::impl_try_from_iter_via_try_extend_one!(
    type: IndexSet<T> where [T: Eq + Hash] of T;
    ctor: |iter| Self::with_capacity(iter.size_hint().0)
);

crate::impls::macros::impl_try_extend_via_try_extend_one!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher + Clone] of T;
    reserve: |iter, set| set.reserve(iter.size_hint().0);
    build_empty_collection: |set: &mut IndexSet<T, S>| IndexSet::with_hasher(set.hasher().clone())
);

crate::impls::macros::impl_try_extend_safe_for_colliding_type!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher + Clone] of T;
    build_staging: |set: &mut IndexSet<T, S>, iter| IndexSet::with_capacity_and_hasher(iter.size_hint().0, set.hasher().clone());
    contains: |set, item| set.contains(item)
);

crate::impls::macros::impl_try_extend_one_for_colliding_type!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher] of T;
    contains: |set, item| set.contains(item);
    insert: |set, item| { set.insert(item); }
);
