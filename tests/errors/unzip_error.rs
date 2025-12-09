use collect_failable::{CollectionCollision, TryUnzip, UnzipError};
use fluent_result::bool::dbg::Expect;
use std::{collections::HashSet, iter::Once};

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
type InnerError = CollectionCollision<u32, Once<u32>, Collection>;

fn create_a_error() -> UnzipError<u32, u32, Collection, Collection, std::array::IntoIter<(u32, u32), 4>> {
    COLLISION_DATA_A.into_iter().try_unzip::<_, _, Collection, Collection>().expect_err("Should fail on A side")
}

fn create_b_error() -> UnzipError<u32, u32, Collection, Collection, std::array::IntoIter<(u32, u32), 4>> {
    COLLISION_DATA_B.into_iter().try_unzip::<_, _, Collection, Collection>().expect_err("Should fail on B side")
}

/// Test that a formatted output (Debug/Display) matches expected value
macro_rules! test_format {
    ($name:ident, $setup:expr, $format:literal, $expected:expr) => {
        #[test]
        fn $name() {
            let output = format!($format, $setup);
            assert_eq!(output, $expected);
        }
    };
}

#[test]
#[should_panic(expected = "Should panic with msg")]
fn expect_a_panic() {
    _ = create_b_error().expect_a("Should panic with msg");
}

#[test]
#[should_panic(expected = "Should panic with msg")]
fn expect_b_panic() {
    _ = create_a_error().expect_b("Should panic with msg");
}

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
fn error_trait_source_a() {
    use std::error::Error;

    let err_a = create_a_error();
    let source = err_a.source().expect("Should have error source");
    source.is::<InnerError>().expect_true("Should have error source");
}

#[test]
fn error_trait_source_b() {
    use std::error::Error;

    let err_b = create_b_error();
    let source = err_b.source().expect("Should have error source");
    source.is::<InnerError>().expect_true("Should have error source");
}
