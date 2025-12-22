use collect_failable::errors::ResultCollectionError;
use std::collections::HashSet;

use crate::error_tests::{test_deref, test_format, test_source, TestError};

type Collection = HashSet<u32>;

fn create_err() -> ResultCollectionError<TestError, Collection, TestError, std::iter::Empty<u32>> {
    let collected = HashSet::from([1, 2, 3]);
    ResultCollectionError::new(TestError::new("iter error"), Ok(collected), std::iter::empty())
}

fn create_err_collection() -> ResultCollectionError<TestError, Collection, TestError, std::iter::Empty<u32>> {
    ResultCollectionError::new(TestError::new("iter error"), Err(TestError::new("collection error")), std::iter::empty())
}

const EXPECTED_DISPLAY_OK: &str = "Iterator error: Test error: iter error";
const EXPECTED_DISPLAY_ERR: &str = "Iterator error: Test error: iter error; Collection error: Test error: collection error";
const EXPECTED_DEBUG: &str =
    "ResultCollectionError { error: TestError { identity: \"iter error\" }, result: Ok(...), iter: \"core::iter::sources::empty::Empty<u32>\" }";

test_format!(display_format_ok, create_err(), "{}", EXPECTED_DISPLAY_OK);
test_format!(display_format_err, create_err_collection(), "{}", EXPECTED_DISPLAY_ERR);
test_format!(debug_format_ok, create_err(), "{:?}", EXPECTED_DEBUG);

#[test]
fn into_data() {
    let data = create_err().into_data();
    assert_eq!(data.error, TestError::new("iter error"));
    assert_eq!(data.result, Ok(HashSet::from([1, 2, 3])));
}

test_deref!(deref_error, create_err(), error, TestError::new("iter error"));

test_source!(error_trait_source, create_err(), TestError);
