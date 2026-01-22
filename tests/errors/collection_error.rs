use collect_failable::errors::CollectionError;

use crate::error_tests::{test_source, TestError};

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"CollectionError { collected: "alloc::vec::Vec<i32>", error: TestError("test"), iterator: "core::ops::range::RangeInclusive<i32>" }"#;
    const EXPECTED_DISPLAY: &str = "Collection Error: Test error: test";
    const EXPECTED_DEBUG_DATA: &str =
        r#"CollectionErrorData { iterator: RangeInclusive<i32>, collected: Vec<i32>, error: TestError("test") }"#;

    test_format!(debug, CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")), "{:?}", EXPECTED_DEBUG);
    test_format!(display, CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")), "{}", EXPECTED_DISPLAY);
    test_format!(
        debug_data,
        CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")).into_data(),
        "{:?}",
        EXPECTED_DEBUG_DATA
    );
}

mod ctors {
    use super::*;
    use collect_failable::errors::{CapacityError, Collision};
    use collect_failable::SizeHint;

    use crate::error_tests::test_ctor;

    test_ctor!(
        new,
        CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")),
        iterator => 1..=2,
        collected => vec![3, 4],
        error => TestError::new("test")
    );

    test_ctor!(
        bounds,
        CollectionError::<_, Vec<i32>, _>::bounds(1..=2, SizeHint::exact(5)),
        iterator => 1..=2,
        collected => Vec::<i32>::new(),
        error => CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(2))
    );

    test_ctor!(
        overflow,
        CollectionError::overflow(3..=4, vec![1, 2], 99, SizeHint::exact(2)),
        iterator => 3..=4,
        collected => vec![1, 2],
        error => CapacityError::overflow(SizeHint::exact(2), 99)
    );

    test_ctor!(
        underflow,
        CollectionError::<_, Vec<i32>, _>::underflow(1..=2, vec![1, 2], SizeHint::exact(5)),
        iterator => 1..=2,
        collected => vec![1, 2],
        error => CapacityError::underflow(SizeHint::exact(5), 2)
    );

    test_ctor!(
        collision,
        CollectionError::collision(3..=4, [1, 2], 99),
        collected => [1, 2],
        error => Collision::new(99)
    );

    test_ctor!(
        into_data,
        CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")).into_data(),
        iterator => 1..=2,
        collected => vec![3, 4],
        error => TestError::new("test")
    );
}

test_source!(source, CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")), TestError<i32>);

mod into_iterator {
    use super::*;
    use crate::error_tests::into_iterator;

    into_iterator!(
        into_iter,
        CollectionError::new(1..=2, vec![3, 4], TestError::<i32>::new("test")),
        expected_len = 4,
        contains = [1, 2, 3, 4]
    );
}
