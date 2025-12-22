use collect_failable::errors::CollectionError;
use std::collections::HashSet;

use crate::error_tests::{into_iterator, test_format, test_source, TestError};

type Collection = HashSet<u32>;

fn create_with_rejected() -> CollectionError<std::array::IntoIter<u32, 2>, Collection, TestError> {
    let remaining = [3, 4];
    let iter = remaining.into_iter();
    let collected = HashSet::from([1, 2]);
    let rejected = Some(99);

    CollectionError::new(iter, collected, rejected, TestError::new("with rejected"))
}

fn create_without_rejected() -> CollectionError<std::array::IntoIter<u32, 2>, Collection, TestError> {
    let remaining = [3, 4];
    let iter = remaining.into_iter();
    let collected = HashSet::from([1, 2]);
    let rejected = None;

    CollectionError::new(iter, collected, rejected, TestError::new("without rejected"))
}

const EXPECTED_DEBUG_WITH_REJECTED: &str = r#"PartialIterErr { collected: "std::collections::hash::set::HashSet<u32>", rejected: Some(..), error: TestError { identity: "with rejected" }, iterator: "core::array::iter::IntoIter<u32, 2>" }"#;
const EXPECTED_DEBUG_WITHOUT_REJECTED: &str = r#"PartialIterErr { collected: "std::collections::hash::set::HashSet<u32>", rejected: None, error: TestError { identity: "without rejected" }, iterator: "core::array::iter::IntoIter<u32, 2>" }"#;
const EXPECTED_DISPLAY_WITH_REJECTED: &str = "Test error: with rejected";
const EXPECTED_DISPLAY_WITHOUT_REJECTED: &str = "Test error: without rejected";

test_format!(debug_format_with_rejected, create_with_rejected(), "{:?}", EXPECTED_DEBUG_WITH_REJECTED);
test_format!(debug_format_without_rejected, create_without_rejected(), "{:?}", EXPECTED_DEBUG_WITHOUT_REJECTED);
test_format!(display_format_with_rejected, create_with_rejected(), "{}", EXPECTED_DISPLAY_WITH_REJECTED);
test_format!(display_format_without_rejected, create_without_rejected(), "{}", EXPECTED_DISPLAY_WITHOUT_REJECTED);

#[test]
fn into_err() {
    let error = create_with_rejected().into_error();
    assert_eq!(error, TestError::new("with rejected"));
}

#[test]
fn into_parts_with_rejected() {
    let parts = create_with_rejected().into_data();

    assert_eq!(parts.error, TestError::new("with rejected"));
    assert_eq!(parts.collected, HashSet::from([1, 2]));
    assert_eq!(parts.rejected, Some(99));
    assert_eq!(parts.iterator.collect::<Vec<_>>(), vec![3, 4]);
}

#[test]
fn into_parts_without_rejected() {
    let error = create_without_rejected();
    let parts = error.into_data();

    assert_eq!(parts.error, TestError::new("without rejected"));
    assert_eq!(parts.collected, HashSet::from([1, 2]));
    assert_eq!(parts.rejected, None);
    assert_eq!(parts.iterator.collect::<Vec<_>>(), vec![3, 4]);
}

// Should contain: rejected (99) + collected (1, 2 in some order) + remaining (3, 4)
into_iterator!(into_iterator_with_rejected, create_with_rejected(), expected_len = 5, contains = [99, 1, 2, 3, 4]);

// Should contain: collected (1, 2 in some order) + remaining (3, 4)
into_iterator!(into_iterator_without_rejected, create_without_rejected(), expected_len = 4, contains = [1, 2, 3, 4]);

test_source!(error_trait_source, create_with_rejected(), TestError);
