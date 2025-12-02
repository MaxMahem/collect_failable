#[test]
fn fold_mut_success_aggregate_hashmap_example() {
    use collect_failable::utils::FoldMut;
    use std::collections::HashMap;

    let pairs = vec![("a", 1), ("b", 2), ("a", 3)];

    let result = pairs.into_iter().fold_mut(HashMap::new(), |map, (key, val)| {
        map.entry(key).or_insert(0);
        *map.get_mut(key).unwrap() += val;
    });

    assert_eq!(result.get("a"), Some(&4));
    assert_eq!(result.get("b"), Some(&2));
}

#[test]
fn try_fold_mut_success() {
    use collect_failable::utils::FoldMut;
    use std::collections::BTreeMap;

    let pairs = vec![("a", 1), ("b", 2), ("a", 3)];

    let result = pairs
        .into_iter()
        .try_fold_mut(BTreeMap::new(), |map, (key, val)| {
            map.entry(key).or_insert(0);
            *map.get_mut(key).unwrap() += val;
            Ok::<(), ()>(())
        })
        .expect("should be ok");

    assert_eq!(result.get("a"), Some(&4));
    assert_eq!(result.get("b"), Some(&2));
}

#[test]
fn try_fold_mut_failure_example() {
    use collect_failable::utils::FoldMut;
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;

    let pairs = vec![("a", 1), ("b", -2), ("a", 3)];

    let err = pairs
        .into_iter()
        .try_fold_mut(HashMap::new(), |map, (key, val)| match map.entry(key) {
            Entry::Vacant(entry) => Ok(_ = entry.insert(val)),
            Entry::Occupied(_) => Err("key collision"),
        })
        .expect_err("should be err");

    assert_eq!(err, "key collision");
}
