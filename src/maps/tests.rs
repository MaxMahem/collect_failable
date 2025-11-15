use std::collections::{BTreeMap, HashMap};

use hashbrown::HashMap as HashBrownMap;

use crate::TryExtend;
use crate::TryFromIterator;

macro_rules! test_try_from_iter_and_extend_iter {
    ($module:ident, $map_type:ty) => {
        mod $module {
            use super::*;

            #[test]
            fn try_collect_key_collision() {
                let result = <$map_type>::try_from_iter([(1, 2), (1, 3)]);

                assert_eq!(result.expect_err("should be err").key, 1);
            }

            #[test]
            fn try_collect_no_collision() {
                let result = <$map_type>::try_from_iter([(1, 2), (2, 3)]);

                let map = result.expect("should be ok");
                assert_eq!(map.len(), 2);
                assert_eq!(map.get(&1), Some(&2));
                assert_eq!(map.get(&2), Some(&3));
            }

            #[test]
            fn try_extend_collision_with_map() {
                let mut map = <$map_type>::from([(1, 2)]);

                let err = map.try_extend([(1, 3)]).expect_err("should be err");

                assert_eq!(map.len(), 1);
                assert_eq!(map.get(&1), Some(&2));
                assert_eq!(err.key, 1);
            }

            #[test]
            fn try_extend_collision_within_iter() {
                let mut map = <$map_type>::new();

                let err = map.try_extend([(1, 2), (1, 3)]).expect_err("should be err");

                assert_eq!(map.len(), 0);
                assert_eq!(err.key, 1);
            }

            #[test]
            fn try_extend_no_collision() {
                let mut map = <$map_type>::new();

                map.try_extend([(1, 2), (2, 3)]).expect("should be ok");

                assert_eq!(map.len(), 2);
                assert_eq!(map.get(&1), Some(&2));
                assert_eq!(map.get(&2), Some(&3));
            }
        }
    };
}

test_try_from_iter_and_extend_iter!(hash_map, HashMap<_, _>);
test_try_from_iter_and_extend_iter!(btree_map, BTreeMap<_, _>);
test_try_from_iter_and_extend_iter!(hashbrown_map, HashBrownMap<_, _>);
test_try_from_iter_and_extend_iter!(index_map, indexmap::IndexMap<_, _>);
