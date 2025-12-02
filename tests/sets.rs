use std::collections::{BTreeSet, HashSet};

use hashbrown::HashSet as HashBrownSet;

use collect_failable::utils::Identifiable;
use collect_failable::{TryExtend, TryExtendSafe, TryFromIterator};

macro_rules! try_from_iter_and_extend_iter {
    ($module:ident, $set_type:ty) => {
        mod $module {
            use super::*;

            use collect_failable::ValueCollision;

            const COLLIDE_WITH_SELF: [u32; 5] = [3, 4, 3, 5, 3];
            const COLLIDE_WITH_UNQIUE: [u32; 3] = [3, 4, 1];
            const UNIQUE_VALUES: [u32; 2] = [1, 2];
            const SELF_COLLISION: ValueCollision<u32> = ValueCollision { value: 3 };
            const SET_COLLISION: ValueCollision<u32> = ValueCollision { value: 1 };

            #[test]
            fn try_collect_value_collision() {
                let err = <$set_type>::try_from_iter(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(err, SELF_COLLISION, "should match err");
            }

            #[test]
            fn try_collect_no_collision() {
                let set = <$set_type>::try_from_iter(UNIQUE_VALUES).expect("should be ok");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "should match data");
            }

            #[test]
            fn try_extend_collision_with_map() {
                let mut set = <$set_type>::from(UNIQUE_VALUES);

                let err = set.try_extend_safe(COLLIDE_WITH_UNQIUE).expect_err("should be err");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "set should be unchanged");
                assert_eq!(err, SET_COLLISION, "should match err");
            }

            #[test]
            fn try_extend_safe_collision_within_iter() {
                let mut set = <$set_type>::from(UNIQUE_VALUES);

                let err = set.try_extend_safe(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "set should be unchanged");
                assert_eq!(err, SELF_COLLISION, "err should have value");
            }

            #[test]
            fn try_extend_safe_no_collision() {
                let mut set = <$set_type>::new();

                set.try_extend_safe(UNIQUE_VALUES).expect("should be ok");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "should match data");
            }

            #[test]
            fn try_extend_unsafe_collision_with_map() {
                let mut set = <$set_type>::from(UNIQUE_VALUES);

                let err = set.try_extend(COLLIDE_WITH_UNQIUE).expect_err("should be err");

                assert_eq!(err, SET_COLLISION, "should match err");
            }

            #[test]
            fn try_extend_unsafe_collision_within_iter() {
                let mut set = <$set_type>::new();

                let err = set.try_extend(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(err, SELF_COLLISION, "should match err");
            }

            #[test]
            fn try_extend_unsafe_no_collision() {
                let mut set = <$set_type>::new();

                set.try_extend(UNIQUE_VALUES).expect("should be ok");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "should match data");
            }

            #[test]
            fn try_extend_preserves_original_value() {
                let mut set = <$set_type>::new();
                let v1 = Identifiable { value: 1, id: 1 };
                let v2 = Identifiable { value: 1, id: 2 };

                set.try_extend(std::iter::once(v1.clone())).expect("should be ok");

                let err = set.try_extend(std::iter::once(v2)).expect_err("should be err");

                assert_eq!(err, ValueCollision { value: v1.clone() }, "should return collision with original value");

                // Verify the set still contains the original value (id: 1)
                let stored = set.iter().next().unwrap();
                assert_eq!(stored.id, 1, "Set should contain the original value");
            }
        }
    };
}

try_from_iter_and_extend_iter!(hash_set, HashSet<_>);
try_from_iter_and_extend_iter!(btree_set, BTreeSet<_>);
try_from_iter_and_extend_iter!(hashbrown_set, HashBrownSet<_>);
try_from_iter_and_extend_iter!(index_set, indexmap::IndexSet<_>);
