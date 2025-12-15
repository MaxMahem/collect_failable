use arrayvec::ArrayVec;
use collect_failable::{utils::FixedSizeHint, CapacityMismatch, TryExtend, TryExtendSafe, TryFromIterator};

type TryFromArray<T> = ArrayVec<T, 2>;
type ExtendArray<T> = ArrayVec<T, 4>;

const TOO_LONG_ARRAY: [u32; 3] = [1, 2, 3];
const VALID_ARRAY: [u32; 2] = [1, 2];
const EXTENDED_ARRAY: [u32; 4] = [1, 2, 1, 2];

// Bounds errors occur when size hints indicate overflow (early return)
const TRY_FROM_BOUNDS_ERR: CapacityMismatch = CapacityMismatch::bounds(2..=2, (3, Some(3)));
const EXTEND_BOUNDS_ERR: CapacityMismatch = CapacityMismatch::bounds(4..=4, (3, Some(3)));

// Overflow errors occur when actual collection fails (after hidden size hints)
const TRY_FROM_OVERFLOW_ERR: CapacityMismatch = CapacityMismatch::overflow(2..=2);
const EXTEND_OVERFLOW_ERR: CapacityMismatch = CapacityMismatch::overflow(4..=4);

#[test]
fn try_from_iter_valid_array() {
    let array: TryFromArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");
    assert_eq!(*array, VALID_ARRAY, "Should match data");
}

#[test]
fn try_from_iter_too_long_data_early_return() {
    let err = ArrayVec::<_, 2>::try_from_iter(TOO_LONG_ARRAY).expect_err("Should be err");
    assert_eq!(err.error, TRY_FROM_BOUNDS_ERR, "Should match err");
}

#[test]
fn try_from_iter_too_long_data_rollback() {
    let iter = FixedSizeHint::hide_size(TOO_LONG_ARRAY);
    let err = ArrayVec::<_, 2>::try_from_iter(iter).expect_err("Should be err");
    assert_eq!(err.error, TRY_FROM_OVERFLOW_ERR, "Should match err");
}

#[test]
fn try_from_iter_reconstruct() {
    let err = ArrayVec::<_, 2>::try_from_iter([1, 2, 3]).expect_err("Should be err");

    let reconstructed: Vec<_> = err.into_iter().collect();
    assert_eq!(reconstructed.len(), 3, "Should reconstruct as: rejected, collected, remaining");
    assert!(reconstructed.contains(&1));
    assert!(reconstructed.contains(&2));
    assert!(reconstructed.contains(&3));
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
    assert_eq!(err.error, EXTEND_BOUNDS_ERR);
    assert_eq!(*array, VALID_ARRAY, "Should be unchanged");
}

#[test]
fn try_extend_safe_rollback() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    let iter = FixedSizeHint::hide_size(TOO_LONG_ARRAY);
    let err = array.try_extend_safe(iter).expect_err("Should rollback");

    assert_eq!(err.error, EXTEND_OVERFLOW_ERR);
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
    assert_eq!(err.error, EXTEND_BOUNDS_ERR);
}

#[test]
fn try_extend_push_fail() {
    let mut array: ExtendArray<_> = ArrayVec::try_from_iter(VALID_ARRAY).expect("Should be ok");

    let iter = FixedSizeHint::hide_size(TOO_LONG_ARRAY);
    let err = array.try_extend(iter).expect_err("Should rollback");

    assert_eq!(err.error, EXTEND_OVERFLOW_ERR);
}
