#[test]
fn try_from_iter_tuple_example() {
    use std::collections::HashSet;
    use collect_failable::TryFromIterator;

    let data = vec![(1, 2), (2, 3), (3, 4)];
    let (a, b): (HashSet<i32>, HashSet<i32>) = TryFromIterator::try_from_iter(data).unwrap();

    assert_eq!(a, HashSet::from([1, 2, 3]));
    assert_eq!(b, HashSet::from([2, 3, 4]));
}

#[test]
fn try_extend_safe_tuple_example() {
    use std::collections::HashSet;
    use collect_failable::TryExtend;

    let mut data = (HashSet::new(), HashSet::new());
    data.try_extend_safe([(1, 2), (2, 3)]).unwrap();

    assert_eq!(data.0, HashSet::from([1, 2]));
    assert_eq!(data.1, HashSet::from([2, 3]));
}

#[test]
fn try_extend_tuple_example() {
    use std::collections::HashSet;
    use collect_failable::TryExtend;

    let mut data = (HashSet::new(), HashSet::new());
    data.try_extend([(1, 2), (2, 3)]).unwrap();

    assert_eq!(data.0, HashSet::from([1, 2]));
    assert_eq!(data.1, HashSet::from([2, 3]));
}
