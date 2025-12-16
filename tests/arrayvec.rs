mod collection_tests;
use collection_tests::{try_collect, try_extend, try_extend_safe};

use arrayvec::ArrayVec;

use collect_failable::utils::FixedSizeHintEx;
use collect_failable::{CapacityMismatch, TryExtend, TryExtendSafe, TryFromIterator};

type TestArray = ArrayVec<u32, 2>;

mod try_from_iter {
    use super::*;

    try_collect!(valid, TestArray, 1..=2, Ok(ArrayVec::from_iter(1..=2)));
    try_collect!(bounds, TestArray, 1..=3, Err(CapacityMismatch::bounds(0..=2, (3, Some(3)))));
    try_collect!(overflow, TestArray, (1..=3).hide_size(), Err(CapacityMismatch::overflow(0..=2)));
}

#[test]
fn try_from_iter_reconstruct() {
    let err = TestArray::try_from_iter(1..=3).expect_err("Should be err");

    let reconstructed: Vec<u32> = err.into_iter().collect();
    assert_eq!(reconstructed.len(), 3, "Should reconstruct as: rejected, collected, remaining");
    assert!(reconstructed.contains(&1));
    assert!(reconstructed.contains(&2));
    assert!(reconstructed.contains(&3));
}

mod try_extend_safe {
    use super::*;

    try_extend_safe!(valid, TestArray::from_iter(3..=3), 3..=3, Ok(TestArray::from_iter([3, 3])));
    try_extend_safe!(bound_fail, TestArray::from_iter(3..=3), 1..=3, Err(CapacityMismatch::bounds(0..=1, (1..=3).size_hint())));
    try_extend_safe!(overflow, TestArray::from_iter(3..=3), (1..=2).hide_size(), Err(CapacityMismatch::overflow(0..=1)));
}

mod try_extend {
    use super::*;

    try_extend!(valid, ArrayVec::<u32, 2>::from_iter(3..=3), 3..=3, Ok(ArrayVec::from_iter([3, 3])));
    try_extend!(bounds_fail, ArrayVec::<u32, 2>::from_iter(3..=3), 1..=3, Err(CapacityMismatch::bounds(0..=1, (3, Some(3)))));
    try_extend!(overflow, ArrayVec::<u32, 2>::from_iter(3..=3), (1..=2).hide_size(), Err(CapacityMismatch::overflow(0..=1)));
}
