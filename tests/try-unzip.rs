const INVALID_DATA_A: [(u32, u32); 2] = [(1, 3), (1, 4)];
const INVALID_DATA_B: [(u32, u32); 2] = [(1, 2), (2, 2)];

#[test]
fn try_unzip_fail_a() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let err = INVALID_DATA_A.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should be Err");
    assert_eq!(err.unwrap_a().item, 1);
}

#[test]
fn try_unzip_fail_b() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let err = INVALID_DATA_B.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should be Err");
    assert_eq!(err.unwrap_b().item, 2);
}

#[test]
fn try_unzip_success_example() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let data = vec![(1, 2), (2, 3)];
    let (a, b): (HashSet<i32>, HashSet<i32>) = data.into_iter().try_unzip().expect("should be ok");

    assert_eq!(a, HashSet::from([1, 2]));
    assert_eq!(b, HashSet::from([2, 3]));
}

#[test]
fn try_unzip_collision_example() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let data = vec![(1, 2), (1, 3)];
    let err = data.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should fail");
    assert_eq!(err.unwrap_a().item, 1);
}

#[test]
fn try_unzip_different_containers_example() {
    use collect_failable::TryUnzip;
    use std::collections::{BTreeSet, HashSet};

    // Unzip into two different container types
    let data = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (nums, chars): (BTreeSet<_>, HashSet<_>) = data.into_iter().try_unzip().expect("should be ok");

    assert_eq!(nums, BTreeSet::from([1, 2, 3]));
    assert_eq!(chars, HashSet::from(['a', 'b', 'c']));
}
