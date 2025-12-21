#[test]
fn try_extend_tuple_example() {
    use collect_failable::TryExtend;
    use std::collections::HashSet;

    let mut data = (HashSet::new(), HashSet::new());
    data.try_extend([(1, 2), (2, 3)]).unwrap();

    assert_eq!(data.0, HashSet::from([1, 2]));
    assert_eq!(data.1, HashSet::from([2, 3]));
}
