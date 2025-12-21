use std::collections::HashSet;

use collect_failable::TryExtend;

type HashSetTuple<T> = (HashSet<T>, HashSet<T>);

const VALID_DATA: [(u32, u32); 2] = [(1, 2), (2, 3)];
const INVALID_DATA_LEFT: [(u32, u32); 2] = [(1, 3), (1, 4)];
const INVALID_DATA_RIGHT: [(u32, u32); 2] = [(4, 2), (5, 2)];

macro_rules! test_try_extend_collision {
    ($name:ident, $data:expr, $expected_value:expr, $side:ident) => {
        #[test]
        fn $name() {
            let mut valid = HashSetTuple::from_iter(VALID_DATA);

            let err = valid.try_extend($data).expect_err("should be err");

            match err {
                collect_failable::TupleExtensionError::A(side) if stringify!($side) == "A" => {
                    assert_eq!(side.error.item, $expected_value, "left collision value should match");
                }
                collect_failable::TupleExtensionError::B(side) if stringify!($side) == "B" => {
                    assert_eq!(side.error.item, $expected_value, "right collision value should match");
                }
                _ => panic!("Error on wrong side"),
            }
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

// try_extend tests
test_try_extend_success!(try_extend_valid_data, try_extend);
test_try_extend_collision!(try_extend_collision_left, INVALID_DATA_LEFT, 1, A);
test_try_extend_collision!(try_extend_collision_right, INVALID_DATA_RIGHT, 2, B);
