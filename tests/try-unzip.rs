const INVALID_DATA_A: [(u32, u32); 2] = [(1, 3), (1, 4)];
const INVALID_DATA_B: [(u32, u32); 2] = [(1, 2), (2, 2)];

#[test]
fn try_unzip_fail_a() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let err_a =
        INVALID_DATA_A.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should be Err").unwrap_a();
    assert_eq!(err_a.error.item, 1, "Should be colliding value");
}

#[test]
fn try_unzip_fail_b() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    let err_b =
        INVALID_DATA_B.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should be Err").unwrap_b();
    assert_eq!(err_b.error.item, 2, "Should be colliding value");
}

#[test]
fn try_unzip_recover_partial_b_on_a_failure() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    // When FromA fails, the incomplete FromB should be recoverable
    let data = vec![(1, 10), (2, 20), (1, 30), (3, 40)]; // Collision on first element (A)
    let err_a = data.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should fail").unwrap_a();
    assert_eq!(err_a.error.item, 1);
    assert_eq!(err_a.incomplete, HashSet::from([10, 20]));
    assert_eq!(err_a.remaining.len(), 1);
}

#[test]
fn try_unzip_recover_partial_a_on_b_failure() {
    use collect_failable::TryUnzip;
    use std::collections::HashSet;

    // When FromB fails, the incomplete FromA should be recoverable
    let data = vec![(10, 1), (20, 2), (30, 2), (40, 3)]; // Collision on second element (B)
    let err_b = data.into_iter().try_unzip::<_, _, HashSet<_>, HashSet<_>>().expect_err("Should fail").unwrap_b();

    assert_eq!(err_b.error.item, 2);
    assert_eq!(err_b.incomplete, HashSet::from([10, 20, 30]));
    assert_eq!(err_b.remaining.len(), 1);
}
