#[derive(Debug, PartialEq, Eq)]
struct Collection {
    called: bool,
}

const DEFAULT: Collection = Collection { called: false };
const CALLED: Collection = Collection { called: true };

impl collect_failable::TryExtend<i32> for Collection {
    type Error = ();

    fn try_extend<I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = i32>,
    {
        self.called = true;
        Ok(())
    }
}

impl collect_failable::TryExtendSafe<i32> for Collection {
    fn try_extend_safe<I>(&mut self, _iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = i32>,
    {
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
    use collect_failable::{KeyCollision, TryExtendSafe};
    use std::collections::HashMap;

    let mut map = HashMap::from([(1, 2)]);
    let result = map.try_extend_safe([(1, 3)]);

    assert_eq!(result, Err(KeyCollision { key: 1 }));

    // map is unchanged
    assert_eq!(map, HashMap::from([(1, 2)]));
}

#[test]
fn try_extend_safe_internal_iterator_collision() {
    use collect_failable::{KeyCollision, TryExtendSafe};
    use std::collections::HashMap;

    let mut map = HashMap::from([(1, 2)]);

    // collisions within the iterator itself are also detected
    let result = map.try_extend_safe([(2, 4), (2, 5)]);

    // result is an error with the colliding key
    assert_eq!(result, Err(KeyCollision { key: 2 }));

    // map is unchanged
    assert_eq!(map, HashMap::from([(1, 2)]));
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
    use collect_failable::{KeyCollision, TryExtend};
    use std::collections::HashMap;

    let mut map = HashMap::from([(1, 2)]);
    let err = map.try_extend([(2, 3), (1, 3)]).expect_err("should be err");

    assert_eq!(err, KeyCollision { key: 1 });

    // map may be modified, but colliding value should not be changed
    assert_eq!(map[&1], 2);
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
