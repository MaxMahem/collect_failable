use collect_failable::{OneOf2, TryExtend, TryFromIterator, ValueCollision};
use std::collections::HashSet;

const VALID_DATA: [(u32, u32); 2] = [(1, 2), (2, 3)];
const INVALID_DATA: [(u32, u32); 2] = [(1, 3), (1, 4)];
const ERROR: OneOf2<ValueCollision<u32>, ValueCollision<u32>> = OneOf2::A(ValueCollision { value: 1 });

type HashSetTuple<T> = (HashSet<T>, HashSet<T>);

#[test]
fn try_from_iter_valid_data() {
    let found = HashSetTuple::try_from_iter(VALID_DATA).expect("should be ok");
    let expected: HashSetTuple<_> = VALID_DATA.into_iter().collect();

    assert_eq!(found, expected, "should match data");
}

#[test]
fn try_from_iter_invalid_data() {
    let err = HashSetTuple::try_from_iter(INVALID_DATA).expect_err("should be err");
    assert_eq!(err, ERROR, "should match err");
}

#[test]
fn try_extend_safe_collision_with_data() {
    let mut valid = HashSetTuple::from_iter(VALID_DATA);
    let err = valid.try_extend_safe(VALID_DATA).expect_err("should be err");

    assert_eq!(valid, HashSetTuple::from_iter(VALID_DATA), "should be unchanged");
    assert_eq!(err, ERROR, "should match err");
}

#[test]
fn try_extend_collision_within_iter() {
    let mut empty = HashSetTuple::default();
    let err = empty.try_extend(INVALID_DATA).expect_err("should be err");

    assert_eq!(err, ERROR, "should match err");
}

#[test]
fn try_extend_valid_data() {
    let mut empty = HashSetTuple::default();
    empty.try_extend(VALID_DATA).expect("should be ok");

    assert_eq!(empty, HashSetTuple::from_iter(VALID_DATA), "should match data");
}
