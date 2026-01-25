use core::hash::{BuildHasher, Hash};

use fluent_result::bool::dbg::Expect;
use indexmap::IndexSet;

crate::impls::macros::impl_try_from_iter_via_try_extend_one!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher + Default] of T;
    ctor: |iter| IndexSet::with_capacity_and_hasher(iter.size_hint().0, S::default())
);

crate::impls::macros::impl_try_extend_via_try_extend_one!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher + Clone] of T;
    reserve: |set, iter| set.reserve(iter.size_hint().0);
    build_empty: |set| IndexSet::with_hasher(set.hasher().clone())
);

crate::impls::macros::impl_try_extend_safe_for_colliding_type!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher + Clone] of T;
    build_staging: |iter, set| IndexSet::with_capacity_and_hasher(iter.size_hint().0, set.hasher().clone());
    contains: IndexSet::contains
);

crate::impls::macros::impl_try_extend_one_for_colliding_type!(
    type: IndexSet<T, S> where [T: Eq + Hash, S: BuildHasher] of T;
    contains: IndexSet::contains;
    insert: |set, item| set.insert(item).expect_true("insert should succeed after contains check")
);
