use crate::error_tests::{TestError, test_source};

use collect_failable::errors::TupleExtendError;

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"TupleExtendError { error: TestError("test"), pending: i32, remaining: RangeInclusive<i32> }"#;
    const EXPECTED_DISPLAY: &str = "Error while extending collection: Test error: test";
    const EXPECTED_DEBUG_DATA: &str =
        r#"TupleExtendErrorData { error: TestError("test"), pending: i32, remaining: RangeInclusive<i32> }"#;

    test_format!(debug, TupleExtendError::new(TestError::<i32>::new("test"), Some(20), 3..=5), "{:?}", EXPECTED_DEBUG);

    test_format!(display, TupleExtendError::new(TestError::<i32>::new("test"), Some(20), 3..=5), "{}", EXPECTED_DISPLAY);

    test_format!(
        debug_data,
        TupleExtendError::new(TestError::<i32>::new("test"), Some(20), 3..=5).into_data(),
        "{:?}",
        EXPECTED_DEBUG_DATA
    );
}

mod ctors {
    use super::*;
    use crate::error_tests::test_ctor;

    test_ctor!(
        new,
        TupleExtendError::new(TestError::<i32>::new("test"), Some(20), 3..=5),
        error => TestError::new("test"),
        pending => Some(20),
        remaining => 3..=5
    );

    test_ctor!(
        into_data,
        TupleExtendError::new(TestError::<i32>::new("test"), Some(20), 3..=5).into_data(),
        error => TestError::new("test"),
        pending => Some(20),
        remaining => 3..=5
    );
}

test_source!(source, TupleExtendError::new(TestError::<i32>::new("test"), Some(20), 3..=5), TestError::<i32>);
