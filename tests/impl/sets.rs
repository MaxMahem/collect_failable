use crate::utils::Identifiable;
use collect_failable::TryExtend;
use std::collections::{BTreeSet, HashSet};

use hashbrown::HashSet as HashBrownSet;

macro_rules! test_sets_preserve_original_value {
    ($module:ident, $set_type:ty) => {
        mod $module {
            use super::*;

            #[test]
            fn try_extend_preserves_original_value() {
                let mut set = <$set_type>::new();
                let v1 = Identifiable { value: 1, id: 1 };
                let v2 = Identifiable { value: 1, id: 2 };

                set.try_extend(std::iter::once(v1.clone())).expect("should be ok");

                let err = set.try_extend(std::iter::once(v2)).expect_err("should be err");

                let parts = err.into_data();
                assert_eq!(parts.error.item.value, 1, "should return collision with value 1");
                assert_eq!(parts.error.item.id, 2, "colliding item should have id 2");

                // Verify the set still contains the original value (id: 1)
                let stored = set.iter().next().unwrap();
                assert_eq!(stored.id, 1, "Set should contain the original value");
            }
        }
    };
}

test_sets_preserve_original_value!(hash_set, HashSet<_>);
test_sets_preserve_original_value!(btree_set, BTreeSet<_>);
test_sets_preserve_original_value!(hashbrown_set, HashBrownSet<_>);
test_sets_preserve_original_value!(index_set, indexmap::IndexSet<_>);
