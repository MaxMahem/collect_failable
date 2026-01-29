use crate::error_tests::{TestError, test_source};
use crate::utils::panics;

use collect_failable::errors::ExtendError;
use collect_failable::errors::capacity::CapacityError;
use collect_failable::errors::types::SizeHint;

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"ExtendError { iter: RangeInclusive<i32>, error: TestError("test") }"#;
    const EXPECTED_DISPLAY: &str = "Extension Error: Test error: test";
    const EXPECTED_DEBUG_DATA: &str = r#"ExtendErrorData { iter: RangeInclusive<i32>, error: TestError("test") }"#;

    test_format!(debug, ExtendError::new(1..=2, TestError::<i32>::new("test")), "{:?}", EXPECTED_DEBUG);
    test_format!(display, ExtendError::new(1..=2, TestError::<i32>::new("test")), "{}", EXPECTED_DISPLAY);
    test_format!(debug_data, ExtendError::new(1..=2, TestError::<i32>::new("test")).into_data(), "{:?}", EXPECTED_DEBUG_DATA);
}

mod ctors {
    use super::*;
    use collect_failable::errors::capacity::CapacityError;
    use collect_failable::errors::types::SizeHint;
    use size_hinter::InvalidIterator;

    use crate::error_tests::test_ctor;

    test_ctor!(
        new,
        ExtendError::new(1..=2, TestError::<i32>::new("test")),
        iter => 1..=2,
        error => TestError::new("test")
    );

    test_ctor!(
        bounds,
        ExtendError::<_, CapacityError<i32>>::bounds(1..=2, SizeHint::exact(5)),
        iter => 1..=2,
        error => CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(2))
    );

    test_ctor!(
        overflow,
        ExtendError::overflow(3..=4, 99),
        iter => 3..=4,
        error => CapacityError::overflow(SizeHint::ZERO, 99)
    );

    test_ctor!(
        into_data,
        ExtendError::new(1..=2, TestError::<i32>::new("test")).into_data(),
        iter => 1..=2,
        error => TestError::new("test")
    );

    panics!(
        panic_bounds,
        ExtendError::<_, CapacityError<i32>>::bounds(InvalidIterator::<i32>::DEFAULT, SizeHint::exact(5)),
        "Invalid size hint"
    );
}

test_source!(source, ExtendError::new(1..=2, TestError::<i32>::new("test")), TestError<i32>);

#[test]
fn into_iter() {
    let items: Vec<_> = ExtendError::overflow(1..=2, 99).into_iter().collect();

    assert_unordered::assert_eq_unordered!(items, vec![99, 1, 2]);
}

mod ensure_fits_into {
    use super::*;
    use crate::error_tests::test_failable;
    use arrayvec::ArrayVec;

    test_failable!(pass, ExtendError::ensure_fits_into(1..=5, &ArrayVec::<i32, 5>::new()), Ok);

    test_failable!(
        fail,
        ExtendError::ensure_fits_into(1..=6, &ArrayVec::<i32, 5>::new()),
        iter => 1..=6,
        error => CapacityError::bounds(SizeHint::at_most(5), SizeHint::exact(6))
    );

    panics!(
        panic,
        ExtendError::ensure_fits_into(size_hinter::InvalidIterator::<i32>::DEFAULT, &ArrayVec::<i32, 5>::new()),
        "Invalid size hint"
    );
}
