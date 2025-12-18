#[test]
fn try_collect_ex_collision_example() {
    use collect_failable::TryCollectEx;
    use std::collections::HashMap;

    let data = [(1, 2), (1, 3)];
    let result = data.into_iter().try_collect_ex::<HashMap<_, _>>().expect_err("should be err");

    // CollectionCollision now contains the entire collision context
    let parts = result.into_data();
    assert_eq!(parts.item.0, 1, "colliding key should be 1");
}
