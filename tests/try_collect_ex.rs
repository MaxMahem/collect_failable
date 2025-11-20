use collect_failable::TryCollectEx;
use std::collections::HashMap;

#[test]
fn failable_collect_no_collision_matches_collect() {
    let data = [(1, 2), (2, 3)];

    let collect_map = data.into_iter().collect::<HashMap<_, _>>();
    let try_collect_map = data.into_iter().try_collect_ex::<HashMap<_, _>>().expect("should be ok");

    // matches collect implementation
    assert_eq!(collect_map, try_collect_map);
}

#[test]
fn failable_collect_collision_fails() {
    // colliding keys errors
    let try_collect_err = [(1, 2), (1, 3)].into_iter().try_collect_ex::<HashMap<_, _>>().expect_err("should be err");

    assert_eq!(try_collect_err.key, 1);
}
