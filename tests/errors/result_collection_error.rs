use collect_failable::errors::ResultCollectionError;
use std::collections::HashSet;

use crate::error_tests::{TestError, test_ctor, test_format, test_source};

test_ctor!(
    new,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Ok::<_, ()>(HashSet::from([1, 2, 3])), 0..0),
    error => TestError::new("iter error"),
    result => Ok(HashSet::from([1, 2, 3])),
    iter => 0..0
);

test_ctor!(
    into_data,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Ok::<_, ()>(HashSet::from([1, 2, 3])), 0..0).into_data(),
    error => TestError::new("iter error"),
    result => Ok(HashSet::from([1, 2, 3])),
    iter => 0..0
);

const EXPECTED_DISPLAY_OK: &str = "Iterator error: Test error: iter error";
const EXPECTED_DISPLAY_ERR: &str = "Iterator error: Test error: iter error; Collection error: Test error: collection error";
const EXPECTED_DEBUG: &str =
    "ResultCollectionError { error: TestError(\"iter error\"), result: Ok(HashSet<i32>), iter: Range<i32> }";
const EXPECTED_DEBUG_DATA: &str =
    "ResultCollectionErrorData { error: TestError(\"iter error\"), result: Ok(HashSet<i32>), iter: Range<i32> }";

test_format!(
    display_format_ok,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Ok::<_, i32>(HashSet::from([1, 2, 3])), 0..0),
    "{}",
    EXPECTED_DISPLAY_OK
);
test_format!(
    display_format_err,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Err::<(), _>(TestError::<()>::new("collection error")), 0..0),
    "{}",
    EXPECTED_DISPLAY_ERR
);

test_format!(
    debug_format_ok,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Ok::<_, ()>(HashSet::from([1, 2, 3])), 0..0),
    "{:?}",
    EXPECTED_DEBUG
);

test_format!(
    debug_format_data,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Ok::<_, ()>(HashSet::from([1, 2, 3])), 0..0).into_data(),
    "{:?}",
    EXPECTED_DEBUG_DATA
);

test_source!(
    error_trait_source,
    ResultCollectionError::new(TestError::<()>::new("iter error"), Ok::<_, TestError::<()>>(HashSet::from([1, 2, 3])), 0..0),
    TestError::<()>
);
