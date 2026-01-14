use collect_failable::errors::TupleExtensionError;
use collect_failable::TryExtendOne;

use super::error_tests::{test_format, test_source, TestError};

const SIDE_A_ERROR: TestError = TestError::new("A side failed");
const SIDE_B_ERROR: TestError = TestError::new("B side failed");

// Wrapper type to allow using TestError in tests
#[derive(Debug)]
#[allow(dead_code)]
struct TestColl;

impl TryExtendOne for TestColl {
    type Item = u32;
    type Error = TestError;

    fn try_extend_one(&mut self, _item: u32) -> Result<(), TestError> {
        // Not actually used in error construction tests
        unimplemented!()
    }
}

fn create_a_error_with_unevaluated() -> TupleExtensionError<TestColl, TestColl, std::array::IntoIter<(u32, u32), 3>> {
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_a(SIDE_A_ERROR, Some(20), remaining.into_iter())
}

fn create_a_error_without_unevaluated() -> TupleExtensionError<TestColl, TestColl, std::array::IntoIter<(u32, u32), 3>> {
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_a(SIDE_A_ERROR, None, remaining.into_iter())
}

fn create_b_error_with_unevaluated() -> TupleExtensionError<TestColl, TestColl, std::array::IntoIter<(u32, u32), 3>> {
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_b(SIDE_B_ERROR, Some(10), remaining.into_iter())
}

fn create_b_error_without_unevaluated() -> TupleExtensionError<TestColl, TestColl, std::array::IntoIter<(u32, u32), 3>> {
    let remaining = [(3, 30), (4, 40), (5, 50)];
    TupleExtensionError::new_b(SIDE_B_ERROR, None, remaining.into_iter())
}

const EXPECTED_DEBUG_A_WITH: &str = r#"TupleExtensionError { side: Left(Side { error: TestError { identity: "A side failed", _phantom: PhantomData<()> }, unevaluated: Some(..) }), remaining: "core::array::iter::IntoIter<(u32, u32), 3>" }"#;
const EXPECTED_DEBUG_A_WITHOUT: &str = r#"TupleExtensionError { side: Left(Side { error: TestError { identity: "A side failed", _phantom: PhantomData<()> }, unevaluated: None }), remaining: "core::array::iter::IntoIter<(u32, u32), 3>" }"#;
const EXPECTED_DEBUG_B_WITH: &str = r#"TupleExtensionError { side: Right(Side { error: TestError { identity: "B side failed", _phantom: PhantomData<()> }, unevaluated: Some(..) }), remaining: "core::array::iter::IntoIter<(u32, u32), 3>" }"#;
const EXPECTED_DEBUG_B_WITHOUT: &str = r#"TupleExtensionError { side: Right(Side { error: TestError { identity: "B side failed", _phantom: PhantomData<()> }, unevaluated: None }), remaining: "core::array::iter::IntoIter<(u32, u32), 3>" }"#;
const EXPECTED_DISPLAY_A: &str = "Failed while extending first collection: Test error: A side failed";
const EXPECTED_DISPLAY_B: &str = "Failed while extending second collection: Test error: B side failed";

// Test Debug formatting
test_format!(debug_a_with_unevaluated, create_a_error_with_unevaluated(), "{:?}", EXPECTED_DEBUG_A_WITH);
test_format!(debug_a_without_unevaluated, create_a_error_without_unevaluated(), "{:?}", EXPECTED_DEBUG_A_WITHOUT);
test_format!(debug_b_with_unevaluated, create_b_error_with_unevaluated(), "{:?}", EXPECTED_DEBUG_B_WITH);
test_format!(debug_b_without_unevaluated, create_b_error_without_unevaluated(), "{:?}", EXPECTED_DEBUG_B_WITHOUT);

// Test Display formatting
test_format!(display_a, create_a_error_with_unevaluated(), "{}", EXPECTED_DISPLAY_A);
test_format!(display_b, create_b_error_with_unevaluated(), "{}", EXPECTED_DISPLAY_B);

// Test into_data method
#[test]
fn into_data() {
    let error = create_a_error_with_unevaluated();
    let data = error.into_data();

    assert!(data.side.is_left());
    assert_eq!(data.remaining.collect::<Vec<_>>(), vec![(3, 30), (4, 40), (5, 50)]);
}

#[test]
fn deref_field_access() {
    let error = create_a_error_with_unevaluated();
    assert!(error.side.is_left());
}

test_source!(error_trait_source_a, create_a_error_with_unevaluated(), TestError);
test_source!(error_trait_source_b, create_b_error_with_unevaluated(), TestError);
