#[test]
fn test_try_collect_ex() {
    use collect_failable::TryCollectEx;

    let input = vec![1, 2, 3];
    let result: Result<[i32; 3], _> = input.into_iter().try_collect_ex();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [1, 2, 3]);
}
