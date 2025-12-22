use collect_failable::errors::CollectionCollision;
use std::collections::HashSet;

use crate::error_tests::{getter, into_iterator, test_format};

type Collection = HashSet<u32>;

fn create_collision() -> CollectionCollision<std::vec::IntoIter<u32>, Collection> {
    let collected = HashSet::from([1, 2]);
    let item = 4; // Collides with item in collected
    let iterator = vec![3].into_iter(); // Remaining items

    CollectionCollision::new(iterator, collected, item)
}

const EXPECTED_DEBUG: &str = r#"CollectionCollision { collected: "std::collections::hash::set::HashSet<u32>", item: "u32", iterator: "alloc::vec::into_iter::IntoIter<u32>" }"#;
const EXPECTED_DISPLAY: &str = "Collection collision";

test_format!(debug_format, create_collision(), "{:?}", EXPECTED_DEBUG);
test_format!(display_format, create_collision(), "{}", EXPECTED_DISPLAY);

#[test]
fn into_parts() {
    let error = create_collision();
    let parts = error.into_data();

    assert_eq!(parts.item, 4);
    assert_eq!(parts.collected, HashSet::from([1, 2]));
    assert_eq!(parts.iterator.collect::<Vec<_>>(), vec![3]);
}

getter!(item, create_collision(), into_item(), 4);

// Should contain: item (1) + collected (1, 2 in some order) + remaining (3)
into_iterator!(into_iterator, create_collision(), expected_len = 4, contains = [1, 2, 3, 4]);
