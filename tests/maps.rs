use std::collections::{BTreeMap, HashMap};

use hashbrown::HashMap as HashBrownMap;

use collect_failable::{KeyCollision, TryExtend, TryExtendSafe, TryFromIterator};

const UNIQUE_KEYS: [(i32, i32); 2] = [(1, 2), (2, 3)];
const COLLIDE_WITH_SELF: [(i32, i32); 3] = [(3, 3), (4, 4), (3, 5)];
const COLLIDE_WITH_REMAINING: [(i32, i32); 5] = [(1, 2), (2, 3), (1, 4), (3, 5), (4, 6)];
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
                let err = <$map_type>::try_from_iter(COLLIDE_WITH_REMAINING).expect_err("should be err");
                assert_eq!(err.len(), 5, "should have 5 items");

                let parts = err.into_parts();
                
                let expected_collected = <$map_type>::from([(1, 2), (2, 3)]);
                assert_eq!(parts.collected, expected_collected, "collected should have items before collision");

                assert_eq!(parts.item.0, 1, "colliding key should be 1");
                assert_eq!(parts.item.1, 4, "colliding value should be 4");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining, vec![(3, 5), (4, 6)], "remaining iterator should have 2 items");
            }

            #[test]
            fn try_collect_key_collision_into_iter() {
                let err = <$map_type>::try_from_iter(COLLIDE_WITH_REMAINING).expect_err("should be err");

                let all_items: Vec<_> = err.into_iter().collect();
                
                assert_eq!(all_items.len(), 5, "should have all 5 original items");
                assert!(all_items.contains(&(1, 2)), "should contain (1, 2)");
                assert!(all_items.contains(&(2, 3)), "should contain (2, 3)");
                assert!(all_items.contains(&(1, 4)), "should contain colliding (1, 4)");
                assert!(all_items.contains(&(3, 5)), "should contain remaining (3, 5)");
                assert!(all_items.contains(&(4, 6)), "should contain remaining (4, 6)");
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
                assert_eq!(map.get(&SELF_COLLISION.key), Some(&SELF_COLLIDE_VALUE), "value should not be added");
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
