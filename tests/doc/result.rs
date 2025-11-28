#[test]
fn try_from_iter_result_example() {
    use collect_failable::{TryCollectEx, TryFromIterator, ValueCollision};
    use std::collections::HashSet;

    // container error type can be inferred
    let iter: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let result: Result<Result<HashSet<i32>, _>, _> = Result::try_from_iter(iter);
    assert_eq!(result, Ok(Ok(HashSet::from([1, 2, 3]))));

    // Short-circuiting on the first error
    let data: Vec<Result<i32, &str>> = vec![Ok(1), Err("oops"), Ok(3)];
    let result: Result<Result<HashSet<i32>, _>, _> = data.into_iter().try_collect_ex();
    assert_eq!(result, Err("oops"));

    // Construction of a container can also fail
    let data: Vec<Result<i32, &str>> = vec![Ok(1), Ok(1), Ok(3)];
    let result: Result<Result<HashSet<i32>, _>, _> = data.into_iter().try_collect_ex();
    assert_eq!(result, Ok(Err(ValueCollision::new(1))));
}

/// Example showing the use of the `??` operator for handling nested Result types.
///
/// The `??` operator simplifies handling of nested Results by automatically converting
/// both the outer and inner errors using the `From` trait. This is the most ergonomic
/// approach when both error types can be converted to your function's return type.
#[test]
fn double_question_mark_example() {
    use collect_failable::TryCollectEx;
    use std::collections::HashSet;

    // Most errors can be converted to Box<dyn std::error::Error> using From
    fn process_data(data: Vec<Result<i32, &str>>) -> Result<HashSet<i32>, Box<dyn std::error::Error>> {
        let set = data.into_iter().try_collect_ex::<Result<HashSet<i32>, _>>()??;
        Ok(set)
    }

    // Success case
    let result = process_data(vec![Ok(1), Ok(2), Ok(3)]).expect("should be ok");
    assert_eq!(result, HashSet::from([1, 2, 3]));

    // Outer error (element error)
    let err = process_data(vec![Ok(1), Err("parse error"), Ok(3)]).expect_err("should be err");
    assert_eq!(err.to_string(), "parse error");

    // Inner error (container error)
    let err = process_data(vec![Ok(1), Ok(1), Ok(3)]).expect_err("should be err");
    assert_eq!(err.to_string(), "Value collision");
}

// TODO: Example showing the use of `flatten_err` for handling nested Result types.
// Note: Commented out until fluent_result module structure can be verified
/*
/// Example showing the use of `flatten_err` for handling nested Result types.
///
/// The `flatten_err` method from the `fluent_result` crate simplifies nested Results
/// by flattening `Result<Result<T, E1>, E2>` into `Result<T, Either<E2, E1>>`.
/// This preserves type information about which error occurred.
#[test]
fn flatten_err_example() {
    use collect_failable::{TryCollectEx, ValueCollision};
    use fluent_result::nested::FlattenErr;
    use fluent_result::either::Either;
    use std::collections::HashSet;

    fn process_data(data: Vec<Result<i32, &str>>) -> Result<HashSet<i32>, Either<&str, ValueCollision<i32>>> {
        // flatten_err converts Result<Result<HashSet, ValueCollision>, &str>
        // into Result<HashSet, Either<&str, ValueCollision>>
        data.into_iter().try_collect_ex::<Result<HashSet<i32>, _>>().flatten_err()
    }

    // Success case
    let result = process_data(vec![Ok(1), Ok(2), Ok(3)]);
    assert_eq!(result, Ok(HashSet::from([1, 2, 3])));

    // Outer error (element error) becomes Either::Left
    let result = process_data(vec![Ok(1), Err("parse error"), Ok(3)]);
    assert_eq!(result, Err(Either::Left("parse error")));

    // Inner error (container error) becomes Either::Right
    let result = process_data(vec![Ok(1), Ok(1), Ok(3)]);
    assert!(matches!(result, Err(Either::Right(_))));
    if let Err(Either::Right(collision)) = result {
        assert_eq!(collision.value, 1);
    }
}
*/

// TODO: Example showing the use of `box_err` for handling nested Result types.
// Note: Commented out until fluent_result module structure can be verified
/*
/// Example showing the use of `box_err` for handling nested Result types.
///
/// The `box_err` method from the `fluent_result` crate simplifies nested Results
/// by boxing errors, converting `Result<Result<T, E1>, E2>` into
/// `Result<T, Box<dyn Error>>`. This is useful when you don't care about the specific
/// error type and want a uniform error handling strategy.
#[test]
fn box_err_example() {
    use collect_failable::{TryCollectEx, ValueCollision};
    use fluent_result::nested::BoxErr;
    use std::collections::HashSet;
    use std::error::Error;

    // Both &str and ValueCollision need to implement Error for boxing
    impl std::fmt::Display for ValueCollision<i32> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Value collision: {}", self.value)
        }
    }

    impl Error for ValueCollision<i32> {}

    fn process_data(data: Vec<Result<i32, &str>>) -> Result<HashSet<i32>, Box<dyn Error>> {
        // box_err converts Result<Result<HashSet, ValueCollision>, &str>
        // into Result<HashSet, Box<dyn Error>>
        data.into_iter().try_collect_ex::<Result<HashSet<i32>, _>>().box_err()
    }

    // Success case
    let result = process_data(vec![Ok(1), Ok(2), Ok(3)]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), HashSet::from([1, 2, 3]]));

    // Element error is boxed
    let result = process_data(vec![Ok(1), Err("parse error"), Ok(3)]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("parse error"));

    // Container error is boxed
    let result = process_data(vec![Ok(1), Ok(1), Ok(3)]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("collision"));
}
*/
