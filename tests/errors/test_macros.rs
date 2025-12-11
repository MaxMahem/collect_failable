use thiserror::Error;

/// Simple error type for testing with identity
#[derive(Debug, Clone, PartialEq, Eq, Error, derive_more::Constructor)]
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

/// Test that an expect method panics with the expected message on wrong variant
///
/// - `expect_panic!(test_name, ctor(), method, "panic message");`
macro_rules! expect_panic {
    ($name:ident, $setup:expr, $method:ident, $msg:expr) => {
        #[test]
        #[should_panic(expected = $msg)]
        fn $name() {
            _ = $setup.$method($msg);
        }
    };
}

/// Test any getter (method call or field access) against an expected value
///
/// - `getter!(len, create_error(), len(), 5);`
macro_rules! getter {
    // Method call with parentheses
    ($name:ident, $setup:expr, $method:ident(), $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($setup.$method(), $expected);
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

pub(crate) use expect_panic;
pub(crate) use getter;
pub(crate) use into_iterator;
pub(crate) use test_format;
