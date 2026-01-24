use crate::collection_tests::{panics, recover_iter_data, try_collect};

use collect_failable::errors::CapacityError;
use collect_failable::{FixedCap, RemainingCap, TryCollectEx, TryFromIterator};
use size_hinter::{InvalidIterator, SizeHint, SizeHinter};
use tap::Pipe;

const BOUNDS_OVER_ERR: CapacityError<u32> = CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(6));
const BOUNDS_UNDER_ERR: CapacityError<u32> = CapacityError::bounds(SizeHint::exact(5), SizeHint::exact(4));
const OVERFLOW_ERR: CapacityError<u32> = CapacityError::overflowed(6);
const UNDERFLOW_ERR: CapacityError<u32> = CapacityError::underflow(SizeHint::exact(5), 4);

type Array = [u32; 5];

try_collect!(valid_array, Array, 1..=5, Ok([1, 2, 3, 4, 5]));
try_collect!(too_long_data, Array, 1..=6, Err(BOUNDS_OVER_ERR));
try_collect!(too_short_data, Array, 1..=4, Err(BOUNDS_UNDER_ERR));
try_collect!(too_long_data_hidden, Array, (1..=6).hide_size(), Err(OVERFLOW_ERR));
try_collect!(too_short_data_hidden, Array, (1..=4).hide_size(), Err(UNDERFLOW_ERR));

panics!(panic_on_invalid_iterator, Array::try_from_iter(InvalidIterator::DEFAULT), "invalid size hint");

#[test]
fn array_capacity() {
    let array: [i32; 2] = [0; 2];
    assert_eq!(array.remaining_cap(), SizeHint::exact(0));
    assert_eq!(<[i32; 2] as FixedCap>::CAP, SizeHint::exact(2));
}

mod recover_iter {
    use super::*;

    recover_iter_data!(bounds, Array, 1..4, [0; 0][..], vec![1, 2, 3]);
    recover_iter_data!(underflow, Array, (1..4).hide_size(), [1, 2, 3][..], vec![1, 2, 3]);
    recover_iter_data!(overflow, Array, (1..=6).hide_size(), [1, 2, 3, 4, 5][..], vec![6, 1, 2, 3, 4, 5]);
}

mod partial_array_drop {
    use super::*;

    macro_rules! partial_array_drop {
        ($name:ident, $dropcount:expr) => {
            #[test]
            fn $name() {
                let (counters, viewers) = $dropcount;

                counters
                    .into_iter()
                    .hide_size()
                    .try_collect_ex::<[_; 5]>()
                    .expect_err("should error")
                    .into_data()
                    .collected
                    .pipe(drop);

                viewers.iter().for_each(|viewer| assert_eq!(viewer.get(), 1, "Item should be dropped once"));
            }
        };
    }

    partial_array_drop!(partial_array_drops_on_underflow, dropcount::new_vec(4));
    partial_array_drop!(partial_array_drops_on_overflow, dropcount::new_vec(6));
}

#[test]
fn partial_array_drain_drop() {
    let (counters, viewers) = dropcount::new_vec(3);

    counters
        .into_iter()
        .hide_size()
        .try_collect_ex::<[_; 5]>()
        .expect_err("should overflow")
        .into_data()
        .collected
        .into_iter()
        .pipe(drop);

    viewers.iter().for_each(|viewer| assert_eq!(viewer.get(), 1, "Item should be dropped once"));
}

#[test]
fn partial_array_drain_drop_partial() {
    let (counters, viewers) = dropcount::new_vec(3);

    let mut drain = counters
        .into_iter()
        .hide_size()
        .try_collect_ex::<[_; 5]>()
        .expect_err("should overflow")
        .into_data()
        .collected
        .into_iter();

    assert!(drain.next().is_some());
    assert!(drain.next().is_some());

    drop(drain);

    viewers.iter().for_each(|viewer| assert_eq!(viewer.get(), 1, "Item should be dropped once"));
}

#[test]
fn partial_array_drain_next_back() {
    let back = (1..=3)
        .hide_size()
        .try_collect_ex::<Array>()
        .expect_err("should underflow")
        .into_data()
        .collected
        .into_iter()
        .next_back();

    assert_eq!(back, Some(3));
}
