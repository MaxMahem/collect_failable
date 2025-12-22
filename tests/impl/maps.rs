use super::collection_tests::{try_extend, try_extend_one, try_extend_safe};

use std::collections::{BTreeMap, HashMap};

use hashbrown::HashMap as HashBrownMap;

use collect_failable::errors::ItemCollision;
use collect_failable::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

const UNIQUE_KEYS: [(i32, i32); 2] = [(1, 2), (2, 3)];
const COLLIDE_WITH_SELF: [(i32, i32); 3] = [(3, 3), (4, 4), (3, 5)];
const COLLIDE_WITH_REMAINING: [(i32, i32); 5] = [(1, 2), (2, 3), (1, 4), (3, 5), (4, 6)];
const COLLIDE_WITH_MAP: [(i32, i32); 2] = [(3, 3), (1, 2)];

macro_rules! test_try_from_iter_and_extend_iter {
    ($module:ident, $map_type:ty) => {
        mod $module {
            use super::*;

            #[test]
            fn try_collect_key_collision() {
                let err = <$map_type>::try_from_iter(COLLIDE_WITH_REMAINING).expect_err("should be err");

                let parts = err.into_data();

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

                let parts = err.into_data();

                // try_extend_safe doesn't add to collected on collision
                assert_eq!(parts.collected.len(), 1, "collected should have 1 item before collision");
                assert_eq!(parts.collected.get(&3), Some(&3), "collected should have (3, 3)");

                assert_eq!(parts.item.0, 1, "colliding key should be 1");
                assert_eq!(parts.item.1, 2, "colliding value should be 2");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining.len(), 0, "remaining should be empty");
            }

            #[test]
            fn try_extend_safe_collision_within_iter() {
                let mut map = <$map_type>::from(UNIQUE_KEYS);

                let err = map.try_extend_safe(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(map, <$map_type>::from(UNIQUE_KEYS), "map should be unchanged");

                let parts = err.into_data();

                // Should have collected items before collision (both (3,3) and (4,4))
                assert_eq!(parts.collected.len(), 2, "collected should have 2 items before collision");
                assert_eq!(parts.collected.get(&3), Some(&3), "collected should have (3, 3)");
                assert_eq!(parts.collected.get(&4), Some(&4), "collected should have (4, 4)");

                assert_eq!(parts.item.0, 3, "colliding key should be 3");
                assert_eq!(parts.item.1, 5, "colliding value should be 5");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining.len(), 0, "remaining should be empty");
            }

            try_extend_safe!(try_extend_safe_no_collision, <$map_type>::new(), UNIQUE_KEYS, Ok(<$map_type>::from(UNIQUE_KEYS)));

            #[test]
            fn try_extend_collision_with_map() {
                let mut map = <$map_type>::from(UNIQUE_KEYS);

                let err = map.try_extend(COLLIDE_WITH_MAP).expect_err("should be err");

                // Map should have the first item from COLLIDE_WITH_MAP since try_extend has basic guarantee
                assert_eq!(map.len(), 3, "map should have original 2 items plus 1 added before collision");
                assert_eq!(map.get(&3), Some(&3), "map should have (3, 3) from successful insert");

                let parts = err.into_data();

                // try_extend doesn't collect items in the error
                assert_eq!(parts.collected.len(), 0, "collected should be empty");

                assert_eq!(parts.item.0, 1, "colliding key should be 1");
                assert_eq!(parts.item.1, 2, "colliding value should be 2");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining.len(), 0, "remaining should be empty");
            }

            #[test]
            fn try_extend_collision_within_iter() {
                let mut map = <$map_type>::new();

                let err = map.try_extend(COLLIDE_WITH_SELF).expect_err("should be err");

                // Map should have items before collision
                assert_eq!(map.len(), 2, "map should have 2 items before collision");
                assert_eq!(map.get(&3), Some(&3), "map should have (3, 3)");
                assert_eq!(map.get(&4), Some(&4), "map should have (4, 4)");

                let parts = err.into_data();

                // try_extend doesn't collect items in the error
                assert_eq!(parts.collected.len(), 0, "collected should be empty");

                assert_eq!(parts.item.0, 3, "colliding key should be 3");
                assert_eq!(parts.item.1, 5, "colliding value should be 5");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining.len(), 0, "remaining should be empty");
            }

            try_extend!(try_extend_no_collision, <$map_type>::new(), UNIQUE_KEYS, Ok(<$map_type>::from(UNIQUE_KEYS)));

            mod try_extend_one {
                use super::*;

                try_extend_one!(valid, <$map_type>::new(), (1, 1), Ok(<$map_type>::from([(1, 1)])));
                try_extend_one!(collision, <$map_type>::from([(1, 1), (2, 2)]), (1, 2), Err(ItemCollision::new((1, 2))));
            }
        }
    };
}

test_try_from_iter_and_extend_iter!(hash_map, HashMap<_, _>);
test_try_from_iter_and_extend_iter!(btree_map, BTreeMap<_, _>);
test_try_from_iter_and_extend_iter!(hashbrown_map, HashBrownMap<_, _>);
test_try_from_iter_and_extend_iter!(index_map, indexmap::IndexMap<_, _>);
