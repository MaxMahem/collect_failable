use collect_failable::{OneOf2, TryUnzip, ValueCollision};
use std::collections::HashSet;

const VALID_DATA: [(u32, u32); 2] = [(1, 2), (2, 3)];
const VALID_SET_A: [u32; 2] = [1, 2];
const VALID_SET_B: [u32; 2] = [2, 3];

const INVALID_DATA_A: [(u32, u32); 2] = [(1, 3), (1, 4)];
const INVALID_DATA_B: [(u32, u32); 2] = [(1, 2), (2, 2)];
const ERROR_A: OneOf2<ValueCollision<u32>, ValueCollision<u32>> = OneOf2::A(ValueCollision { value: 1 });
const ERROR_B: OneOf2<ValueCollision<u32>, ValueCollision<u32>> = OneOf2::B(ValueCollision { value: 2 });

type HashSetTuple<T> = (HashSet<T>, HashSet<T>);

#[test]
fn try_unzip_success() {
    let (a, b): HashSetTuple<_> = VALID_DATA.into_iter().try_unzip().expect("Should be ok");
    assert_eq!(a, HashSet::from(VALID_SET_A));
    assert_eq!(b, HashSet::from(VALID_SET_B));
}

#[test]
fn try_unzip_fail_a() {
    let err = INVALID_DATA_A.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should be Err");
    assert_eq!(err, ERROR_A);
}

#[test]
fn try_unzip_fail_b() {
    let err = INVALID_DATA_B.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should be Err");
    assert_eq!(err, ERROR_B);
}
