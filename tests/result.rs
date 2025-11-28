#[test]
fn try_from_iter_result_success() {
    use collect_failable::TryFromIterator;

    let input: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let result: Result<Result<[i32; 3], _>, &str> = Result::try_from_iter(input);
    assert_eq!(result, Ok(Ok([1, 2, 3])));
}

#[test]
fn try_from_iter_result_iter_failure_example() {
    use collect_failable::TryFromIterator;

    let input: Vec<Result<i32, &str>> = vec![Ok(1), Err("oops"), Ok(3)];
    let result: Result<Result<[i32; 3], _>, &str> = Result::try_from_iter(input);
    assert_eq!(result, Err("oops"));
}

#[test]
fn try_from_iter_result_container_failure_example() {
    use collect_failable::{KeyCollision, TryFromIterator};
    use std::collections::HashMap;

    let input: Vec<Result<(i32, i32), &str>> = vec![Ok((1, 2)), Ok((1, 3))];
    let result: Result<Result<HashMap<i32, i32>, _>, &str> = Result::try_from_iter(input);
    assert_eq!(result, Ok(Err(KeyCollision { key: 1 })));
}
