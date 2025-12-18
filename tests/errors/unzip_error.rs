use collect_failable::{ItemCollision, TryUnzip, UnzipError};
use std::array::IntoIter;
use std::collections::HashSet;

use crate::error_tests::{expect_panic, test_format, test_source};

/// Data that will cause collision on first collection (A side)
const COLLISION_DATA_A: [(u32, u32); 4] = [(1, 10), (2, 20), (1, 30), (3, 40)];

/// Data that will cause collision on second collection (B side)
const COLLISION_DATA_B: [(u32, u32); 4] = [(10, 1), (20, 2), (30, 1), (40, 3)];

const EXPECTED_DEBUG_UNZIP_ERROR_A: &str = r#"UnzipError::A(ZipErrorSide { error: ItemCollision { item: 1 }, failed: .., successful: .., unevaluated: Some(..), remaining: "core::array::iter::IntoIter<(u32, u32), 4>" })"#;
const EXPECTED_DEBUG_UNZIP_ERROR_B: &str = r#"UnzipError::B(ZipErrorSide { error: ItemCollision { item: 1 }, failed: .., successful: .., unevaluated: None, remaining: "core::array::iter::IntoIter<(u32, u32), 4>" })"#;
const EXPECTED_DEBUG_ZIP_ERROR_SIDE: &str = r#"ZipErrorSide { error: ItemCollision { item: 1 }, failed: .., successful: .., unevaluated: Some(..), remaining: "core::array::iter::IntoIter<(u32, u32), 4>" }"#;
const EXPECTED_DISPLAY_UNZIP_ERROR_A: &str = "Failed while extending first collection: item collision";
const EXPECTED_DISPLAY_UNZIP_ERROR_B: &str = "Failed while extending second collection: item collision";

type Collection = HashSet<u32>;

fn create_a_error() -> UnzipError<u32, u32, Collection, Collection, IntoIter<(u32, u32), 4>> {
    COLLISION_DATA_A.into_iter().try_unzip::<_, _, Collection, Collection>().expect_err("Should fail on A side")
}

fn create_b_error() -> UnzipError<u32, u32, Collection, Collection, IntoIter<(u32, u32), 4>> {
    COLLISION_DATA_B.into_iter().try_unzip::<_, _, Collection, Collection>().expect_err("Should fail on B side")
}

expect_panic!(expect_a_panic, create_b_error(), expect_a, "Should panic with msg");
expect_panic!(expect_b_panic, create_a_error(), expect_b, "Should panic with msg");

#[test]
fn zip_error_side_into_err() {
    let error = create_a_error().expect_a("Should fail on A side").into_error();
    assert_eq!(error.item, 1);
}

#[test]
fn zip_error_side_into_parts() {
    let parts = create_a_error().expect_a("Should fail on A side").into_data();

    assert_eq!(parts.error.item, 1);
    assert_eq!(parts.failed, HashSet::from([1, 2])); // Failed collection A
    assert_eq!(parts.successful, HashSet::from([10, 20])); // Successful collection B
    assert_eq!(parts.unevaluated, Some(30));
    assert_eq!(parts.remaining.collect::<Vec<_>>(), vec![(3, 40)]);
}

test_format!(unzip_error_debug_a, create_a_error(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_A);
test_format!(unzip_error_debug_b, create_b_error(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_B);
test_format!(unzip_error_display_a, create_a_error(), "{}", EXPECTED_DISPLAY_UNZIP_ERROR_A);
test_format!(unzip_error_display_b, create_b_error(), "{}", EXPECTED_DISPLAY_UNZIP_ERROR_B);
test_format!(zip_error_side_debug, create_a_error().unwrap_a(), "{:?}", EXPECTED_DEBUG_ZIP_ERROR_SIDE);

test_source!(zip_error_side_a_source, create_a_error(), ItemCollision<u32>);
test_source!(zip_error_side_b_source, create_b_error(), ItemCollision<u32>);
