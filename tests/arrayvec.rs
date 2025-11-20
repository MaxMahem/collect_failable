mod utils;
use utils::HideSize;

use arrayvec::{ArrayVec, CapacityError};
use collect_failable::{ExceedsCapacity, TryExtend, TryFromIterator};

type TryFromArray<T> = ArrayVec<T, 2>;
type ExtendArray<T> = ArrayVec<T, 4>;

const TOO_LONG_ARRAY: [u32; 3] = [1, 2, 3];
const VALID_ARRAY: [u32; 2] = [1, 2];
const EXTENDED_ARRAY: [u32; 4] = [1, 2, 1, 2];

const TRY_FROM_ERR: ExceedsCapacity = ExceedsCapacity { capacity: 2, necessary: 3 };
const EXTEND_ERR: ExceedsCapacity = ExceedsCapacity { capacity: 4, necessary: 5 };

#[test]
fn capcity_error_from_exceeds_capacity() {
    let err = CapacityError::<()>::from(ExceedsCapacity::new(2, 3));
    assert_eq!(err, CapacityError::new(()));
}

#[test]
fn try_from_iter_valid_array() {
    let array: TryFromArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");
    assert_eq!(*array, VALID_ARRAY, "Should match data");
}

#[test]
fn try_from_iter_too_long_data_early_return() {
    let err = ArrayVec::<_, 2>::try_from_iter(TOO_LONG_ARRAY).expect_err("Should be err");
    assert_eq!(err, TRY_FROM_ERR, "Should match err");
}

#[test]
fn try_from_iter_too_long_data_rollback() {
    let iter = HideSize(TOO_LONG_ARRAY.into_iter());
    let err = ArrayVec::<_, 2>::try_from_iter(iter).expect_err("Should be err");
    assert_eq!(err, TRY_FROM_ERR, "Should match err");
}

#[test]
fn try_extend_safe_valid() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    array.try_extend_safe(VALID_ARRAY).expect("Should be ok");
    assert_eq!(*array, EXTENDED_ARRAY, "Should match data");
}

#[test]
fn try_extend_safe_early_return() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    let err = array.try_extend_safe(TOO_LONG_ARRAY).expect_err("Should fail early");
    assert_eq!(err, EXTEND_ERR);
    assert_eq!(*array, VALID_ARRAY, "Should be unchanged");
}

#[test]
fn try_extend_safe_rollback() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    let iter = HideSize(TOO_LONG_ARRAY.into_iter());
    let err = array.try_extend_safe(iter).expect_err("Should rollback");

    assert_eq!(err, EXTEND_ERR);
    assert_eq!(*array, VALID_ARRAY, "Should be unchanged");
}

#[test]
fn try_extend_valid() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    array.try_extend(VALID_ARRAY).expect("Should be ok");
    assert_eq!(*array, EXTENDED_ARRAY, "Should match data");
}

#[test]
fn try_extend_early_return() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    let err = array.try_extend(TOO_LONG_ARRAY).expect_err("Should fail early");
    assert_eq!(err, EXTEND_ERR);
}

#[test]
fn try_extend_push_fail() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    let iter = HideSize(TOO_LONG_ARRAY.into_iter());
    let err = array.try_extend(iter).expect_err("Should rollback");

    assert_eq!(err, EXTEND_ERR);
}
