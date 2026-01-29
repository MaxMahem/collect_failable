use collect_failable::errors::{CapacityError, CapacityErrorKind, SizeHint};
use size_hinter::InvalidIterator;

use crate::error_tests::{test_ctor, test_failable};
use crate::utils::panics;

test_ctor!(
    bounds,
    CapacityError::<i32>::bounds(SizeHint::bounded(5, 10), SizeHint::bounded(11, 15)),
    capacity => SizeHint::bounded(5, 10),
    kind => CapacityErrorKind::Bounds { hint: SizeHint::bounded(11, 15) }
);

test_ctor!(
    overflow,
    CapacityError::overflow(SizeHint::bounded(5, 10), 42),
    capacity => SizeHint::bounded(5, 10),
    kind => CapacityErrorKind::Overflow { rejected: 42 }
);

test_ctor!(
    collect_overflowed,
    CapacityError::collect_overflowed::<[i32; 5]>(42),
    capacity => SizeHint::exact(5),
    kind => CapacityErrorKind::Overflow { rejected: 42 }
);

test_ctor!(
    extend_overflowed,
    CapacityError::extend_overflowed(42),
    capacity => SizeHint::ZERO,
    kind => CapacityErrorKind::Overflow { rejected: 42 }
);

test_ctor!(
    underflow,
    CapacityError::<i32>::underflow(SizeHint::bounded(5, 10), 4),
    capacity => SizeHint::bounded(5, 10),
    kind => CapacityErrorKind::Underflow { count: 4 }
);

test_ctor!(
    collect_underflowed,
    CapacityError::<i32>::collect_underflowed::<[i32; 5]>(4),
    capacity => SizeHint::exact(5),
    kind => CapacityErrorKind::Underflow { count: 4 }
);

mod error_item_provider {
    use super::*;
    use crate::error_tests::test_item_present;

    test_item_present!(present, CapacityError::overflow(SizeHint::bounded(5, 10), 42), Some(42));
    test_item_present!(absent, CapacityError::<i32>::bounds(SizeHint::bounded(5, 10), SizeHint::bounded(11, 15)), None);
}

mod ensure_fits {
    use super::*;

    test_failable!(pass, CapacityError::<i32>::ensure_fits(SizeHint::bounded(5, 10), &(1..7)), Ok);
    test_failable!(fail, CapacityError::<i32>::ensure_fits(SizeHint::bounded(5, 10), &(1..=11)),
        capacity => SizeHint::bounded(5, 10),
        kind => CapacityErrorKind::Bounds { hint: SizeHint::exact(11) }
    );

    panics!(
        panic,
        CapacityError::<i32>::ensure_fits(SizeHint::bounded(5, 10), &InvalidIterator::<i32>::DEFAULT),
        "Invalid size hint"
    );
}

mod ensure_fits_in {
    use super::*;

    test_failable!(pass, CapacityError::<i32>::ensure_fits_in::<[i32; 5], _>(&(1..=5)), Ok);
    test_failable!(fail, CapacityError::<i32>::ensure_fits_in::<[i32; 5], _>(&(1..=6)),
        capacity => SizeHint::exact(5),
        kind => CapacityErrorKind::Bounds { hint: SizeHint::exact(6) }
    );

    panics!(panic, CapacityError::<i32>::ensure_fits_in::<[i32; 5], _>(&InvalidIterator::<i32>::DEFAULT), "Invalid size hint");
}

mod ensure_fits_into {
    use super::*;
    use arrayvec::ArrayVec;

    #[test]
    fn pass() {
        CapacityError::<i32>::ensure_fits_into(&(1..=5), &ArrayVec::<i32, 5>::new()).expect("Should not fail");
    }

    #[test]
    fn fail() {
        let err = CapacityError::<i32>::ensure_fits_into(&(1..=6), &ArrayVec::<i32, 5>::new()).expect_err("Should fail");

        assert_eq!(err.capacity, SizeHint::at_most(5));
        assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(6) });
    }

    panics!(
        panic,
        CapacityError::<i32>::ensure_fits_into(&InvalidIterator::<i32>::DEFAULT, &ArrayVec::<i32, 5>::new()),
        "Invalid size hint"
    );
}
