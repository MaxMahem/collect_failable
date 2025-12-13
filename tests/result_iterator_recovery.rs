use collect_failable::{ResultIterError, TryFromIterator};

#[test]
fn can_recover_iterator_from_result_iter_error() {
    let input: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Err("error at 3"), Ok(4), Ok(5)];
    let result: Result<Result<[i32; 5], _>, ResultIterError<_, _, _, _>> = Result::try_from_iter(input);

    // Get the error
    let err = result.unwrap_err();

    // Check that we got an error
    assert_eq!(err.iteration_error, "error at 3");

    // Recover the remaining iterator
    let remaining_iter = err.into_result_iter();

    // The remaining iterator should contain Ok(4) and Ok(5)
    let remaining: Vec<_> = remaining_iter.collect();
    assert_eq!(remaining, vec![Ok(4), Ok(5)]);
}

#[test]
fn iterator_is_consumed_when_no_error() {
    let input: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let result: Result<Result<[i32; 3], _>, ResultIterError<_, _, _, _>> = Result::try_from_iter(input);

    // This should succeed
    assert!(result.is_ok());
}
