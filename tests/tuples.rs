use std::collections::HashSet;

use collect_failable::{OneOf2, TryExtend, TryFromIterator, ValueCollision};

type HashSetTuple<T> = (HashSet<T>, HashSet<T>);

const VALID_DATA: [(u32, u32); 2] = [(1, 2), (2, 3)];
const INVALID_DATA_LEFT: [(u32, u32); 2] = [(1, 3), (1, 4)];
const INVALID_DATA_RIGHT: [(u32, u32); 2] = [(4, 2), (5, 2)];
const LEFT_COLLISION: OneOf2<ValueCollision<u32>, ValueCollision<u32>> = OneOf2::A(ValueCollision { value: 1 });
const RIGHT_COLLISION: OneOf2<ValueCollision<u32>, ValueCollision<u32>> = OneOf2::B(ValueCollision { value: 2 });

macro_rules! test_tuple_impl {
    ($name:ident, $data:expr, $expected_error:expr) => {
        #[test]
        fn $name() {
            let err = HashSetTuple::try_from_iter($data).expect_err("should be err");
            assert_eq!(err, $expected_error, "should match err");
        }
    };
}

macro_rules! test_try_extend_collision {
    ($name:ident, $data:expr, $expected_error:expr) => {
        #[test]
        fn $name() {
            let mut valid = HashSetTuple::from_iter(VALID_DATA);
            
            let err = valid.try_extend($data).expect_err("should be err");
            
            assert_eq!(err, $expected_error, "should match err");
        }
    };
}

macro_rules! test_try_extend_success {
    ($name:ident, $method:ident) => {
        #[test]
        fn $name() {
            let mut empty = HashSetTuple::default();
            empty.$method(VALID_DATA).expect("should be ok");
            
            assert_eq!(empty, HashSetTuple::from_iter(VALID_DATA), "should match data");
        }
    };
}

// TryFromIterator tests
#[test]
fn try_from_iter_valid_data() {
    let found = HashSetTuple::try_from_iter(VALID_DATA).expect("should be ok");
    let expected: HashSetTuple<_> = VALID_DATA.into_iter().collect();
    
    assert_eq!(found, expected, "should match data");
}

test_tuple_impl!(try_from_iter_collision_left, INVALID_DATA_LEFT, LEFT_COLLISION);
test_tuple_impl!(try_from_iter_collision_right, INVALID_DATA_RIGHT, RIGHT_COLLISION);

// try_extend tests
test_try_extend_success!(try_extend_valid_data, try_extend);
test_try_extend_collision!(try_extend_collision_left, INVALID_DATA_LEFT, LEFT_COLLISION);
test_try_extend_collision!(try_extend_collision_right, INVALID_DATA_RIGHT, RIGHT_COLLISION);
