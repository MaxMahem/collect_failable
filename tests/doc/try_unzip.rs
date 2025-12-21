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

    match vec![(1, "a"), (2, "b"), (1, "c"), (3, "d")].into_iter().try_unzip::<HashSet<_>, HashSet<_>>() {
        Err(err) => {
            let data = err.into_data();
            let side = data.side.left().expect("Should be left");

            assert_eq!(side.error.item, 1);
            assert_eq!(side.failed, HashSet::from([1, 2]));
            assert_eq!(side.successful, HashSet::from(["a", "b"]));
            assert_eq!(side.unevaluated, Some("c"));
            assert_eq!(data.remaining.collect::<Vec<_>>(), [(3, "d")]);
        }
        Ok(_) => panic!("Should be Err"),
    }
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
