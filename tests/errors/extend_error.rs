use std::ops::Range;

use collect_failable::errors::ExtendError;
use collect_failable::errors::capacity::CapacityError;
use collect_failable::errors::types::SizeHint;

use crate::error_tests::{INVALID_ITER, TEST_ERROR, TestError, test_into_iter, test_source};
use crate::utils::panics;

const ITER: Range<i32> = 1..3;
const REMAIN_HINT: SizeHint = SizeHint::exact(2);
const CAP: SizeHint = SizeHint::exact(5);

const OVERFLOW_VALUE: i32 = 99;

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"ExtendError { remain: core::ops::range::Range<i32>, error: TestError("test") }"#;
    const EXPECTED_DISPLAY: &str = "Extension Error: Test error: test";
    const EXPECTED_DEBUG_DATA: &str = r#"ExtendErrorData { remain: core::ops::range::Range<i32>, error: TestError("test") }"#;

    test_format!(debug, ExtendError::new(ITER, TEST_ERROR), "{:?}", EXPECTED_DEBUG);
    test_format!(display, ExtendError::new(ITER, TEST_ERROR), "{}", EXPECTED_DISPLAY);
    test_format!(debug_data, ExtendError::new(ITER, TEST_ERROR).into_data(), "{:?}", EXPECTED_DEBUG_DATA);
    test_format!(display_data, ExtendError::new(ITER, TEST_ERROR).into_data(), "{}", EXPECTED_DISPLAY);
}

mod ctors {
    use super::*;

    use crate::error_tests::test_ctor;

    test_ctor!(
        new,
        ExtendError::new(ITER, TEST_ERROR),
        remain => ITER,
        error => TEST_ERROR
    );

    test_ctor!(
        bounds,
        ExtendError::<_, CapacityError<i32>>::bounds(ITER, CAP),
        remain => ITER,
        error => CapacityError::bounds(CAP, REMAIN_HINT)
    );

    panics!(panic_bounds, ExtendError::<_, CapacityError<i32>>::bounds(INVALID_ITER, CAP), "Invalid size hint");

    test_ctor!(
        overflow,
        ExtendError::overflow(ITER, OVERFLOW_VALUE),
        remain => ITER,
        error => CapacityError::overflow(SizeHint::ZERO, OVERFLOW_VALUE)
    );

    test_ctor!(
        overflow_remaining_cap,
        ExtendError::overflow_remaining_cap(ITER, OVERFLOW_VALUE, &arrayvec::ArrayVec::<i32, 5>::new()),
        remain => ITER,
        error => CapacityError::overflow(SizeHint::at_most(5), OVERFLOW_VALUE)
    );

    test_ctor!(
        overflow_remaining_cap_array,
        ExtendError::overflow_remaining_cap(ITER, OVERFLOW_VALUE, &[1, 2, 3]),
        remain => ITER,
        error => CapacityError::overflow(SizeHint::ZERO, OVERFLOW_VALUE)
    );

    test_ctor!(
        collision,
        ExtendError::collision(ITER, OVERFLOW_VALUE),
        remain => ITER,
        error => collect_failable::errors::collision::Collision::new(OVERFLOW_VALUE)
    );

    test_ctor!(
        into_data,
        ExtendError::new(ITER, TEST_ERROR).into_data(),
        remain => ITER,
        error => TEST_ERROR
    );
}

test_source!(source, ExtendError::new(ITER, TEST_ERROR), TestError<i32>);
test_source!(source_data, ExtendError::new(ITER, TEST_ERROR).into_data(), TestError<i32>);

test_into_iter!(into_iter, ExtendError::overflow(ITER, OVERFLOW_VALUE), vec![OVERFLOW_VALUE, 1, 2]);
test_into_iter!(into_iter_data, ExtendError::overflow(ITER, OVERFLOW_VALUE).into_data(), vec![OVERFLOW_VALUE, 1, 2]);

mod ensure_fits_into {
    use super::*;
    use crate::error_tests::test_ctor;

    type ArrayVec = arrayvec::ArrayVec<i32, 5>;

    test_ctor!(pass, ExtendError::ensure_fits_into(ITER, &ArrayVec::new()).expect("should be Ok"));

    test_ctor!(
        fail,
        ExtendError::ensure_fits_into(1..=6, &ArrayVec::new()).expect_err("should be Err"),
        remain => 1..=6,
        error => CapacityError::bounds(SizeHint::at_most(5), SizeHint::exact(6))
    );

    panics!(panic, ExtendError::ensure_fits_into(INVALID_ITER, &ArrayVec::new()), "Invalid size hint");
}
