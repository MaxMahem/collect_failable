use collect_failable::errors::Collision;

use crate::error_tests::test_ctor;

test_ctor!(
    new,
    Collision::new(42),
    item => 42
);

#[test]
fn error_item_provider() {
    use collect_failable::errors::ErrorItemProvider;

    let error = Collision::new(42);

    assert_eq!(error.item(), Some(&42));
    assert_eq!(error.into_item(), Some(42));
}
