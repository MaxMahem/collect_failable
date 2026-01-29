use crate::collection_tests::{recover_iter_data, try_collect};
use crate::utils::panics;

use collect_failable::TryFromIterator;
use collect_failable::errors::capacity::{CapacityError, FixedCap, RemainingCap};

use size_hinter::{InvalidIterator, SizeHint, SizeHinter};

const BOUNDS_OVER_ERR: CapacityError<u32> = CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(6));
const BOUNDS_UNDER_ERR: CapacityError<u32> = CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(4));
const OVERFLOW_ERR: CapacityError<u32> = CapacityError::overflow(SizeHint::exact(5), 6);
const UNDERFLOW_ERR: CapacityError<u32> = CapacityError::underflow(SizeHint::exact(5), 4);

type Array = [u32; 5];

try_collect!(valid_array, Array, 1..=5, Ok([1, 2, 3, 4, 5]));
try_collect!(too_long_data, Array, 1..=6, Err(BOUNDS_OVER_ERR));
try_collect!(too_short_data, Array, 1..=4, Err(BOUNDS_UNDER_ERR));
try_collect!(too_long_data_hidden, Array, (1..=6).hide_size(), Err(OVERFLOW_ERR));
try_collect!(too_short_data_hidden, Array, (1..=4).hide_size(), Err(UNDERFLOW_ERR));

panics!(panic_on_invalid_iterator, Array::try_from_iter(InvalidIterator::DEFAULT), "Invalid size hint: InvalidSizeHint");

#[test]
fn array_capacity() {
    let array: Array = [0; 5];
    assert_eq!(array.remaining_cap(), SizeHint::exact(0));
    assert_eq!(<Array as FixedCap>::CAP, SizeHint::exact(5));
}

mod recover_iter {
    use super::*;

    recover_iter_data!(bounds, Array, 1..4, [0; 0][..], vec![1, 2, 3]);
    recover_iter_data!(underflow, Array, (1..4).hide_size(), [1, 2, 3][..], vec![1, 2, 3]);
    recover_iter_data!(overflow, Array, (1..=6).hide_size(), [1, 2, 3, 4, 5][..], vec![6, 1, 2, 3, 4, 5]);
}
