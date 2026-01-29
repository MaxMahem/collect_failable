use crate::collection_tests::{recover_iter_data, try_collect};
use crate::utils::panics;

use collect_failable::errors::capacity::{CapacityError, FixedCap, RemainingCap};
use collect_failable::{TryCollectEx, TryFromIterator};

use size_hinter::{InvalidIterator, SizeHint, SizeHinter};
use tap::Pipe;

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
    for consume in 0..=6 {
        let (counters, viewers) = dropcount::new_vec(6);

        let mut drain = counters
            .into_iter()
            .hide_size()
            .try_collect_ex::<[_; 5]>()
            .expect_err("should overflow")
            .into_data()
            .collected
            .into_iter();

        drain.by_ref().take(consume).for_each(drop);

        drop(drain);

        viewers.iter().for_each(|viewer| assert_eq!(viewer.get(), 1, "Items should be dropped once (consume={})", consume));
    }
}

mod try_from {
    use collect_failable::errors::partial_array::IntoArrayError;

    use super::*;

    #[test]
    fn try_from_partial_array_overflow() {
        let partial = (1..=6).hide_size().try_collect_ex::<Array>().expect_err("should overflow").into_data().collected;

        let array: Array = partial.try_into().expect("should succeed since array is full");

        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn try_from_partial_array_underflow() {
        let partial = (1..=4).hide_size().try_collect_ex::<Array>().expect_err("should underflow").into_data().collected;

        let IntoArrayError { partial_array, error } = Array::try_from(partial).expect_err("should fail since array is not full");

        assert_eq!(error, UNDERFLOW_ERR);
        assert_eq!(partial_array, [1, 2, 3, 4][..]);
    }
}

mod eq {
    use super::*;

    #[test]
    fn partial_array_eq_slice() {
        let err = (1..=3).hide_size().try_collect_ex::<Array>().expect_err("should underflow");
        let partial = err.into_data().collected;
        assert_eq!(partial, [1, 2, 3][..]);
        assert_ne!(partial, [1, 2][..]);
        assert_ne!(partial, [1, 2, 3, 4][..]);
    }

    #[test]
    fn partial_array_eq_slice_overflow() {
        let err = (1..=6).hide_size().try_collect_ex::<Array>().expect_err("should overflow");
        let partial = err.into_data().collected;
        assert_eq!(partial, [1, 2, 3, 4, 5][..]);
    }
}

mod iter {
    use super::*;

    #[test]
    fn borrow() {
        let partial = (1..=4).hide_size().try_collect_ex::<Array>().expect_err("should underflow").into_data().collected;

        let collected: Vec<_> = (&partial).into_iter().copied().collect();

        assert_eq!(collected, vec![1, 2, 3, 4]);
    }
}
