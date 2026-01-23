use alloc::collections::BTreeSet;

crate::impls::macros::impl_try_from_iter_via_try_extend_one!(
    type: BTreeSet<T> where [T: Ord] of T;
    ctor: |_| Self::new()
);

crate::impls::macros::impl_try_extend_via_try_extend_one!(
    type: BTreeSet<T> where [T: Ord] of T;
    reserve: |_, _| ();
    build_empty_collection: |_| BTreeSet::new()
);

crate::impls::macros::impl_try_extend_safe_for_colliding_type!(
    type: BTreeSet<T> where [T: Ord] of T;
    build_staging: |_, _| BTreeSet::new();
    contains: |set, item| set.contains(item)
);

crate::impls::macros::impl_try_extend_one_for_colliding_type!(
    type: BTreeSet<T> where [T: Ord] of T;
    contains: |set, item| set.contains(item);
    insert: |set, item| { set.insert(item); }
);
