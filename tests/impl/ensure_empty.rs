use collect_failable::impls::EnsureEmpty;

#[test]
fn test_ensure_empty_empty() {
    let iter = std::iter::empty::<i32>();
    assert!(iter.ensure_empty().is_ok());
}

#[test]
fn test_ensure_empty_not_empty() {
    let iter = std::iter::once(1);
    let result = iter.ensure_empty();
    assert!(result.is_err());

    let collect_failable::impls::NotEmpty { mut iter, item } = result.unwrap_err();

    assert_eq!(item, 1);
    assert!(iter.next().is_none());
}
