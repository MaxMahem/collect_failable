use collect_failable::TupleExtensionError;
use fluent_result::bool::dbg::Expect;

use super::test_macros::{expect_panic, test_format};

/// Simple error type for testing
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestError(String);

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test error: {}", self.0)
    }
}

impl std::error::Error for TestError {}

fn create_a_error_with_unevaluated() -> TupleExtensionError<TestError, TestError, u32, u32, std::array::IntoIter<(u32, u32), 3>> {
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_a(TestError("A side failed".to_string()), Some(20), remaining.into_iter())
}

fn create_a_error_without_unevaluated() -> TupleExtensionError<TestError, TestError, u32, u32, std::array::IntoIter<(u32, u32), 3>>
{
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_a(TestError("A side failed".to_string()), None, remaining.into_iter())
}

fn create_b_error_with_unevaluated() -> TupleExtensionError<TestError, TestError, u32, u32, std::array::IntoIter<(u32, u32), 3>> {
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_b(TestError("B side failed".to_string()), Some(10), remaining.into_iter())
}

fn create_b_error_without_unevaluated() -> TupleExtensionError<TestError, TestError, u32, u32, std::array::IntoIter<(u32, u32), 3>>
{
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_b(TestError("B side failed".to_string()), None, remaining.into_iter())
}

const EXPECTED_DEBUG_A_WITH: &str = r#"TupleExtensionError::A(TupleExtensionErrorSide { error: TestError("A side failed"), unevaluated: Some(..), remaining: "core::array::iter::IntoIter<(u32, u32), 3>" })"#;
const EXPECTED_DEBUG_A_WITHOUT: &str = r#"TupleExtensionError::A(TupleExtensionErrorSide { error: TestError("A side failed"), unevaluated: None, remaining: "core::array::iter::IntoIter<(u32, u32), 3>" })"#;
const EXPECTED_DEBUG_B_WITH: &str = r#"TupleExtensionError::B(TupleExtensionErrorSide { error: TestError("B side failed"), unevaluated: Some(..), remaining: "core::array::iter::IntoIter<(u32, u32), 3>" })"#;
const EXPECTED_DEBUG_B_WITHOUT: &str = r#"TupleExtensionError::B(TupleExtensionErrorSide { error: TestError("B side failed"), unevaluated: None, remaining: "core::array::iter::IntoIter<(u32, u32), 3>" })"#;
const EXPECTED_DISPLAY_A: &str = "Failed while extending first collection: Test error: A side failed";
const EXPECTED_DISPLAY_B: &str = "Failed while extending second collection: Test error: B side failed";

test_format!(debug_a_with_unevaluated, create_a_error_with_unevaluated(), "{:?}", EXPECTED_DEBUG_A_WITH);
test_format!(debug_a_without_unevaluated, create_a_error_without_unevaluated(), "{:?}", EXPECTED_DEBUG_A_WITHOUT);
test_format!(debug_b_with_unevaluated, create_b_error_with_unevaluated(), "{:?}", EXPECTED_DEBUG_B_WITH);
test_format!(debug_b_without_unevaluated, create_b_error_without_unevaluated(), "{:?}", EXPECTED_DEBUG_B_WITHOUT);
test_format!(display_a, create_a_error_with_unevaluated(), "{}", EXPECTED_DISPLAY_A);
test_format!(display_b, create_b_error_with_unevaluated(), "{}", EXPECTED_DISPLAY_B);

expect_panic!(expect_a_panic, create_b_error_with_unevaluated(), expect_a, "Should panic with msg");
expect_panic!(expect_b_panic, create_a_error_with_unevaluated(), expect_b, "Should panic with msg");

#[test]
fn unwrap_a() {
    let error = create_a_error_with_unevaluated();
    let side = error.unwrap_a();
    assert_eq!(side.error, TestError("A side failed".to_string()));
}

#[test]
fn unwrap_b() {
    let error = create_b_error_with_unevaluated();
    let side = error.unwrap_b();
    assert_eq!(side.error, TestError("B side failed".to_string()));
}

#[test]
fn error_side_into_error() {
    let error = create_a_error_with_unevaluated();
    let side = error.unwrap_a();
    let inner = side.into_error();
    assert_eq!(inner, TestError("A side failed".to_string()));
}

#[test]
fn error_side_into_parts_with_unevaluated() {
    let error = create_a_error_with_unevaluated();
    let side = error.unwrap_a();
    let parts = side.into_data();

    assert_eq!(parts.error, TestError("A side failed".to_string()));
    assert_eq!(parts.unevaluated, Some(20));
    assert_eq!(parts.remaining.collect::<Vec<_>>(), vec![(3, 30), (4, 40), (5, 50)]);
}

#[test]
fn error_side_into_parts_without_unevaluated() {
    let error = create_a_error_without_unevaluated();
    let side = error.unwrap_a();
    let parts = side.into_data();

    assert_eq!(parts.error, TestError("A side failed".to_string()));
    assert_eq!(parts.unevaluated, None);
    assert_eq!(parts.remaining.collect::<Vec<_>>(), vec![(3, 30), (4, 40), (5, 50)]);
}

#[test]
fn error_trait_source_a() {
    use std::error::Error;

    let err = create_a_error_with_unevaluated();
    let source = err.source().expect("Should have error source");
    source.is::<TestError>().expect_true("Should have TestError source");
}

#[test]
fn error_trait_source_b() {
    use std::error::Error;

    let err = create_b_error_with_unevaluated();
    let source = err.source().expect("Should have error source");
    source.is::<TestError>().expect_true("Should have TestError source");
}
