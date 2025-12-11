use collect_failable::{CollectionCollision, TryUnzip, UnzipError};
use std::array::IntoIter;
use std::collections::HashSet;
use std::error::Error;
use std::iter::Once;

use crate::test_macros::{expect_panic, test_format};

/// Data that will cause collision on first collection (A side)
const COLLISION_DATA_A: [(u32, u32); 4] = [(1, 10), (2, 20), (1, 30), (3, 40)];

/// Data that will cause collision on second collection (B side)
const COLLISION_DATA_B: [(u32, u32); 4] = [(10, 1), (20, 2), (30, 1), (40, 3)];

const EXPECTED_DEBUG_UNZIP_ERROR_A: &str = r#"UnzipError::A(ZipErrorSide { error: CollectionCollision { collected: "std::collections::hash::set::HashSet<u32>", item: "u32", iterator: "core::iter::sources::once::Once<u32>" }, incomplete: "std::collections::hash::set::HashSet<u32>", unevaluated: Some(u32), remaining: "core::array::iter::IntoIter<(u32, u32), 4>" })"#;
const EXPECTED_DEBUG_UNZIP_ERROR_B: &str = r#"UnzipError::B(ZipErrorSide { error: CollectionCollision { collected: "std::collections::hash::set::HashSet<u32>", item: "u32", iterator: "core::iter::sources::once::Once<u32>" }, incomplete: "std::collections::hash::set::HashSet<u32>", unevaluated: None, remaining: "core::array::iter::IntoIter<(u32, u32), 4>" })"#;
const EXPECTED_DEBUG_ZIP_ERROR_SIDE: &str = r#"ZipErrorSide { error: CollectionCollision { collected: "std::collections::hash::set::HashSet<u32>", item: "u32", iterator: "core::iter::sources::once::Once<u32>" }, incomplete: "std::collections::hash::set::HashSet<u32>", unevaluated: Some(u32), remaining: "core::array::iter::IntoIter<(u32, u32), 4>" }"#;
const EXPECTED_DISPLAY_UNZIP_ERROR_A: &str = "Failed while extending first collection: Collection collision";
const EXPECTED_DISPLAY_UNZIP_ERROR_B: &str = "Failed while extending second collection: Collection collision";

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
    let parts = create_a_error().expect_a("Should fail on A side").into_parts();

    assert_eq!(parts.error.item, 1);
    assert_eq!(parts.incomplete, HashSet::from([10, 20]));
    assert_eq!(parts.unevaluated, Some(30));
    assert_eq!(parts.remaining.collect::<Vec<_>>(), vec![(3, 40)]);
}

test_format!(unzip_error_debug_a, create_a_error(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_A);
test_format!(unzip_error_debug_b, create_b_error(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_B);
test_format!(unzip_error_display_a, create_a_error(), "{}", EXPECTED_DISPLAY_UNZIP_ERROR_A);
test_format!(unzip_error_display_b, create_b_error(), "{}", EXPECTED_DISPLAY_UNZIP_ERROR_B);
test_format!(zip_error_side_debug, create_a_error().unwrap_a(), "{:?}", EXPECTED_DEBUG_ZIP_ERROR_SIDE);

#[test]
fn zip_error_side_a_source() {
    let error = create_a_error();
    let source = error.source().expect("Should have error source");
    assert!(source.is::<CollectionCollision<u32, Once<u32>, Collection>>());
}

#[test]
fn zip_error_side_b_source() {
    let error = create_b_error();
    let source = error.source().expect("Should have error source");
    assert!(source.is::<CollectionCollision<u32, Once<u32>, Collection>>());
}
