#[test]
fn try_from_iter_arrayvec_example() {
    use arrayvec::ArrayVec;
    use collect_failable::{ExceedsCapacity, TryFromIterator};

    let array: ArrayVec<i32, 4> = ArrayVec::try_from_iter(1..=3).expect("Should be ok");
    assert_eq!(array.as_slice(), &[1, 2, 3]);

    let err = ArrayVec::<i32, 3>::try_from_iter(1..=4).expect_err("should be err");
    assert_eq!(err, ExceedsCapacity { capacity: 3, necessary: 4 });
}

#[test]
fn try_extend_safe_arrayvec_example() {
    use arrayvec::ArrayVec;
    use collect_failable::{ExceedsCapacity, TryCollectEx, TryExtend};

    let mut array: ArrayVec<i32, 4> = (1..=2).try_collect_ex().expect("Should be ok");

    array.try_extend_safe([3]).expect("Should be ok");
    assert_eq!(*array, [1, 2, 3]);

    let err = array.try_extend_safe([4, 5]).expect_err("Should be err");
    assert_eq!(err, ExceedsCapacity { capacity: 4, necessary: 5 });
    assert_eq!(*array, [1, 2, 3]); // Unchanged
}

#[test]
fn try_extend_arrayvec_example() {
    use arrayvec::ArrayVec;
    use collect_failable::{ExceedsCapacity, TryCollectEx, TryExtend};

    let mut array: ArrayVec<i32, 4> = (1..=2).try_collect_ex().expect("Should be ok");

    array.try_extend([3]).expect("Should be ok");
    assert_eq!(*array, [1, 2, 3]);

    let err = array.try_extend([4, 5]).expect_err("Should be err");
    assert_eq!(err, ExceedsCapacity { capacity: 4, necessary: 5 });
}
