use collect_failable::errors::collision::Collision;

crate::error_tests::test_ctor!(
    new,
    Collision::new(42),
    item => 42
);

crate::error_tests::test_item_present!(error_item_provider, Collision::new(42), Some(42));
