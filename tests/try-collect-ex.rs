#[test]
fn failable_collect_no_collision_matches_collect() {
    use collect_failable::TryCollectEx;
    use std::collections::HashMap;

    let data = [(1, 2), (2, 3)];

    let try_collect_map = data.into_iter().try_collect_ex::<HashMap<_, _>>().expect("should be ok");

    // matches collect implementation
    assert_eq!(HashMap::from(data), try_collect_map);
}

#[test]
fn try_collect_ex_collision_example() {
    use collect_failable::{KeyCollision, TryCollectEx};
    use std::collections::HashMap;

    let data = [(1, 2), (1, 3)];
    let result = data.into_iter().try_collect_ex::<HashMap<_, _>>().expect_err("should be err");
    assert_eq!(result, KeyCollision { key: 1 });
}
