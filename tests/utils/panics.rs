/// Test that an expression panics with a specific message
///
/// Usage: `panics!(test_name, expression, "expected panic message");`
macro_rules! panics {
    ($name:ident, $expression:expr, $message:literal) => {
        #[test]
        #[should_panic(expected = $message)]
        fn $name() {
            _ = $expression;
        }
    };
}
pub(crate) use panics;
