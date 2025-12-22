use super::collection_tests::{try_extend, try_extend_one, try_extend_safe};

use std::collections::{BTreeSet, HashSet};

use hashbrown::HashSet as HashBrownSet;

use crate::utils::Identifiable;
use collect_failable::errors::ItemCollision;
use collect_failable::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

macro_rules! try_from_iter_and_extend_iter {
    ($module:ident, $set_type:ty) => {
        mod $module {
            use super::*;

            const COLLIDE_WITH_SELF: [u32; 5] = [3, 4, 3, 5, 3];
            const COLLIDE_WITH_UNIQUE: [u32; 3] = [3, 4, 1];
            const UNIQUE_VALUES: [u32; 2] = [1, 2];

            #[test]
            fn try_collect_value_collision() {
                let err = <$set_type>::try_from_iter(COLLIDE_WITH_SELF).expect_err("should be err");

                let parts = err.into_data();

                let expected_collected = <$set_type>::from([3, 4]);
                assert_eq!(parts.collected, expected_collected, "collected should have items before collision");

                assert_eq!(parts.item, 3, "colliding value should be 3");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining, vec![5, 3], "remaining iterator should have 2 items");
            }

            #[test]
            fn try_collect_no_collision() {
                let set = <$set_type>::try_from_iter(UNIQUE_VALUES).expect("should be ok");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "should match data");
            }

            #[test]
            fn try_extend_safe_collision_with_set() {
                let mut set = <$set_type>::from(UNIQUE_VALUES);

                let err = set.try_extend_safe(COLLIDE_WITH_UNIQUE).expect_err("should be err");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "set should be unchanged");

                let parts = err.into_data();

                // try_extend_safe doesn't add to collected on collision
                assert_eq!(parts.collected.len(), 2, "collected should have 2 items before collision");
                assert!(parts.collected.contains(&3), "collected should have 3");
                assert!(parts.collected.contains(&4), "collected should have 4");

                assert_eq!(parts.item, 1, "colliding value should be 1");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining.len(), 0, "remaining should be empty");
            }

            #[test]
            fn try_extend_safe_collision_within_iter() {
                let mut set = <$set_type>::from(UNIQUE_VALUES);

                let err = set.try_extend_safe(COLLIDE_WITH_SELF).expect_err("should be err");

                assert_eq!(set, <$set_type>::from(UNIQUE_VALUES), "set should be unchanged");

                let parts = err.into_data();

                // Should have collected items before collision
                assert_eq!(parts.collected.len(), 2, "collected should have 2 items before collision");
                assert!(parts.collected.contains(&3), "collected should have 3");
                assert!(parts.collected.contains(&4), "collected should have 4");

                assert_eq!(parts.item, 3, "colliding value should be 3");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining, vec![5, 3], "remaining should have 2 items");
            }

            try_extend_safe!(
                try_extend_safe_no_collision,
                <$set_type>::new(),
                UNIQUE_VALUES,
                Ok(<$set_type>::from(UNIQUE_VALUES))
            );

            #[test]
            fn try_extend_collision_with_set() {
                let mut set = <$set_type>::from(UNIQUE_VALUES);

                let err = set.try_extend(COLLIDE_WITH_UNIQUE).expect_err("should be err");

                // Set should have the items from COLLIDE_WITH_UNIQUE added before collision
                assert_eq!(set.len(), 4, "set should have original 2 items plus 2 added before collision");
                assert!(set.contains(&3), "set should have 3 from successful insert");
                assert!(set.contains(&4), "set should have 4 from successful insert");

                let parts = err.into_data();

                // try_extend doesn't collect items in the error
                assert_eq!(parts.collected.len(), 0, "collected should be empty");

                assert_eq!(parts.item, 1, "colliding value should be 1");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining.len(), 0, "remaining should be empty");
            }

            #[test]
            fn try_extend_collision_within_iter() {
                let mut set = <$set_type>::new();

                let err = set.try_extend(COLLIDE_WITH_SELF).expect_err("should be err");

                // Set should have items before collision
                assert_eq!(set.len(), 2, "set should have 2 items before collision");
                assert!(set.contains(&3), "set should have 3");
                assert!(set.contains(&4), "set should have 4");

                let parts = err.into_data();

                // try_extend doesn't collect items in the error
                assert_eq!(parts.collected.len(), 0, "collected should be empty");

                assert_eq!(parts.item, 3, "colliding value should be 3");

                let remaining: Vec<_> = parts.iterator.collect();
                assert_eq!(remaining, vec![5, 3], "remaining should have 2 items");
            }

            try_extend!(try_extend_no_collision, <$set_type>::new(), UNIQUE_VALUES, Ok(<$set_type>::from(UNIQUE_VALUES)));

            #[test]
            fn try_extend_preserves_original_value() {
                let mut set = <$set_type>::new();
                let v1 = Identifiable { value: 1, id: 1 };
                let v2 = Identifiable { value: 1, id: 2 };

                set.try_extend(std::iter::once(v1.clone())).expect("should be ok");

                let err = set.try_extend(std::iter::once(v2)).expect_err("should be err");

                let parts = err.into_data();
                assert_eq!(parts.item.value, 1, "should return collision with value 1");
                assert_eq!(parts.item.id, 2, "colliding item should have id 2");

                // Verify the set still contains the original value (id: 1)
                let stored = set.iter().next().unwrap();
                assert_eq!(stored.id, 1, "Set should contain the original value");
            }

            mod try_extend_one {
                use super::*;

                try_extend_one!(valid, <$set_type>::new(), 1, Ok(<$set_type>::from([1])));
                try_extend_one!(collision, <$set_type>::from([1, 2, 3]), 2, Err(ItemCollision::new(2)));
            }
        }
    };
}

try_from_iter_and_extend_iter!(hash_set, HashSet<_>);
try_from_iter_and_extend_iter!(btree_set, BTreeSet<_>);
try_from_iter_and_extend_iter!(hashbrown_set, HashBrownSet<_>);
try_from_iter_and_extend_iter!(index_set, indexmap::IndexSet<_>);
