use std::ops::Range;

use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind, FixedCap};
use collect_failable::errors::types::SizeHint;

const ARRAY_SIZE: usize = 5;

type Collection = [i32; ARRAY_SIZE];

const COLLECTION_CAP: SizeHint = <Collection>::CAP;
const DISJOINT_HINT: SizeHint = SizeHint::exact(4);
const BOUNDS_ERR_KIND: CapacityErrorKind<i32> = CapacityErrorKind::Bounds { hint: DISJOINT_HINT };

const OVERFLOW_VALUE: i32 = 42;
const OVERFLOW_ERR_KIND: CapacityErrorKind<i32> = CapacityErrorKind::Overflow { overflow: OVERFLOW_VALUE };

const UNDERFLOW_COUNT: usize = 4;
const UNDERFLOW_ERR_KIND: CapacityErrorKind<i32> = CapacityErrorKind::Underflow { count: UNDERFLOW_COUNT };

const OVERLAPPING_ITER: Range<i32> = 1..6;
const DISJOINT_ITER: Range<i32> = 1..5;

mod ctors {
    use super::*;
    use crate::error_tests::{INVALID_ITER, test_ctor};
    use crate::utils::panics;

    test_ctor!(
        bounds,
        CapacityError::<i32>::bounds(COLLECTION_CAP, DISJOINT_HINT),
        capacity => COLLECTION_CAP,
        kind => BOUNDS_ERR_KIND
    );

    test_ctor!(
        overflow,
        CapacityError::overflow(COLLECTION_CAP, OVERFLOW_VALUE),
        capacity => COLLECTION_CAP,
        kind => OVERFLOW_ERR_KIND
    );

    test_ctor!(
        underflow,
        CapacityError::<i32>::underflow(COLLECTION_CAP, UNDERFLOW_COUNT),
        capacity => COLLECTION_CAP,
        kind => UNDERFLOW_ERR_KIND
    );

    test_ctor!(
        from_arrayvec_capacity_error,
        CapacityError::from(arrayvec::CapacityError::new(OVERFLOW_VALUE)),
        capacity => SizeHint::ZERO,
        kind => OVERFLOW_ERR_KIND
    );

    mod ensure_fits {
        use super::*;
        use crate::error_tests::test_ctor;

        test_ctor!(pass, CapacityError::<i32>::ensure_fits(&OVERLAPPING_ITER, COLLECTION_CAP).expect("should be Ok"));
        test_ctor!(fail, CapacityError::<i32>::ensure_fits(&DISJOINT_ITER, COLLECTION_CAP).expect_err("should be Err"),
            capacity => COLLECTION_CAP,
            kind => BOUNDS_ERR_KIND
        );

        panics!(panic, CapacityError::<i32>::ensure_fits(&INVALID_ITER, COLLECTION_CAP), "Invalid size hint");
    }
}

mod error_item_provider {
    use super::*;
    use crate::error_tests::test_item_present;

    test_item_present!(present, CapacityError::overflow(COLLECTION_CAP, OVERFLOW_VALUE), Some(OVERFLOW_VALUE));
    test_item_present!(absent, CapacityError::<i32>::bounds(COLLECTION_CAP, DISJOINT_HINT), None);
}
