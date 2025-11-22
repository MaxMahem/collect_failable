#[test]
fn try_from_iter_collision_example() {
    use collect_failable::{KeyCollision, TryFromIterator};
    use std::collections::HashMap;

    let err = HashMap::try_from_iter([(1, 2), (1, 3)]).expect_err("should collide");
    assert_eq!(err, KeyCollision { key: 1 });
}

#[test]
fn try_from_iter_success_example() {
    use collect_failable::TryFromIterator;
    use std::collections::HashMap;

    let result = HashMap::try_from_iter([(1, 2), (2, 3)]);
    assert_eq!(result, Ok(HashMap::from([(1, 2), (2, 3)])));
}
