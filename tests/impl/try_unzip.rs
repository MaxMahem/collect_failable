const INVALID_DATA_A: [(u32, u32); 2] = [(1, 3), (1, 4)];
const INVALID_DATA_B: [(u32, u32); 2] = [(1, 2), (2, 2)];

#[test]
fn try_unzip_success() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let data = vec![(1, 10), (2, 20), (3, 30)];
    let (set_a, set_b) = data.into_iter().try_unzip::<HashSet<_>, HashSet<_>>().expect("Should succeed");

    assert_eq!(set_a, HashSet::from([1, 2, 3]));
    assert_eq!(set_b, HashSet::from([10, 20, 30]));
}

#[test]
fn try_unzip_fail_a() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let err = INVALID_DATA_A.into_iter().try_unzip::<HashSet<_>, HashSet<_>>().expect_err("Should be Err");
    let data = err.into_data();
    let side = data.side.left().expect("Should be left");

    assert_eq!(side.error.item, 1, "Should be colliding value");
}

#[test]
fn try_unzip_fail_b() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let err = INVALID_DATA_B.into_iter().try_unzip::<HashSet<_>, HashSet<_>>().expect_err("Should be Err");
    let data = err.into_data();
    let side = data.side.right().expect("Should be right");

    assert_eq!(side.error.item, 2, "Should be colliding value");
}

#[test]
fn try_unzip_recover_partial_b_on_a_failure() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    // When FromA fails, the incomplete FromB should be recoverable
    let data = vec![(1, 10), (2, 20), (1, 30), (3, 40)]; // Collision on first element (A)
    let err = data.into_iter().try_unzip::<HashSet<_>, HashSet<_>>().expect_err("Should fail");
    let err_data = err.into_data();
    let side = err_data.side.left().expect("Should be left");

    assert_eq!(side.error.item, 1);
    assert_eq!(side.failed, HashSet::from([1, 2])); // Failed collection A
    assert_eq!(side.successful, HashSet::from([10, 20])); // Successful collection B
    assert_eq!(err_data.remaining.len(), 1);
}

#[test]
fn try_unzip_recover_partial_a_on_b_failure() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    // When FromB fails, the incomplete FromA should be recoverable
    let data = vec![(10, 1), (20, 2), (30, 2), (40, 3)]; // Collision on second element (B)
    let err = data.into_iter().try_unzip::<HashSet<_>, HashSet<_>>().expect_err("Should fail");
    let err_data = err.into_data();
    let side = err_data.side.right().expect("Should be right");

    assert_eq!(side.error.item, 2);
    assert_eq!(side.failed, HashSet::from([1, 2])); // Failed collection B
    assert_eq!(side.successful, HashSet::from([10, 20, 30])); // Successful collection A
    assert_eq!(err_data.remaining.len(), 1);
}
