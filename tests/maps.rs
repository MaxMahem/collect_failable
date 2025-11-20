use std::collections::{BTreeMap, HashMap};

use hashbrown::HashMap as HashBrownMap;

use collect_failable::KeyCollision;
use collect_failable::TryExtend;
use collect_failable::TryFromIterator;

const UNIQUE_KEYS: [(i32, i32); 2] = [(1, 2), (2, 3)];
const COLLIDE_WITH_SELF: [(i32, i32); 3] = [(3, 3), (4, 4), (3, 5)];
const COLLIDE_WITH_MAP: [(i32, i32); 2] = [(3, 3), (1, 2)];

const SELF_COLLISION: KeyCollision<i32> = KeyCollision { key: 3 };
const MAP_COLLISION: KeyCollision<i32> = KeyCollision { key: 1 };

const SELF_COLLIDE_VALUE: i32 = 3;

macro_rules! test_try_from_iter_and_extend_iter {
    ($module:ident, $map_type:ty) => {
        mod $module {
            use super::*;

            #[test]
            fn try_collect_key_collision() {
                let err = <$map_type>::try_from_iter(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(err, SELF_COLLISION, "should have err key");
            }

            #[test]
            fn try_collect_no_collision() {
                let map = <$map_type>::try_from_iter(UNIQUE_KEYS).expect("should be ok");

                assert_eq!(map, <$map_type>::from(UNIQUE_KEYS), "should match data");
            }

            #[test]
            fn try_extend_safe_collision_with_map() {
                let mut map = <$map_type>::from(UNIQUE_KEYS);

                let err = map.try_extend_safe(COLLIDE_WITH_MAP).expect_err("should be err");

                assert_eq!(map, <$map_type>::from(UNIQUE_KEYS), "map should be unchanged");
                assert_eq!(err, MAP_COLLISION, "err should have key value");
            }

            #[test]
            fn try_extend_safe_collision_within_iter() {
                let mut map = <$map_type>::from(UNIQUE_KEYS);

                let err = map.try_extend_safe(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(map, <$map_type>::from(UNIQUE_KEYS), "map should be unchanged");
                assert_eq!(err, SELF_COLLISION, "err should have key value");
            }

            #[test]
            fn try_extend_safe_no_collision() {
                let mut map = <$map_type>::new();

                map.try_extend_safe(UNIQUE_KEYS).expect("should be ok");

                assert_eq!(map, <$map_type>::from(UNIQUE_KEYS), "should match data");
            }

            #[test]
            fn try_extend_unsafe_collision_within_map() {
                let mut map = <$map_type>::from(UNIQUE_KEYS);

                let err = map.try_extend(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(err, SELF_COLLISION, "err should match");
                assert_eq!(
                    map.get(&SELF_COLLISION.key),
                    Some(&SELF_COLLIDE_VALUE),
                    "value should not be added"
                );
            }

            #[test]
            fn try_extend_unsafe_collision_within_iter() {
                let mut map = <$map_type>::new();

                let err = map.try_extend(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(err, SELF_COLLISION, "err should have key value");
            }

            #[test]
            fn try_extend_unsafe_no_collision() {
                let mut map = <$map_type>::new();

                map.try_extend(UNIQUE_KEYS).expect("should be ok");

                assert_eq!(map, <$map_type>::from(UNIQUE_KEYS), "should match data");
            }
        }
    };
}

test_try_from_iter_and_extend_iter!(hash_map, HashMap<_, _>);
test_try_from_iter_and_extend_iter!(btree_map, BTreeMap<_, _>);
test_try_from_iter_and_extend_iter!(hashbrown_map, HashBrownMap<_, _>);
test_try_from_iter_and_extend_iter!(index_map, indexmap::IndexMap<_, _>);
