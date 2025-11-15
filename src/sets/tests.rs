use std::collections::{BTreeSet, HashSet};

use hashbrown::HashSet as HashBrownSet;

use crate::TryExtend;
use crate::TryFromIterator;

macro_rules! try_from_iter_and_extend_iter {
    ($module:ident, $map_type:ty) => {
        mod $module {
            use super::*;

            #[test]
            fn try_collect_value_collision() {
                let result = <$map_type>::try_from_iter([1, 1]);

                assert_eq!(result.expect_err("should be err").value, 1);
            }

            #[test]
            fn try_collect_no_collision() {
                let result = <$map_type>::try_from_iter([1, 2]);

                let set = result.expect("should be ok");
                assert_eq!(set.len(), 2);
                assert!(set.contains(&1));
                assert!(set.contains(&2));
            }

            #[test]
            fn try_extend_collision_with_map() {
                let mut set = <$map_type>::from([1]);

                let err = set.try_extend([1]).expect_err("should be err");

                assert_eq!(set.len(), 1);
                assert!(set.contains(&1));
                assert_eq!(err.value, 1);
            }

            #[test]
            fn try_extend_collision_within_iter() {
                let mut set = <$map_type>::new();

                let err = set.try_extend([1, 1]).expect_err("should be err");

                assert_eq!(set.len(), 0);
                assert_eq!(err.value, 1);
            }

            #[test]
            fn try_extend_no_collision() {
                let mut set = <$map_type>::new();

                set.try_extend([1, 2]).expect("should be ok");

                assert_eq!(set.len(), 2);
                assert!(set.contains(&1));
                assert!(set.contains(&2));
            }
        }
    };
}

try_from_iter_and_extend_iter!(hash_set, HashSet<_>);
try_from_iter_and_extend_iter!(btree_set, BTreeSet<_>);
try_from_iter_and_extend_iter!(hashbrown_set, HashBrownSet<_>);
try_from_iter_and_extend_iter!(index_set, indexmap::IndexSet<_>);
