use collect_failable::errors::CollectionError;
use std::collections::HashSet;

use crate::error_tests::{into_iterator, test_deref, test_format, test_source, TestError};

type Collection = HashSet<u32>;

fn create_with_rejected() -> CollectionError<std::array::IntoIter<u32, 2>, Collection, TestError> {
    let remaining = [3, 4];
    let iter = remaining.into_iter();
    let collected = HashSet::from([1, 2]);
    let rejected = Some(99);

    CollectionError::new(iter, collected, rejected, TestError::new("with rejected"))
}

fn create_without_rejected() -> CollectionError<std::array::IntoIter<u32, 2>, Collection, TestError> {
    let remaining = [3, 4];
    let iter = remaining.into_iter();
    let collected = HashSet::from([1, 2]);
    let rejected = None;

    CollectionError::new(iter, collected, rejected, TestError::new("without rejected"))
}

const EXPECTED_DEBUG_WITH_REJECTED: &str = r#"PartialIterErr { collected: "std::collections::hash::set::HashSet<u32>", rejected: Some(..), error: TestError { identity: "with rejected" }, iterator: "core::array::iter::IntoIter<u32, 2>" }"#;
const EXPECTED_DEBUG_WITHOUT_REJECTED: &str = r#"PartialIterErr { collected: "std::collections::hash::set::HashSet<u32>", rejected: None, error: TestError { identity: "without rejected" }, iterator: "core::array::iter::IntoIter<u32, 2>" }"#;
const EXPECTED_DISPLAY_WITH_REJECTED: &str = "Test error: with rejected";
const EXPECTED_DISPLAY_WITHOUT_REJECTED: &str = "Test error: without rejected";

test_format!(debug_format_with_rejected, create_with_rejected(), "{:?}", EXPECTED_DEBUG_WITH_REJECTED);
test_format!(debug_format_without_rejected, create_without_rejected(), "{:?}", EXPECTED_DEBUG_WITHOUT_REJECTED);
test_format!(display_format_with_rejected, create_with_rejected(), "{}", EXPECTED_DISPLAY_WITH_REJECTED);
test_format!(display_format_without_rejected, create_without_rejected(), "{}", EXPECTED_DISPLAY_WITHOUT_REJECTED);

#[test]
fn into_parts_with_rejected() {
    let parts = create_with_rejected().into_data();

    assert_eq!(parts.error, TestError::new("with rejected"));
    assert_eq!(parts.collected, HashSet::from([1, 2]));
    assert_eq!(parts.rejected, Some(99));
    assert_eq!(parts.iterator.collect::<Vec<_>>(), vec![3, 4]);
}

#[test]
fn into_parts_without_rejected() {
    let parts = create_without_rejected().into_data();

    assert_eq!(parts.error, TestError::new("without rejected"));
    assert_eq!(parts.collected, HashSet::from([1, 2]));
    assert_eq!(parts.rejected, None);
    assert_eq!(parts.iterator.collect::<Vec<_>>(), vec![3, 4]);
}

test_deref!(deref_error, create_with_rejected(), error, TestError::new("with rejected"));

// Should contain: rejected (99) + collected (1, 2 in some order) + remaining (3, 4)
into_iterator!(into_iterator_with_rejected, create_with_rejected(), expected_len = 5, contains = [99, 1, 2, 3, 4]);

// Should contain: collected (1, 2 in some order) + remaining (3, 4)
into_iterator!(into_iterator_without_rejected, create_without_rejected(), expected_len = 4, contains = [1, 2, 3, 4]);

test_source!(error_trait_source, create_with_rejected(), TestError);

// Macro for testing CollectionError constructors
macro_rules! test_ctor {
    // Branch for regular constructor tests
    ($name:ident, $ctor:expr, iter: $iter:expr, collected: $collected:expr, rejected: $rejected:expr, error: $error:expr) => {
        #[test]
        fn $name() {
            let error = $ctor;

            assert_eq!(error.collected, $collected);
            assert_eq!(error.rejected, $rejected);
            assert_eq!(error.error, $error);
            assert_eq!(error.into_data().iterator.collect::<Vec<_>>(), $iter);
        }
    };
    // Branch for panic tests
    ($name:ident, $ctor:expr, panic: $msg:expr) => {
        #[test]
        #[should_panic(expected = $msg)]
        fn $name() {
            let _ = $ctor;
        }
    };
}

mod ctors {
    use super::*;
    use collect_failable::errors::CapacityMismatch;
    use size_hinter::{SizeHint, SizeHinter};

    test_ctor!(
        new,
        CollectionError::new(1..=2, vec![3, 4], None, TestError::new("test")),
        iter: vec![1, 2],
        collected: vec![3, 4],
        rejected: None,
        error: TestError::new("test")
    );

    test_ctor!(
        bounds_min_too_high,
        CollectionError::<_, Vec<u32>, _>::bounds(0..4, SizeHint::bounded(1, 3)),
        iter: vec![0, 1, 2, 3],
        collected: vec![],
        rejected: None,
        error: CapacityMismatch::bounds(SizeHint::bounded(1, 3), SizeHint::exact(4))
    );

    test_ctor!(
        bounds_max_too_low,
        CollectionError::<_, Vec<u32>, _>::bounds(0..0, SizeHint::bounded(5, 10)),
        iter: vec![],
        collected: vec![],
        rejected: None,
        error: CapacityMismatch::bounds(SizeHint::bounded(5, 10), SizeHint::exact(0))
    );

    test_ctor!(
        bounds_unbounded_upper,
        CollectionError::<_, Vec<u32>, _>::bounds((1..=10).hint_min(6), SizeHint::bounded(1, 5)),
        iter: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        collected: vec![],
        rejected: None,
        error: CapacityMismatch::bounds(SizeHint::bounded(1, 5), SizeHint::new(6, None))
    );

    test_ctor!(
        overflow,
        CollectionError::overflow(4..=6, vec![1, 2, 3], 99, SizeHint::bounded(1, 3)),
        iter: vec![4, 5, 6],
        collected: vec![1, 2, 3],
        rejected: Some(99),
        error: CapacityMismatch::overflow(SizeHint::bounded(1, 3))
    );

    test_ctor!(
        underflow,
        CollectionError::<_, [u32; 2], _>::underflow(3..=4, [1, 2], SizeHint::bounded(5, 10)),
        iter: vec![3, 4],
        collected: [1, 2],
        rejected: None,
        error: CapacityMismatch::underflow(SizeHint::bounded(5, 10), 2)
    );
}
