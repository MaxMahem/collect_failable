use collect_failable::TupleCollectionError;

use crate::error_tests::{expect_panic, test_format, test_source, TestError};

fn create_a_error() -> TupleCollectionError<TestError, TestError, Vec<u32>, Vec<u32>> {
    TupleCollectionError::new_a(TestError::new("A side failed"), vec![1, 2, 3])
}

fn create_b_error() -> TupleCollectionError<TestError, TestError, Vec<u32>, Vec<u32>> {
    TupleCollectionError::new_b(TestError::new("B side failed"), vec![4, 5, 6])
}

const EXPECTED_DEBUG_A: &str = r#"TupleCollectionError::A(TupleCollectionErrorSide { error: TestError { identity: "A side failed" }, from: "alloc::vec::Vec<u32>" })"#;
const EXPECTED_DEBUG_B: &str = r#"TupleCollectionError::B(TupleCollectionErrorSide { error: TestError { identity: "B side failed" }, from: "alloc::vec::Vec<u32>" })"#;
const EXPECTED_DISPLAY_A: &str = "Failed while collecting first collection: Test error: A side failed";
const EXPECTED_DISPLAY_B: &str = "Failed while collecting second collection: Test error: B side failed";

test_format!(debug_a, create_a_error(), "{:?}", EXPECTED_DEBUG_A);
test_format!(debug_b, create_b_error(), "{:?}", EXPECTED_DEBUG_B);
test_format!(display_a, create_a_error(), "{}", EXPECTED_DISPLAY_A);
test_format!(display_b, create_b_error(), "{}", EXPECTED_DISPLAY_B);

expect_panic!(expect_a_panic, create_b_error(), expect_a, "Should panic with msg");
expect_panic!(expect_b_panic, create_a_error(), expect_b, "Should panic with msg");

#[test]
fn unwrap_a() {
    let error = create_a_error();
    let side = error.unwrap_a();
    assert_eq!(side.error, TestError::new("A side failed"));
}

#[test]
fn unwrap_b() {
    let error = create_b_error();
    let side = error.unwrap_b();
    assert_eq!(side.error, TestError::new("B side failed"));
}

#[test]
fn error_side_into_error() {
    let error = create_a_error();
    let side = error.unwrap_a();
    let inner = side.into_error();
    assert_eq!(inner, TestError::new("A side failed"));
}

#[test]
fn error_side_into_parts() {
    let error = create_a_error();
    let side = error.unwrap_a();
    let parts = side.into_data();

    assert_eq!(parts.error, TestError::new("A side failed"));
    assert_eq!(parts.from, vec![1, 2, 3]);
}

test_source!(error_trait_source_a, create_a_error(), TestError);
test_source!(error_trait_source_b, create_b_error(), TestError);
