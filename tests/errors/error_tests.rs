use thiserror::Error;

/// Simple error type for testing with identity
#[derive(Debug, PartialEq, Eq, Error, derive_more::Constructor)]
#[error("Test error: {identity}")]
pub(crate) struct TestError {
    pub identity: &'static str,
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

/// Test that into_iterator produces the expected items (order-independent for HashSet)
macro_rules! into_iterator {
    ($name:ident, $setup:expr, expected_len = $len:expr, contains = [$($item:expr),* $(,)?]) => {
        #[test]
        fn $name() {
            let items: Vec<_> = $setup.into_iter().collect();

            assert_eq!(items.len(), $len);
            $(
                assert!(items.contains(&$item));
            )*
        }
    };
}

/// Test that a constructor produces the expected value
///
/// - `identity!(test_name, ctor_call(), expected_value);`
/// - `identity!(test_name, ctor_call(), panics: "panic message");`
macro_rules! identity {
    // Success case: constructor should equal expected value
    ($name:ident, $ctor:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($ctor, $expected);
        }
    };

    // Panic case: constructor should panic with expected message
    ($name:ident, $ctor:expr, panics: $msg:expr) => {
        #[test]
        #[should_panic(expected = $msg)]
        fn $name() {
            let _ = $ctor;
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

pub(crate) use identity;
pub(crate) use into_iterator;
pub(crate) use test_format;
pub(crate) use test_source;
