use std::ops::Range;

use collect_failable::errors::CollectError;
use collect_failable::errors::capacity::{CapacityError, FixedCap};
use collect_failable::errors::collision::Collision;
use collect_failable::errors::types::SizeHint;

use crate::error_tests::{INVALID_ITER, TEST_ERROR, TestError, test_ctor, test_into_iter, test_source};
use crate::utils::panics;

const COLLECTED_LEN: usize = 2;
type Collection = [i32; COLLECTED_LEN];

const REMAIN_ITER: Range<i32> = 3..5;
const COLLECTED: Collection = [1, 2];
const COLLECTED_CAP: SizeHint = Collection::CAP;

const OVERFLOW_VALUE: i32 = 99;
const OVERFLOW_ERR: CapacityError<i32> = CapacityError::overflow(COLLECTED_CAP, OVERFLOW_VALUE);
const OVERFLOW_ERR_ZERO: CapacityError<i32> = CapacityError::overflow(SizeHint::ZERO, OVERFLOW_VALUE);

const UNDERFLOW_CAP: SizeHint = SizeHint::exact(5);
const UNDERFLOW_ERR: CapacityError<i32> = CapacityError::underflow(UNDERFLOW_CAP, COLLECTED_LEN);

const COLLISION_VALUE: i32 = 99;
const COLLISION_ERR: Collision<i32> = Collision::new(COLLISION_VALUE);

const ALL_VALUES: [i32; 4] = [1, 2, 3, 4];

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str =
        r#"CollectError { remain: core::ops::range::Range<i32>, collected: [i32; 2], error: TestError("test") }"#;
    const EXPECTED_DISPLAY: &str = "Collection Error: Test error: test";
    const EXPECTED_DEBUG_DATA: &str =
        r#"CollectErrorData { remain: core::ops::range::Range<i32>, collected: [i32; 2], error: TestError("test") }"#;

    test_format!(debug, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR), "{:?}", EXPECTED_DEBUG);
    test_format!(display, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR), "{}", EXPECTED_DISPLAY);
    test_format!(debug_data, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR).into_data(), "{:?}", EXPECTED_DEBUG_DATA);
    test_format!(display_data, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR).into_data(), "{}", EXPECTED_DISPLAY);
}

mod ctors {
    use super::*;

    test_ctor!(
        new,
        CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => TEST_ERROR
    );

    test_ctor!(
        bounds,
        CollectError::<_, Collection, _>::bounds(REMAIN_ITER, SizeHint::exact(5)),
        remain => REMAIN_ITER,
        collected => Collection::default(),
        error => CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(2))
    );

    test_ctor!(
        overflow,
        CollectError::overflow(REMAIN_ITER, COLLECTED, OVERFLOW_VALUE, COLLECTED_CAP),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => OVERFLOW_ERR
    );

    test_ctor!(
        collect_overflowed,
        CollectError::collect_overflow::<Collection>(REMAIN_ITER, COLLECTED, OVERFLOW_VALUE),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => OVERFLOW_ERR
    );

    test_ctor!(
        overflow_remaining_cap,
        CollectError::overflow_remaining_cap(REMAIN_ITER, COLLECTED, OVERFLOW_VALUE, &COLLECTED),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => OVERFLOW_ERR_ZERO
    );

    test_ctor!(underflow, CollectError::<_, Collection, _>::underflow(REMAIN_ITER, COLLECTED, UNDERFLOW_CAP),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => UNDERFLOW_ERR
    );

    test_ctor!(collision, CollectError::collision(REMAIN_ITER, COLLECTED, COLLISION_VALUE),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => COLLISION_ERR
    );

    test_ctor!(
        into_data,
        CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR).into_data(),
        remain => REMAIN_ITER,
        collected => COLLECTED,
        error => TEST_ERROR
    );

    panics!(panic_bounds, CollectError::<_, Collection, _>::bounds(INVALID_ITER, SizeHint::exact(5)), "Invalid size hint");
}

test_source!(source, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR), TestError<i32>);
test_source!(source_data, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR).into_data(), TestError<i32>);

test_into_iter!(into_iter, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR), ALL_VALUES.to_vec());
test_into_iter!(into_iter_data, CollectError::new(REMAIN_ITER, COLLECTED, TEST_ERROR).into_data(), ALL_VALUES.to_vec());

mod ensure_fits_in {
    use super::*;

    type ToLargeCollection = [i32; 5];
    const TO_LARGE_COLLECTION_CAP: SizeHint = ToLargeCollection::CAP;
    const BOUNDS_ERR: CapacityError<i32> = CapacityError::bounds(TO_LARGE_COLLECTION_CAP, COLLECTED_CAP);

    test_ctor!(pass, CollectError::<_, Collection, _>::ensure_fits_in::<Collection>(REMAIN_ITER).expect("should be Ok"));

    test_ctor!(
        fail,
        CollectError::<_, Collection, _>::ensure_fits_in::<ToLargeCollection>(REMAIN_ITER).expect_err("should be Err"),
        remain => REMAIN_ITER,
        collected => Collection::default(),
        error => BOUNDS_ERR
    );

    panics!(panic, CollectError::<_, Collection, _>::ensure_fits_in::<Collection>(INVALID_ITER), "Invalid size hint");
}

mod ensure_fits_into {
    use super::*;

    type ArrayVec = arrayvec::ArrayVec<i32, 5>;
    const ARRAYVEC_CAP: SizeHint = ArrayVec::CAP;

    const TO_LARGE_ITER: Range<i32> = 1..7;
    const TO_LARGE_ITER_CAP: SizeHint = SizeHint::exact(6);
    const BOUNDS_ERR: CapacityError<i32> = CapacityError::bounds(ARRAYVEC_CAP, TO_LARGE_ITER_CAP);

    test_ctor!(pass, CollectError::ensure_fits_into(REMAIN_ITER, &ArrayVec::new()).expect("should be Ok"));

    test_ctor!(
        fail,
        CollectError::ensure_fits_into(TO_LARGE_ITER, &ArrayVec::new()).expect_err("should be Err"),
        remain => TO_LARGE_ITER,
        collected => ArrayVec::new(),
        error => BOUNDS_ERR
    );

    panics!(panic, CollectError::ensure_fits_into(INVALID_ITER, &ArrayVec::new()), "Invalid size hint");
}
