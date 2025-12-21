#[test]
fn try_from_iter_array_example() {
    use collect_failable::errors::CapacityMismatch;
    use collect_failable::TryCollectEx;

    // while `TryFromIterator` can be used directly, typically `TryCollectEx` is preferred
    let data = 1..=3;
    let array: [_; 3] = data.into_iter().try_collect_ex().expect("should be ok");
    assert_eq!(array, [1, 2, 3]);

    // an iterator with too many or too few items will fail.
    let data = 1..=2; // too few
    let err = data.into_iter().try_collect_ex::<[u32; 3]>().expect_err("should be err");
    assert_eq!(err.error, CapacityMismatch::bounds(3..=3, (2, Some(2))));

    let data = 1..=4; // too many
    let err = data.into_iter().try_collect_ex::<[u32; 3]>().expect_err("should be err");
    assert_eq!(err.error, CapacityMismatch::bounds(3..=3, (4, Some(4))));
}
