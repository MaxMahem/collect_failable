use crate::collection_tests::recover_iter_data;
use crate::utils::panics;

use arrayvec::ArrayVec;
use collect_failable::TryFromIterator;
use collect_failable::errors::CapacityError;
use size_hinter::{InvalidIterator, SizeHint, SizeHinter};

type TestArrayVec = ArrayVec<i32, 2>;

mod try_from_iter {
    use super::*;
    use crate::collection_tests::try_collect;
    use collect_failable::TryFromIterator;

    try_collect!(valid, TestArrayVec, 1..=2, Ok(TestArrayVec::from_iter(1..=2)));
    try_collect!(bounds, TestArrayVec, 1..=3, Err(CapacityError::bounds(SizeHint::at_most(2), SizeHint::exact(3))));
    try_collect!(overflow, TestArrayVec, (1..=3).hide_size(), Err(CapacityError::overflow(SizeHint::at_most(2), 3)));
}

recover_iter_data!(recover_iter_data, TestArrayVec, (1..=3).hide_size(), [1, 2][..], vec![3, 1, 2]);

mod try_extend_safe {
    use super::*;
    use crate::collection_tests::try_extend_safe;
    use collect_failable::TryExtendSafe;

    try_extend_safe!(valid, TestArrayVec::from_iter(3..=3), 3..=3, Ok(TestArrayVec::from_iter([3, 3])));
    try_extend_safe!(
        bound_fail,
        TestArrayVec::from_iter(3..=3),
        1..=3,
        Err(CapacityError::bounds(SizeHint::at_most(1), SizeHint::exact(3)), TestArrayVec::new(), 1..=3)
    );

    try_extend_safe!(
        overflow,
        TestArrayVec::from_iter(3..=3),
        (1..=2).hide_size(),
        Err(CapacityError::overflow(SizeHint::at_most(1), 2), TestArrayVec::from_iter([1]), std::iter::empty::<i32>())
    );

    panics!(invalid_iter, TestArrayVec::new().try_extend_safe(InvalidIterator::DEFAULT), "Invalid size hint");
}

mod try_extend {
    use super::*;
    use crate::collection_tests::try_extend;
    use collect_failable::TryExtend;

    try_extend!(valid, TestArrayVec::from_iter(3..=3), 3..=3, Ok(TestArrayVec::from_iter([3, 3])));
    try_extend!(
        bounds_fail,
        TestArrayVec::from_iter(3..=3),
        1..=3,
        Err(CapacityError::bounds(SizeHint::at_most(1), SizeHint::exact(3)), TestArrayVec::new(), 1..=3)
    );

    try_extend!(
        overflow,
        TestArrayVec::from_iter(3..=3),
        (1..=2).hide_size(),
        Err(CapacityError::overflow(SizeHint::ZERO, 2), TestArrayVec::new(), std::iter::empty::<i32>())
    );

    panics!(invalid_iter, TestArrayVec::new().try_extend(InvalidIterator::DEFAULT), "Invalid size hint");
}

mod try_extend_one {
    use super::*;
    use crate::collection_tests::try_extend_one;
    use collect_failable::TryExtendOne;

    try_extend_one!(valid, TestArrayVec::new(), 1, Ok(TestArrayVec::from_iter(1..=1)));
    try_extend_one!(collision, TestArrayVec::from([1, 2]), 2, Err(CapacityError::extend_overflowed(2)));
}
