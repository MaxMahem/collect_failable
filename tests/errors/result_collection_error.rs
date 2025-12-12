use collect_failable::ResultIterError;
use std::collections::HashSet;

use crate::test_macros::{test_format, TestError};

type Collection = HashSet<u32>;

fn create_err() -> ResultIterError<TestError, Collection, TestError, std::option::IntoIter<u32>> {
    let collected = HashSet::from([1, 2, 3]);
    ResultIterError::new(TestError::new("iter error"), Ok(collected), None)
}

fn create_err_collection() -> ResultIterError<TestError, Collection, TestError, std::option::IntoIter<u32>> {
    ResultIterError::new(TestError::new("iter error"), Err(TestError::new("collection error")), None)
}

const EXPECTED_DISPLAY_OK: &str = "Iterator error: Test error: iter error";
const EXPECTED_DISPLAY_ERR: &str =
    "Iterator error: Test error: iter error; Collection error: Test error: collection error";

test_format!(display_format_ok, create_err(), "{}", EXPECTED_DISPLAY_OK);
test_format!(display_format_err, create_err_collection(), "{}", EXPECTED_DISPLAY_ERR);

#[test]
fn into_iteration_error() {
    let error = create_err().into_iteration_error();
    assert_eq!(error, TestError::new("iter error"));
}

#[test]
fn into_collection_result_ok() {
    let result = create_err().into_collection_result();
    assert_eq!(result, Ok(HashSet::from([1, 2, 3])));
}

#[test]
fn into_collection_result_err() {
    let result = create_err_collection().into_collection_result();
    assert_eq!(result, Err(TestError::new("collection error")));
}

#[test]
fn into_parts_ok() {
    let parts = create_err().into_parts();
    assert_eq!(parts.iteration_error, TestError::new("iter error"));
    assert_eq!(parts.collection_result, Ok(HashSet::from([1, 2, 3])));
}

#[test]
fn into_parts_err() {
    let parts = create_err_collection().into_parts();
    assert_eq!(parts.iteration_error, TestError::new("iter error"));
    assert_eq!(parts.collection_result, Err(TestError::new("collection error")));
}

#[test]
fn error_trait_source() {
    use std::error::Error;

    let error = create_err();
    let source = error.source().expect("Should have error source");
    assert!(source.is::<TestError>());
}
