use arrayvec::ArrayVec;

use crate::utils::FixedSizeHintEx;
use collect_failable::errors::CapacityMismatch;

type TestArray = ArrayVec<u32, 2>;

mod try_from_iter {
    use super::super::collection_tests::try_collect;
    use super::*;
    use collect_failable::TryFromIterator;

    try_collect!(valid, TestArray, 1..=2, Ok(ArrayVec::from_iter(1..=2)));
    try_collect!(bounds, TestArray, 1..=3, Err(CapacityMismatch::bounds(0..=2, (3, Some(3)))));
    try_collect!(overflow, TestArray, (1..=3).hide_size(), Err(CapacityMismatch::overflow(0..=2)));
}

#[test]
fn try_from_iter_reconstruct() {
    use collect_failable::TryFromIterator;

    let err = TestArray::try_from_iter(1..=3).expect_err("Should be err");

    let reconstructed: Vec<u32> = err.into_iter().collect();
    assert_eq!(reconstructed.len(), 3, "Should reconstruct as: rejected, collected, remaining");
    assert!(reconstructed.contains(&1));
    assert!(reconstructed.contains(&2));
    assert!(reconstructed.contains(&3));
}

mod try_extend_safe {
    use super::super::collection_tests::try_extend_safe;
    use super::*;
    use collect_failable::TryExtendSafe;

    try_extend_safe!(valid, TestArray::from_iter(3..=3), 3..=3, Ok(TestArray::from_iter([3, 3])));
    try_extend_safe!(bound_fail, TestArray::from_iter(3..=3), 1..=3, Err(CapacityMismatch::bounds(0..=1, (1..=3).size_hint())));
    try_extend_safe!(overflow, TestArray::from_iter(3..=3), (1..=2).hide_size(), Err(CapacityMismatch::overflow(0..=1)));
}

mod try_extend {
    use super::super::collection_tests::try_extend;
    use super::*;
    use collect_failable::TryExtend;

    try_extend!(valid, TestArray::from_iter(3..=3), 3..=3, Ok(ArrayVec::from_iter([3, 3])));
    try_extend!(bounds_fail, TestArray::from_iter(3..=3), 1..=3, Err(CapacityMismatch::bounds(0..=1, (3, Some(3)))));
    try_extend!(overflow, TestArray::from_iter(3..=3), (1..=2).hide_size(), Err(CapacityMismatch::overflow(0..=1)));
}

mod try_extend_one {
    use super::super::collection_tests::try_extend_one;
    use arrayvec::CapacityError;
    use collect_failable::TryExtendOne;

    use super::*;

    try_extend_one!(valid, TestArray::new(), 1, Ok(TestArray::from_iter(1..=1)));
    try_extend_one!(collision, TestArray::from([1, 2]), 2, Err(CapacityError::new(2)));
}
