use super::collection_tests::{try_extend, try_extend_one, try_extend_safe};

use std::collections::{BTreeSet, HashSet};

use assert_unordered::assert_eq_unordered;
use hashbrown::HashSet as HashBrownSet;

use crate::utils::Identifiable;
use collect_failable::errors::{Collision, ErrorItemProvider};
use collect_failable::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

macro_rules! try_from_iter_and_extend_iter {
    ($module:ident, $set_type:ty) => {
        mod $module {
            use super::*;
            use crate::collision_tests::{
                test_try_collect_collision, test_try_extend_collision, test_try_extend_safe_collision, CollisionData,
            };

            const COLLIDE_WITH_SELF: CollisionData<u32, 0, 5> = CollisionData { base: [], add: [3, 4, 3, 5, 3], collide_pos: 2 };
            const COLLIDE_WITH_UNIQUE: CollisionData<u32, 2, 3> = CollisionData { base: [1, 2], add: [3, 4, 1], collide_pos: 2 };
            const NO_COLLISION: [u32; 2] = [1, 2];

            test_try_collect_collision!(try_collect_value_collision, $set_type, COLLIDE_WITH_SELF);

            #[test]
            fn try_collect_no_collision() {
                let set = <$set_type>::try_from_iter(NO_COLLISION).expect("should be ok");

                assert_eq!(set, <$set_type>::from(NO_COLLISION), "should match data");
            }

            test_try_extend_safe_collision!(try_extend_safe_collision_with_set, $set_type, COLLIDE_WITH_UNIQUE);

            test_try_extend_safe_collision!(try_extend_safe_collision_within_iter, $set_type, COLLIDE_WITH_SELF);

            try_extend_safe!(try_extend_safe_no_collision, <$set_type>::new(), [1, 2], Ok(<$set_type>::from([1, 2])));

            test_try_extend_collision!(try_extend_collision_with_set, $set_type, COLLIDE_WITH_UNIQUE);

            test_try_extend_collision!(try_extend_collision_within_iter, $set_type, COLLIDE_WITH_SELF);

            #[test]
            fn try_extend_preserves_original_value() {
                let mut set = <$set_type>::new();
                let v1 = Identifiable { value: 1, id: 1 };
                let v2 = Identifiable { value: 1, id: 2 };

                set.try_extend(std::iter::once(v1.clone())).expect("should be ok");

                let err = set.try_extend(std::iter::once(v2)).expect_err("should be err");

                let parts = err.into_data();
                assert_eq!(parts.error.item().unwrap().value, 1, "should return collision with value 1");
                assert_eq!(parts.error.item().unwrap().id, 2, "colliding item should have id 2");

                // Verify the set still contains the original value (id: 1)
                let stored = set.iter().next().unwrap();
                assert_eq!(stored.id, 1, "Set should contain the original value");
            }

            try_extend!(try_extend_no_collision, <$set_type>::new(), [1, 2], Ok(<$set_type>::from([1, 2])));

            mod try_extend_one {
                use super::*;

                try_extend_one!(valid, <$set_type>::new(), 1, Ok(<$set_type>::from([1])));
                try_extend_one!(collision, <$set_type>::from([1, 2, 3]), 2, Err(Collision::new(2)));
            }
        }
    };
}

try_from_iter_and_extend_iter!(hash_set, HashSet<_>);
try_from_iter_and_extend_iter!(btree_set, BTreeSet<_>);
try_from_iter_and_extend_iter!(hashbrown_set, HashBrownSet<_>);
try_from_iter_and_extend_iter!(index_set, indexmap::IndexSet<_>);
