use crate::collection_tests::{try_extend, try_extend_one, try_extend_safe};

use std::collections::{BTreeMap, HashMap};

use assert_unordered::assert_eq_unordered;
use hashbrown::HashMap as HashBrownMap;

use collect_failable::errors::{Collision, ErrorItemProvider};
use collect_failable::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

macro_rules! test_try_from_iter_and_extend_iter {
    ($module:ident, $map_type:ty) => {
        mod $module {
            use super::*;
            use crate::collision_tests::{
                test_try_collect_collision, test_try_extend_collision, test_try_extend_safe_collision, CollisionData,
            };

            const COLLIDE_WITH_SELF: CollisionData<(i32, i32), 0, 5> =
                CollisionData { base: [], add: [(1, 2), (2, 3), (1, 4), (3, 5), (4, 6)], collide_pos: 2 };

            const NO_COLLISION: [(i32, i32); 2] = [(1, 2), (2, 3)];

            const COLLIDE_WITH_MAP: CollisionData<(i32, i32), 2, 3> =
                CollisionData { base: [(1, 2), (2, 3)], add: [(3, 3), (1, 2), (5, 5)], collide_pos: 1 };

            test_try_collect_collision!(try_collect_key_collision, $map_type, COLLIDE_WITH_SELF);

            #[test]
            fn try_collect_no_collision() {
                let map = <$map_type>::try_from_iter(NO_COLLISION).expect("should succeed");

                assert_eq!(map, <$map_type>::from(NO_COLLISION), "should match");
            }

            test_try_extend_safe_collision!(try_extend_safe_collision_with_map, $map_type, COLLIDE_WITH_MAP);

            test_try_extend_safe_collision!(try_extend_safe_collision_within_iter, $map_type, COLLIDE_WITH_SELF);

            try_extend_safe!(
                try_extend_safe_no_collision,
                <$map_type>::new(),
                [(1, 2), (2, 3)],
                Ok(<$map_type>::from([(1, 2), (2, 3)]))
            );

            test_try_extend_collision!(try_extend_collision_with_map, $map_type, COLLIDE_WITH_MAP);

            test_try_extend_collision!(try_extend_collision_within_iter, $map_type, COLLIDE_WITH_SELF);

            try_extend!(try_extend_no_collision, <$map_type>::new(), [(1, 2), (2, 3)], Ok(<$map_type>::from([(1, 2), (2, 3)])));

            mod try_extend_one {
                use super::*;

                try_extend_one!(valid, <$map_type>::new(), (1, 1), Ok(<$map_type>::from([(1, 1)])));
                try_extend_one!(collision, <$map_type>::from([(1, 1), (2, 2)]), (1, 2), Err(Collision::new((1, 2))));
            }
        }
    };
}

test_try_from_iter_and_extend_iter!(hash_map, HashMap<_, _>);
test_try_from_iter_and_extend_iter!(btree_map, BTreeMap<_, _>);
test_try_from_iter_and_extend_iter!(hashbrown_map, HashBrownMap<_, _>);
test_try_from_iter_and_extend_iter!(index_map, indexmap::IndexMap<_, _>);
