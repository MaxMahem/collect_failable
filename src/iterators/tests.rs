use std::collections::{BTreeMap, HashMap};

use hashbrown::HashMap as HashBrownMap;

use crate::TryFromIterator;

macro_rules! test_try_from_iter {
    ($module:ident, $map_type:ty) => {
        mod $module {
            use super::*;

            #[test]
            fn key_collision() {
                let result = <$map_type>::try_from_iter([(1, 2), (1, 3)]);
                assert!(result.is_err());
                assert_eq!(result.unwrap_err().key, 1);
            }

            #[test]
            fn no_collision() {
                let result = <$map_type>::try_from_iter([(1, 2), (2, 3)]);
                assert!(result.is_ok());
                let map = result.unwrap();
                assert_eq!(map.len(), 2);
            }
        }
    };
}

test_try_from_iter!(hash_map, HashMap<_, _>);
test_try_from_iter!(btree_map, BTreeMap<_, _>);
test_try_from_iter!(hashbrown_map, HashBrownMap<_, _>);
test_try_from_iter!(index_map, indexmap::IndexMap<_, _>);
