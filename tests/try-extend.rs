#[derive(Debug, PartialEq, Eq)]
struct Collection {
    called: bool,
}

const DEFAULT: Collection = Collection { called: false };
const CALLED: Collection = Collection { called: true };

impl<I> collect_failable::TryExtend<i32, I> for Collection
where
    I: IntoIterator<Item = i32>,
{
    type Error = ();

    fn try_extend(&mut self, _iter: I) -> Result<(), Self::Error> {
        self.called = true;
        Ok(())
    }
}

impl<I> collect_failable::TryExtendSafe<i32, I> for Collection
where
    I: IntoIterator<Item = i32>,
{
    fn try_extend_safe(&mut self, _iter: I) -> Result<(), Self::Error> {
        self.called = true;
        Ok(())
    }
}

#[test]
fn test_try_extend_calls_try_extend_safe_by_default() {
    use collect_failable::TryExtend;

    let mut test = DEFAULT;
    test.try_extend(vec![1, 2, 3]).expect("Should be ok");
    assert_eq!(test, CALLED);
}

#[test]
fn try_extend_safe_map_collision_example() {
    use collect_failable::TryExtendSafe;
    use std::collections::HashMap;

    let mut map = HashMap::from([(1, 2), (2, 3)]);
    let err = map.try_extend_safe([(3, 4), (1, 5), (4, 6)]).expect_err("should collide");

    assert_eq!(err.item, (1, 5));

    let all: Vec<_> = err.into_iter().collect();
    assert_eq!(all.len(), 3, "length should be unchanged");
    // iterator can be reconstructed. Order is not guranteed for hashmap
    assert!(all.contains(&(3, 4)) && all.contains(&(1, 5)) && all.contains(&(4, 6)));

    assert_eq!(map, HashMap::from([(1, 2), (2, 3)]), "map should be unchanged");
}

#[test]
fn try_extend_safe_internal_iterator_collision() {
    use collect_failable::TryExtendSafe;
    use std::collections::HashMap;

    let mut map = HashMap::from([(1, 2)]);

    let err = map.try_extend_safe([(2, 4), (3, 5), (2, 6)]).expect_err("should collide within iterator");

    assert_eq!(err.item, (2, 6), "should detect the colliding key");

    assert_eq!(map, HashMap::from([(1, 2)]), "map should be unchanged");
}

#[test]
fn try_extend_safe_success_example() {
    use collect_failable::TryExtendSafe;
    use std::collections::HashMap;

    // works like `extend` if there are no collisions
    let mut map = HashMap::from([(1, 2)]);
    let result = map.try_extend_safe([(2, 3)]);

    assert!(result.is_ok());
    assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
}

#[test]
fn try_extend_basic_guarantee_example() {
    use collect_failable::TryExtend;
    use std::collections::HashMap;

    let mut map = HashMap::from([(1, 2)]);
    let err = map.try_extend([(2, 3), (3, 4), (1, 5)]).expect_err("should be err");

    assert_eq!(err.item, (1, 5));

    // map may be modified, but colliding value should not be changed
    assert_eq!(map[&1], 2);
    assert_eq!(map[&2], 3);
    assert_eq!(map[&3], 4);
}

#[test]
fn try_extend_success_example() {
    use collect_failable::TryExtend;
    use std::collections::HashMap;

    // works like `extend` if there are no collisions
    let mut map = HashMap::from([(1, 2)]);
    let result = map.try_extend([(2, 3)]);

    assert!(result.is_ok());
    assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
}
