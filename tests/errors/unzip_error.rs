use std::array::IntoIter;
use std::collections::HashSet;

use collect_failable::errors::{Collision, UnzipError};

use crate::error_tests::test_source;

// Define a type alias for the collection to make signatures cleaner
type Collection = HashSet<i32>;

const COLLISION_ERROR: Collision<i32> = Collision { item: 1 };

fn failed_collection() -> Collection {
    HashSet::from([1, 2])
}

fn partial_collection() -> Collection {
    HashSet::from([10, 20])
}

const PENDING_ITEM: Option<i32> = Some(30);

fn remaining_iterator() -> IntoIter<(i32, i32), 1> {
    [(3, 40)].into_iter()
}

fn sample_error() -> UnzipError<Collision<i32>, Collection, Collection, i32, IntoIter<(i32, i32), 1>> {
    UnzipError::new(COLLISION_ERROR, failed_collection(), partial_collection(), PENDING_ITEM, remaining_iterator())
}

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"UnzipError { error: Collision { item: 1 }, failed: HashSet<i32>, partial: HashSet<i32>, pending: Some(i32), remaining: core::array::iter::IntoIter<(i32, i32), 1> }"#;
    const EXPECTED_DEBUG_DATA: &str = r#"UnzipErrorData { error: Collision { item: 1 }, failed: HashSet<i32>, partial: HashSet<i32>, pending: Some(i32), remaining: core::array::iter::IntoIter<(i32, i32), 1> }"#;
    const EXPECTED_DISPLAY: &str = "Failed while unzipping collection: item collision";

    test_format!(debug, sample_error(), "{:?}", EXPECTED_DEBUG);
    test_format!(debug_data, sample_error().into_data(), "{:?}", EXPECTED_DEBUG_DATA);
    test_format!(display, sample_error(), "{}", EXPECTED_DISPLAY);
    test_format!(display_data, sample_error().into_data(), "{}", EXPECTED_DISPLAY);
}

mod ctors {
    use super::*;
    use crate::error_tests::test_ctor;

    test_ctor!(
        into_data,
        sample_error().into_data(),
        error => COLLISION_ERROR,
        failed => failed_collection(),
        partial => partial_collection(),
        pending => PENDING_ITEM,
    );

    test_ctor!(
        new,
        sample_error(),
        error => COLLISION_ERROR,
        failed => failed_collection(),
        partial => partial_collection(),
        pending => PENDING_ITEM,
    );

    #[test]
    fn remaining_content() {
        let error = sample_error();
        assert_eq!(error.into_data().remaining.collect::<Vec<_>>(), vec![(3, 40)]);
    }
}

test_source!(source, sample_error(), Collision<i32>);
test_source!(source_data, sample_error().into_data(), Collision<i32>);
