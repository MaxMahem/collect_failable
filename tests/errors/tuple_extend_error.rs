use std::ops::RangeInclusive;

use collect_failable::errors::TupleExtendError;

use crate::error_tests::{TEST_ERROR, TestError, test_source};

const PENDING_VALUE: Option<i32> = Some(20);
const REMAIN_ITER: RangeInclusive<i32> = 3..=5;

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str =
        r#"TupleExtendError { error: TestError("test"), pending: i32, remaining: core::ops::range::RangeInclusive<i32> }"#;
    const EXPECTED_DISPLAY: &str = "Error while extending collection: Test error: test";
    const EXPECTED_DEBUG_DATA: &str =
        r#"TupleExtendErrorData { error: TestError("test"), pending: i32, remaining: core::ops::range::RangeInclusive<i32> }"#;

    test_format!(debug, TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER), "{:?}", EXPECTED_DEBUG);

    test_format!(display, TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER), "{}", EXPECTED_DISPLAY);

    test_format!(
        debug_data,
        TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER).into_data(),
        "{:?}",
        EXPECTED_DEBUG_DATA
    );

    test_format!(display_data, TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER).into_data(), "{}", EXPECTED_DISPLAY);
}

mod ctors {
    use super::*;
    use crate::error_tests::test_ctor;

    test_ctor!(
        new,
        TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER),
        error => TEST_ERROR,
        pending => PENDING_VALUE,
        remaining => REMAIN_ITER
    );

    test_ctor!(
        into_data,
        TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER).into_data(),
        error => TEST_ERROR,
        pending => PENDING_VALUE,
        remaining => REMAIN_ITER
    );
}

test_source!(source, TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER), TestError::<i32>);
test_source!(source_data, TupleExtendError::new(TEST_ERROR, PENDING_VALUE, REMAIN_ITER).into_data(), TestError::<i32>);
