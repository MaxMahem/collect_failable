use collect_failable::errors::CollectError;

use crate::error_tests::{TestError, test_source};
use crate::utils::panics;

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"CollectError { collected: "alloc::vec::Vec<i32>", error: TestError("test"), iterator: "core::ops::range::RangeInclusive<i32>" }"#;
    const EXPECTED_DISPLAY: &str = "Collection Error: Test error: test";
    const EXPECTED_DEBUG_DATA: &str =
        r#"CollectErrorData { iterator: RangeInclusive<i32>, collected: Vec<i32>, error: TestError("test") }"#;

    test_format!(debug, CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")), "{:?}", EXPECTED_DEBUG);
    test_format!(display, CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")), "{}", EXPECTED_DISPLAY);
    test_format!(
        debug_data,
        CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")).into_data(),
        "{:?}",
        EXPECTED_DEBUG_DATA
    );
}

mod ctors {
    use super::*;
    use collect_failable::errors::SizeHint;
    use collect_failable::errors::{CapacityError, Collision};
    use size_hinter::InvalidIterator;

    use crate::error_tests::test_ctor;

    test_ctor!(
        new,
        CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")),
        iterator => 1..=2,
        collected => vec![3, 4],
        error => TestError::new("test")
    );

    test_ctor!(
        bounds,
        CollectError::<_, Vec<i32>, _>::bounds(1..=2, SizeHint::exact(5)),
        iterator => 1..=2,
        collected => Vec::<i32>::new(),
        error => CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(2))
    );

    test_ctor!(
        overflow,
        CollectError::overflow(3..=4, vec![1, 2], 99, SizeHint::exact(2)),
        iterator => 3..=4,
        collected => vec![1, 2],
        error => CapacityError::overflow(SizeHint::exact(2), 99)
    );

    test_ctor!(
        collect_overflowed,
        CollectError::collect_overflowed(1..=3, [1, 2], 99),
        iterator => 1..=3,
        collected => [1, 2],
        error => CapacityError::overflow(SizeHint::exact(2), 99)
    );

    test_ctor!(
        extend_overflowed,
        CollectError::<_, Vec<i32>, _>::extend_overflowed(1..=3, 99),
        iterator => 1..=3,
        collected => Vec::<i32>::new(),
        error => CapacityError::overflow(SizeHint::ZERO, 99)
    );

    test_ctor!(
        underflow,
        CollectError::<_, Vec<i32>, _>::underflow(1..=2, vec![1, 2], SizeHint::exact(5)),
        iterator => 1..=2,
        collected => vec![1, 2],
        error => CapacityError::underflow(SizeHint::exact(5), 2)
    );

    test_ctor!(
        collect_underflowed,
        CollectError::<_, [i32; 2], _>::collect_underflowed(1..=2, [1, 2]),
        iterator => 1..=2,
        collected => [1, 2],
        error => CapacityError::underflow(SizeHint::exact(2), 2)
    );

    test_ctor!(
        collision,
        CollectError::collision(3..=4, [1, 2], 99),
        collected => [1, 2],
        error => Collision::new(99)
    );

    test_ctor!(
        into_data,
        CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")).into_data(),
        iterator => 1..=2,
        collected => vec![3, 4],
        error => TestError::new("test")
    );

    panics!(
        panic_bounds,
        CollectError::<_, Vec<i32>, _>::bounds(InvalidIterator::<i32>::DEFAULT, SizeHint::exact(5)),
        "Invalid size hint"
    );
}

test_source!(source, CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")), TestError<i32>);

#[test]
fn into_iter() {
    let items: Vec<_> = CollectError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")).into_iter().collect();

    assert_unordered::assert_eq_unordered!(items, vec![1, 2, 3, 4]);
}

mod ensure_fits_in {
    use super::*;
    use crate::error_tests::test_failable;
    use collect_failable::errors::{CapacityError, SizeHint};

    test_failable!(pass, CollectError::<_, [i32; 5], _>::ensure_fits_in(1..=5), Ok);

    test_failable!(
        fail,
        CollectError::<_, [i32; 5], _>::ensure_fits_in(1..=6),
        iterator => 1..=6,
        collected => [0; 5],
        error => CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(6))
    );

    panics!(
        panic,
        CollectError::<_, [i32; 5], _>::ensure_fits_in(size_hinter::InvalidIterator::<i32>::DEFAULT),
        "Invalid size hint"
    );
}

mod ensure_fits_into {
    use super::*;
    use crate::error_tests::test_failable;
    use arrayvec::ArrayVec;
    use collect_failable::errors::{CapacityError, SizeHint};

    test_failable!(pass, CollectError::ensure_fits_into(1..=5, &ArrayVec::<i32, 5>::new()), Ok);

    test_failable!(
        fail,
        CollectError::ensure_fits_into(1..=6, &ArrayVec::<i32, 5>::new()),
        iterator => 1..=6,
        collected => ArrayVec::<i32, 5>::new(),
        error => CapacityError::bounds(SizeHint::at_most(5), SizeHint::exact(6))
    );

    panics!(
        panic,
        CollectError::ensure_fits_into(size_hinter::InvalidIterator::<i32>::DEFAULT, &ArrayVec::<i32, 5>::new()),
        "Invalid size hint"
    );
}
