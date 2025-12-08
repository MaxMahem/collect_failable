#[test]
fn try_from_iter_collision_example() {
    use collect_failable::TryFromIterator;
    use std::collections::HashMap;

    let err = HashMap::try_from_iter([(1, 2), (1, 3)]).expect_err("should collide");
    let parts = err.into_parts();
    assert_eq!(parts.item.0, 1, "colliding key should be 1");
}

#[test]
fn try_from_iter_success_example() {
    use collect_failable::TryFromIterator;
    use std::collections::HashMap;

    let result = HashMap::try_from_iter([(1, 2), (2, 3)]);
    let map = result.expect("should be Ok");
    assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
}
