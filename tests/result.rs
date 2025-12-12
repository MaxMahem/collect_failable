use std::collections::HashMap;

#[test]
fn try_from_iter_result_success() {
    use collect_failable::TryFromIterator;

    let input: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let result: Result<Result<[i32; 3], _>, _> = Result::try_from_iter(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Ok([1, 2, 3]));
}

#[test]
fn try_from_iter_result_iter_failure_example() {
    use collect_failable::TryFromIterator;

    let input: Vec<Result<i32, &str>> = vec![Ok(1), Err("oops"), Ok(3)];
    let result = Result::<[i32; 3], _>::try_from_iter(input);
    match result {
        Ok(Ok(ok)) => panic!("should have failed: {ok:?}"),
        Ok(Err(err)) => panic!("should be iter failure: {err:?}"),
        Err(e) => assert!(e.collection_result.is_err()),
    }
}

#[test]
fn try_from_iter_result_container_failure_example() {
    use collect_failable::TryFromIterator;

    let input: Vec<Result<(i32, i32), &str>> = vec![Ok((1, 2)), Ok((1, 3))];
    // Test that a collision in the inner container causes the inner Result to be Err
    let result: Result<Result<HashMap<i32, i32>, _>, _> = Result::try_from_iter(input);
    match result {
        Ok(Err(_err)) => {} // This is the expected case - collision in the hashmap
        Ok(Ok(ok)) => panic!("inner result should be Err due to collision: {ok:?}"),
        Err(e) => panic!("outer result should be Ok, got Err: {e:?}"),
    }
}
