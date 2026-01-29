/// Generalized macro that generates complete test functions for collection creation
///
/// Supports both success and error cases for any collection type (arrays, ArrayVec, etc.):
/// - `collection_test!(name, Type, iter, Ok(expected))`
/// - `collection_test!(name, Type, iter, Err(expected_error))`
#[allow(unused_macros)]
macro_rules! try_collect {
    ($name:ident, $collection_type:ty, $iter:expr, Ok($expected:expr)) => {
        #[test]
        fn $name() {
            let found = <$collection_type>::try_from_iter($iter).expect("should be ok");
            assert_eq!(found, $expected, "should match expected value");
        }
    };

    ($name:ident, $collection_type:ty, $iter:expr, Err($expected_error:expr)) => {
        #[test]
        fn $name() {
            let err = <$collection_type>::try_from_iter($iter).expect_err("should be err");
            assert_eq!(err.error, $expected_error, "should match expected error");
        }
    };
}

/// Macro for try_extend_safe tests
///
/// Supports:
/// - `try_extend_safe!(name, initial_collection, extend_iter, Ok(expected))`
/// - `try_extend_safe!(name, initial_collection, extend_iter, Err(expected_error, expected_state))`
#[allow(unused_macros)]
macro_rules! try_extend_safe {
    ($name:ident, $initial:expr, $extend:expr, Ok($expected:expr)) => {
        #[test]
        fn $name() {
            let mut collection = $initial;
            collection.try_extend_safe($extend).expect("should extend successfully");
            assert_eq!(collection, $expected, "should match expected value");
        }
    };

    ($name:ident, $initial:expr, $extend:expr, Err($expected_error:expr, $expected_collected:expr, $expected_iterator:expr)) => {
        #[test]
        fn $name() {
            let mut collection = $initial;

            let err = collection.try_extend_safe($extend).expect_err("should fail to extend");
            let parts = err.into_data();

            assert_eq!(collection, $initial, "should be unchanged on error");
            assert_eq!(parts.error, $expected_error, "should match expected error");
            assert_eq!(parts.collected, $expected_collected, "should match expected collected");
            assert!(parts.iterator.eq($expected_iterator), "should match expected iterator");
        }
    };
}

/// Macro for try_extend tests
///
/// Supports:
/// - `try_extend!(name, initial_collection, extend_iter, Ok(expected))`
/// - `try_extend!(name, initial_collection, extend_iter, Err(expected_error))`
#[allow(unused_macros)]
macro_rules! try_extend {
    ($name:ident, $initial:expr, $extend:expr, Ok($expected:expr)) => {
        #[test]
        fn $name() {
            let mut collection = $initial;
            collection.try_extend($extend).expect("should extend successfully");
            assert_eq!(collection, $expected, "should match expected value");
        }
    };

    ($name:ident, $initial:expr, $extend:expr, Err($expected_error:expr, $expected_collected:expr, $expected_iterator:expr)) => {
        #[test]
        fn $name() {
            let mut collection = $initial;
            let err = collection.try_extend($extend).expect_err("should fail to extend");
            let parts = err.into_data();
            assert_eq!(parts.error, $expected_error, "should match expected error");
            assert_eq!(parts.collected, $expected_collected, "should match expected collected");
            assert!(parts.iterator.eq($expected_iterator), "should match expected iterator");
        }
    };
}

/// Macro for try_extend_one tests
///
/// Supports:
/// - `try_extend_one!(name, initial_collection, item, Ok(expected))`
/// - `try_extend_one!(name, initial_collection, item, Err(expected_item))`
#[allow(unused_macros)]
macro_rules! try_extend_one {
    ($name:ident, $initial:expr, $item:expr, Ok($expected:expr)) => {
        #[test]
        fn $name() {
            let mut collection = $initial;
            collection.try_extend_one($item).expect("should extend successfully");
            assert_eq!(collection, $expected, "should match expected value");
        }
    };

    ($name:ident, $initial:expr, $item:expr, Err($expected:expr)) => {
        #[test]
        fn $name() {
            let mut collection = $initial;
            let initial_clone = collection.clone();
            let err = collection.try_extend_one($item).expect_err("should fail to extend");
            assert_eq!(err, $expected, "should return rejected item");
            assert_eq!(collection, initial_clone, "should be unchanged on error");
        }
    };
}

/// Macro for testing reconstructing iterator data from error
///
/// Supports:
/// - `try_collect_recover_iter_data!(name, Type, iter, collected, data)`
#[allow(unused_macros)]
macro_rules! recover_iter_data {
    ($name:ident, $type:ty, $iter:expr, $collected:expr, $data:expr) => {
        #[test]
        fn $name() {
            let err = <$type>::try_from_iter($iter).expect_err("should fail");
            assert_eq!(err.collected, $collected, "should match collected items");
            let data: Vec<_> = err.into_iter().collect();
            assert_eq!(data, $data, "should match reconstructed items");
        }
    };
}

#[allow(unused_imports)]
pub(crate) use recover_iter_data;
#[allow(unused_imports)]
pub(crate) use try_collect;
#[allow(unused_imports)]
pub(crate) use try_extend;
#[allow(unused_imports)]
pub(crate) use try_extend_one;
#[allow(unused_imports)]
pub(crate) use try_extend_safe;
