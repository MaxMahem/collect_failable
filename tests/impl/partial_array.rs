use collect_failable::errors::capacity::{CapacityError, FixedCap, RemainingCap};
use collect_failable::errors::partial_array::{IntoArrayError, PartialArray};
use collect_failable::errors::types::SizeHint;

type Array = [i32; 5];
type TestPartialArray = PartialArray<i32, 5>;

const UNDERFLOW_ERR: CapacityError<i32> = CapacityError::underflow(SizeHint::exact(5), 4);

#[test]
fn default() {
    let partial = TestPartialArray::default();

    assert_eq!(partial.len(), 0);
    assert_eq!(partial.remaining_cap(), SizeHint::at_most(5));
    assert_eq!(partial, [0; 0][..]);
}

#[test]
fn try_push() {
    let mut partial = TestPartialArray::new();
    assert_eq!(partial.try_push(1), Ok(&mut 1));
    assert_eq!(partial.try_push(2), Ok(&mut 2));
    assert_eq!(partial.try_push(3), Ok(&mut 3));
    assert_eq!(partial.try_push(4), Ok(&mut 4));
    assert_eq!(partial.try_push(5), Ok(&mut 5));
    assert_eq!(partial.try_push(6), Err(6));
    assert_eq!(partial[..], [1, 2, 3, 4, 5]);
}

mod drop {
    use super::*;

    macro_rules! drop_test {
        ($name:ident, $ctor:expr, $len:expr) => {
            #[test]
            fn $name() {
                let (counters, viewers) = dropcount::new_vec($len);

                {
                    let mut partial = $ctor;
                    _ = counters.into_iter().try_for_each(|counter| partial.try_push(counter).map(drop));
                }

                viewers.iter().for_each(|viewer| assert_eq!(viewer.get(), 1, "Item should be dropped once"));
            }
        };
    }

    drop_test!(underflow, PartialArray::<_, 5>::new(), 4);
    drop_test!(overflow, PartialArray::<_, 5>::new(), 6);
}

#[test]
fn drain_drop() {
    (0..=5).for_each(|consume| {
        let (counters, viewers) = dropcount::new_vec(5);

        let mut partial = PartialArray::<_, 5>::new();
        counters.into_iter().try_for_each(|counter| partial.try_push(counter).map(drop)).expect("should succeed");

        let drain = partial.into_iter();

        drain.take(consume).for_each(drop);

        viewers.iter().for_each(|viewer| assert_eq!(viewer.get(), 1, "Items should be dropped once (consume={})", consume));
    });
}

mod try_from {
    use super::*;

    #[test]
    fn partial_array_overflow() {
        let mut partial = TestPartialArray::new();
        for i in 1..=6 {
            partial.try_push(i).ok();
        }

        let array: Array = partial.try_into().expect("should succeed since array is full");

        assert_eq!(array, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn partial_array_underflow() {
        let mut partial = TestPartialArray::new();
        for i in 1..=4 {
            partial.try_push(i).ok();
        }

        let IntoArrayError { partial_array, error } = Array::try_from(partial).expect_err("should fail since array is not full");

        assert_eq!(error, UNDERFLOW_ERR);
        assert_eq!(partial_array, [1, 2, 3, 4][..]);
    }
}

mod eq {
    use super::*;

    #[test]
    fn partial_array_eq_slice() {
        let mut partial = PartialArray::<u32, 5>::new();
        for i in 1..=3 {
            partial.try_push(i).ok();
        }

        assert_eq!(partial, [1, 2, 3][..]);
        assert_ne!(partial, [1, 2][..]);
        assert_ne!(partial, [1, 2, 3, 4][..]);
    }
}

mod iter {
    use super::*;

    #[test]
    fn borrow() {
        let mut partial = PartialArray::<u32, 5>::new();
        for i in 1..=4 {
            partial.try_push(i).ok();
        }

        let collected: Vec<_> = (&partial).into_iter().copied().collect();

        assert_eq!(collected, vec![1, 2, 3, 4]);
    }
}

#[test]
fn capacity() {
    let mut partial = TestPartialArray::new();
    assert_eq!(TestPartialArray::CAP, SizeHint::at_most(5));
    assert_eq!(partial.remaining_cap(), SizeHint::at_most(5));

    _ = (0..=5).try_for_each(|i| {
        assert_eq!(partial.remaining_cap(), SizeHint::at_most(5 - i as usize));
        partial.try_push(i).map(drop)
    });
    assert_eq!(partial.remaining_cap(), SizeHint::at_most(0));
}
