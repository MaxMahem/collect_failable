use std::marker::PhantomData;

use collect_failable::errors::ErrorItemProvider;

/// Simple error type for testing with identity
#[derive(PartialEq, Eq, thiserror::Error)]
#[error("Test error: {identity}")]
pub(crate) struct TestError<T = ()> {
    pub identity: &'static str,
    _phantom: PhantomData<T>,
}

impl<T> std::fmt::Debug for TestError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TestError").field(&self.identity).finish()
    }
}

impl<T> TestError<T> {
    pub const fn new(identity: &'static str) -> Self {
        Self { identity, _phantom: PhantomData }
    }
}

impl<T> ErrorItemProvider for TestError<T> {
    type Item = T;

    fn into_item(self) -> Option<Self::Item> {
        None
    }

    fn item(&self) -> Option<&Self::Item> {
        None
    }
}

/// Test that a formatted output (Debug/Display) matches expected value
macro_rules! test_format {
    ($name:ident, $setup:expr, $format:literal, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!(format!($format, $setup), $expected);
        }
    };
}

/// Test that an error's source() method returns the expected error type
///
/// - `test_source!(test_name, create_error(), ExpectedSourceType);`
macro_rules! test_source {
    ($name:ident, $setup:expr, $source_type:ty) => {
        #[test]
        fn $name() {
            use std::error::Error;

            let error = $setup;
            let source = error.source().expect("Should have error source");
            assert!(source.is::<$source_type>());
        }
    };
}

/// Test that a constructor produces an object with expected field values
///
/// Usage:
/// - `test_ctor!(test_name, constructor_expr, field1 => expected1, field2 => expected2);`
macro_rules! test_ctor {
    ($test_name:ident, $ctor:expr, $( $field:ident => $expected:expr ),+ $(,)?) => {
        #[test]
        fn $test_name() {
            let value = $ctor;
            $(
                assert_eq!(value.$field, $expected);
            )+
        }
    };
}

/// Test that a failable operation (Result) produces the expected Ok or Err variant
///
/// Usage:
/// - `test_failable!(test_name, expr, Ok);`
/// - `test_failable!(test_name, expr, Err field1 => expected1, field2 => expected2);`
macro_rules! test_failable {
    ($test_name:ident, $ctor:expr, Ok) => {
        #[test]
        fn $test_name() {
            $ctor.expect("should be Ok");
        }
    };
    ($test_name:ident, $ctor:expr, $( $field:ident => $expected:expr ),+ $(,)?) => {
        #[test]
        fn $test_name() {
            let err = $ctor.expect_err("should be Err");
            $(
                assert_eq!(err.$field, $expected);
            )+
        }
    };
}

/// Test that an error contains a specific item (or None)
///
/// Usage: `test_item_present!(test_name, error_expression, expected_option);`
macro_rules! test_item_present {
    ($name:ident, $ctor:expr, $expected:expr) => {
        #[test]
        fn $name() {
            use collect_failable::errors::ErrorItemProvider;
            let error = $ctor;
            let expected: Option<_> = $expected;
            assert_eq!(error.item(), expected.as_ref());
            assert_eq!(error.into_item(), expected);
        }
    };
}

pub(crate) use test_ctor;
pub(crate) use test_failable;
pub(crate) use test_format;
pub(crate) use test_item_present;
pub(crate) use test_source;
