use crate::collection_tests::try_extend_one;
use assert_unordered::assert_eq_unordered;
use collect_failable::errors::{CollectionError, Collision, ErrorItemProvider};
use collect_failable::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};
use tap::Pipe;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Debug;

use hashbrown::{HashMap as HashBrownMap, HashSet as HashBrownSet};

#[derive(Debug, Clone, Copy)]
pub struct CollisionData<T, const N: usize> {
    pub add: [T; N],
    pub collide_pos: usize,
}

impl<T: Copy, const N: usize> CollisionData<T, N> {
    pub fn colliding_item(&self) -> Option<&T> {
        self.add.get(self.collide_pos)
    }

    pub fn colliding_error(&self) -> Collision<T> {
        self.colliding_item().copied().expect("collide_pos should be valid").pipe(Collision::new)
    }

    pub fn collected(&self) -> impl Iterator<Item = T> {
        self.add.into_iter().take(self.collide_pos)
    }

    pub fn remaining(&self) -> impl Iterator<Item = T> {
        self.add.into_iter().skip(self.collide_pos + 1)
    }
}

pub struct TestParams<T> {
    /// Values used for base population and valid tests.
    pub values: [T; 2],
    /// Data that can validly extend `values` without collision.
    pub valid_extension: [T; 2],
    /// Sequence containing an internal collision (e.g., duplicates within the sequence).
    pub self_collision: CollisionData<T, 5>,
    /// Sequence colliding with `values`.
    pub other_collision: CollisionData<T, 3>,
}

const MAP_PARAMS: TestParams<(i32, i32)> = TestParams {
    values: [(1, 2), (2, 3)],
    valid_extension: [(3, 3), (4, 4)],
    self_collision: CollisionData { add: [(1, 2), (2, 3), (1, 4), (3, 5), (4, 6)], collide_pos: 2 },
    other_collision: CollisionData { add: [(3, 3), (1, 2), (5, 5)], collide_pos: 1 },
};

const SET_PARAMS: TestParams<u32> = TestParams {
    values: [1, 2],
    valid_extension: [3, 4],
    self_collision: CollisionData { add: [3, 4, 3, 5, 3], collide_pos: 2 },
    other_collision: CollisionData { add: [3, 4, 1], collide_pos: 2 },
};

fn check_collision_error<T, C, const N: usize>(
    err: &CollectionError<std::array::IntoIter<T, N>, C, Collision<T>>,
    data: &CollisionData<T, N>,
    expected_collected: &C,
) where
    T: Copy + Debug + PartialEq,
    C: Debug + PartialEq,
{
    assert_eq!(&err.collected, expected_collected);
    assert_eq!(err.error, data.colliding_error());
    assert_eq!(err.error.item(), data.colliding_item());
    assert_eq!(err.iterator.clone().collect::<Vec<_>>(), data.remaining().collect::<Vec<_>>());
}

macro_rules! generate_collision_tests {
    ($module:ident, $type:ty, $params:expr) => {
        mod $module {
            use super::*;

            #[test]
            fn try_collect_collision() {
                let data = $params.self_collision;
                let err = <$type>::try_from_iter(data.add).expect_err("should collide");

                super::check_collision_error(&err, &data, &data.collected().collect::<$type>());

                assert_eq_unordered!(
                    err.into_iter().collect::<Vec<_>>(),
                    data.add.into_iter().collect::<Vec<_>>(),
                    "all added items should be recovered"
                );
            }

            #[test]
            fn try_collect_no_collision() {
                let collection = <$type>::try_from_iter($params.values).expect("should succeed");
                assert_eq!(collection, <$type>::from($params.values), "should match");
            }

            #[test]
            fn try_extend_safe_collision_with_collection() {
                let data = $params.other_collision;
                let mut collection = <$type>::from($params.values);

                let err = collection.try_extend_safe(data.add).expect_err("should collide");

                assert_eq!(collection, <$type>::from($params.values), "collection should be unchanged");

                super::check_collision_error(&err, &data, &data.collected().collect::<$type>());

                assert_eq_unordered!(
                    data.add.into_iter().collect::<Vec<_>>(),
                    err.into_iter().collect::<Vec<_>>(),
                    "all added items should be recovered"
                );
            }

            #[test]
            fn try_extend_safe_collision_within_iter() {
                let data = $params.self_collision;
                let mut collection = <$type>::new();

                let err = collection.try_extend_safe(data.add).expect_err("should collide");

                assert!(collection.is_empty(), "collection should be unchanged");

                super::check_collision_error(&err, &data, &data.collected().collect::<$type>());

                assert_eq_unordered!(
                    data.add.into_iter().collect::<Vec<_>>(),
                    err.into_iter().collect::<Vec<_>>(),
                    "all added items should be recovered"
                );
            }

            #[test]
            fn try_extend_safe_no_collision() {
                let mut collection = <$type>::new();

                collection.try_extend_safe($params.values).expect("should succeed");

                assert_eq!(collection, <$type>::from($params.values), "should match");
            }

            #[test]
            fn try_extend_safe_valid_extension() {
                let mut collection = <$type>::from($params.values);

                collection.try_extend_safe($params.valid_extension).expect("should succeed");

                let expected = $params.values.into_iter().chain($params.valid_extension).collect::<$type>();
                assert_eq!(collection, expected, "should match total");
            }

            #[test]
            fn try_extend_collision_with_collection() {
                let base = $params.values;
                let data = $params.other_collision;

                let mut collection = <$type>::from(base);

                let err = collection.try_extend(data.add).expect_err("should collide");

                // collection may be modified, so state is not checked

                super::check_collision_error(&err, &data, &Default::default());

                let err_content: Vec<_> = err.into_iter().collect();
                assert_eq_unordered!(
                    err_content.clone(),
                    std::iter::chain(data.remaining(), data.colliding_item().copied()).collect::<Vec<_>>(),
                    "iter should contain all iterated items (remaining + colliding)"
                );

                assert_eq_unordered!(
                    std::iter::chain(err_content, collection).collect::<Vec<_>>(),
                    std::iter::chain(base, data.add).collect::<Vec<_>>(),
                    "all items should be recovered (error content + partial collection = original total)"
                );
            }

            #[test]
            fn try_extend_collision_within_iter() {
                let data = $params.self_collision;
                let mut collection = <$type>::new();

                let err = collection.try_extend(data.add).expect_err("should collide");

                super::check_collision_error(&err, &data, &Default::default());

                let err_content: Vec<_> = err.into_iter().collect();
                assert_eq_unordered!(
                    err_content.clone(),
                    data.remaining().chain(data.colliding_item().copied()).collect::<Vec<_>>(),
                    "iter should contain all iterated items (remaining + colliding)"
                );

                assert_eq_unordered!(
                    std::iter::chain(err_content, collection).collect::<Vec<_>>(),
                    data.add.into_iter().collect::<Vec<_>>(),
                    "all items should be recovered"
                );
            }

            #[test]
            fn try_extend_no_collision() {
                let data = $params.values;
                let mut collection = <$type>::new();
                collection.try_extend(data).expect("should succeed");
                assert_eq!(collection, <$type>::from(data), "should match");
            }

            mod try_extend_one {
                use super::*;

                try_extend_one!(
                    valid,
                    <$type>::from($params.values),
                    $params.valid_extension[0],
                    Ok(<$type>::from_iter($params.values.into_iter().chain(std::iter::once($params.valid_extension[0]))))
                );

                try_extend_one!(
                    collision,
                    <$type>::from($params.values),
                    $params.other_collision.colliding_item().copied().unwrap(),
                    Err($params.other_collision.colliding_error())
                );
            }
        }
    };
}

generate_collision_tests!(maps_hash_map, HashMap<_, _>, MAP_PARAMS);
generate_collision_tests!(maps_btree_map, BTreeMap<_, _>, MAP_PARAMS);
generate_collision_tests!(maps_hashbrown_map, HashBrownMap<_, _>, MAP_PARAMS);
generate_collision_tests!(maps_index_map, indexmap::IndexMap<_, _>, MAP_PARAMS);

generate_collision_tests!(sets_hash_set, HashSet<_>, SET_PARAMS);
generate_collision_tests!(sets_btree_set, BTreeSet<_>, SET_PARAMS);
generate_collision_tests!(sets_hashbrown_set, HashBrownSet<_>, SET_PARAMS);
generate_collision_tests!(sets_index_set, indexmap::IndexSet<_>, SET_PARAMS);
