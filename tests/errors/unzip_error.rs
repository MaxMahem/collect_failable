use std::array::IntoIter;
use std::collections::HashSet;

use collect_failable::errors::{Collision, UnzipError};
use collect_failable::TryUnzip;

use crate::error_tests::{test_format, test_source};

/// Data that will cause collision on first collection (A side)
const COLLISION_DATA_A: [(u32, u32); 4] = [(1, 10), (2, 20), (1, 30), (3, 40)];

/// Data that will cause collision on second collection (B side)
const COLLISION_DATA_B: [(u32, u32); 4] = [(10, 1), (20, 2), (30, 1), (40, 3)];

const EXPECTED_DEBUG_UNZIP_ERROR_A: &str = r#"UnzipError { side: Left(UnzipSide { error: Collision { item: 1 }, failed: HashSet<u32>, successful: HashSet<u32>, unevaluated: u32 }), remaining: IntoIter<(u32, u32), 4> }"#;
const EXPECTED_DEBUG_UNZIP_ERROR_B: &str = r#"UnzipError { side: Right(UnzipSide { error: Collision { item: 1 }, failed: HashSet<u32>, successful: HashSet<u32>, unevaluated: u32 }), remaining: IntoIter<(u32, u32), 4> }"#;
const EXPECTED_DEBUG_UNZIP_ERROR_DATA_A: &str = r#"UnzipErrorData { side: Left(UnzipSide { error: Collision { item: 1 }, failed: HashSet<u32>, successful: HashSet<u32>, unevaluated: u32 }), remaining: IntoIter<(u32, u32), 4> }"#;
const EXPECTED_DEBUG_UNZIP_ERROR_DATA_B: &str = r#"UnzipErrorData { side: Right(UnzipSide { error: Collision { item: 1 }, failed: HashSet<u32>, successful: HashSet<u32>, unevaluated: u32 }), remaining: IntoIter<(u32, u32), 4> }"#;
const EXPECTED_DISPLAY_UNZIP_ERROR_A: &str = "Failed while unzipping into first collection: item collision";
const EXPECTED_DISPLAY_UNZIP_ERROR_B: &str = "Failed while unzipping into second collection: item collision";

type Collection = HashSet<u32>;

fn create_a_error() -> UnzipError<Collection, Collection, IntoIter<(u32, u32), 4>> {
    COLLISION_DATA_A.into_iter().try_unzip::<Collection, Collection>().expect_err("Should fail on A side")
}

fn create_b_error() -> UnzipError<Collection, Collection, IntoIter<(u32, u32), 4>> {
    COLLISION_DATA_B.into_iter().try_unzip::<Collection, Collection>().expect_err("Should fail on B side")
}

#[test]
fn into_data() {
    let error = create_a_error();
    let data = error.into_data();

    let side = data.side.left().expect("Should be left");
    assert_eq!(side.error.item, 1);
    assert_eq!(side.failed, HashSet::from([1, 2])); // Failed collection A
    assert_eq!(side.successful, HashSet::from([10, 20])); // Successful collection B
    assert_eq!(side.unevaluated, Some(30));
    assert_eq!(data.remaining.collect::<Vec<_>>(), vec![(3, 40)]);
}

#[test]
fn deref_field_access() {
    let error = create_a_error();
    assert!(error.side.is_left());
}

#[test]
fn remaining_accessible() {
    let error = create_a_error();
    let type_name = std::any::type_name_of_val(&error.remaining);
    assert!(type_name.contains("IntoIter"));
}

test_format!(unzip_error_debug_a, create_a_error(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_A);
test_format!(unzip_error_debug_b, create_b_error(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_B);
test_format!(unzip_error_data_debug_a, create_a_error().into_data(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_DATA_A);
test_format!(unzip_error_data_debug_b, create_b_error().into_data(), "{:?}", EXPECTED_DEBUG_UNZIP_ERROR_DATA_B);
test_format!(unzip_error_display_a, create_a_error(), "{}", EXPECTED_DISPLAY_UNZIP_ERROR_A);
test_format!(unzip_error_display_b, create_b_error(), "{}", EXPECTED_DISPLAY_UNZIP_ERROR_B);

test_source!(unzip_error_a_source, create_a_error(), Collision<u32>);
test_source!(unzip_error_b_source, create_b_error(), Collision<u32>);
