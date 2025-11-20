mod utils;
use utils::HideSize;

use collect_failable::ItemCountMismatch;
use collect_failable::TryFromIterator;

type ExpectedArray<T> = [T; 2];

const TOO_LONG_ARRAY: [u32; 3] = [1, 2, 3];
const VALID_ARRAY: [u32; 2] = [1, 2];
const TOO_SHORT_ARRAY: [u32; 1] = [1];

const TOO_SHORT_ERR: ItemCountMismatch = ItemCountMismatch { expected: 2, actual: 1 };
const TOO_LONG_ERR: ItemCountMismatch = ItemCountMismatch { expected: 2, actual: 3 };

#[test]
fn try_from_iter_valid_array() {
    let found = ExpectedArray::try_from_iter(VALID_ARRAY).expect("should be ok");
    assert_eq!(found, VALID_ARRAY, "should match data");
}

#[test]
fn try_from_iter_too_long_data() {
    let err = ExpectedArray::try_from_iter(TOO_LONG_ARRAY).expect_err("should be err");
    assert_eq!(err, TOO_LONG_ERR, "should match err");
}

#[test]
fn try_from_iter_too_short_data() {
    let err = ExpectedArray::try_from_iter(TOO_SHORT_ARRAY).expect_err("should be err");
    assert_eq!(err, TOO_SHORT_ERR, "should match err");
}

#[test]
fn try_from_iter_too_long_data_rollback() {
    let iter = HideSize(TOO_LONG_ARRAY.into_iter());
    let err = ExpectedArray::try_from_iter(iter).expect_err("should be err");
    assert_eq!(err, TOO_LONG_ERR, "should match err");
}
