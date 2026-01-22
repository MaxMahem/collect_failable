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

pub(crate) use into_iterator;
pub(crate) use test_ctor;
pub(crate) use test_format;
pub(crate) use test_source;
