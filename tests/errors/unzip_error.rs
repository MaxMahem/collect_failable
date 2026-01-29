use std::array::IntoIter;
use std::collections::HashSet;

use collect_failable::errors::{Collision, UnzipError};

use crate::error_tests::test_source;

// Define a type alias for the collection to make signatures cleaner
type Collection = HashSet<u32>;

fn sample_error() -> UnzipError<Collection, Collection, IntoIter<(u32, u32), 1>> {
    UnzipError::new(Collision { item: 1 }, HashSet::from([1, 2]), HashSet::from([10, 20]), Some(30), [(3, 40)].into_iter())
}

mod format {
    use super::*;
    use crate::error_tests::test_format;

    const EXPECTED_DEBUG: &str = r#"UnzipError { error: Collision { item: 1 }, failed: HashSet<u32>, partial: HashSet<u32>, pending: Some(u32), remaining: IntoIter<(u32, u32), 1> }"#;
    const EXPECTED_DEBUG_DATA: &str = r#"UnzipErrorData { error: Collision { item: 1 }, failed: HashSet<u32>, partial: HashSet<u32>, pending: Some(u32), remaining: IntoIter<(u32, u32), 1> }"#;
    const EXPECTED_DISPLAY: &str = "Failed while unzipping collection: item collision";

    test_format!(debug, sample_error(), "{:?}", EXPECTED_DEBUG);
    test_format!(debug_data, sample_error().into_data(), "{:?}", EXPECTED_DEBUG_DATA);
    test_format!(display, sample_error(), "{}", EXPECTED_DISPLAY);
}

mod ctors {
    use super::*;
    use crate::error_tests::test_ctor;

    test_ctor!(
        into_data,
        sample_error().into_data(),
        error => Collision { item: 1 },
        failed => HashSet::from([1, 2]),
        partial => HashSet::from([10, 20]),
        pending => Some(30),
    );

    test_ctor!(
        new,
        sample_error(),
        error => Collision { item: 1 },
        failed => HashSet::from([1, 2]),
        partial => HashSet::from([10, 20]),
        pending => Some(30),
    );

    #[test]
    fn remaining_content() {
        let error = sample_error();
        assert_eq!(error.into_data().remaining.collect::<Vec<_>>(), vec![(3, 40)]);
    }
}

test_source!(source, sample_error(), Collision<u32>);
