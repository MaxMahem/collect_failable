use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;

use crate::utils::FixedSizeHintEx;
use collect_failable::TryFromIterator;

#[derive(Debug, Clone, Default)]
struct DropTracker {
    count: Rc<Cell<usize>>,
}

impl DropTracker {
    fn new() -> Self {
        Self::default()
    }

    fn count(&self) -> usize {
        self.count.get()
    }
}

impl Drop for DropTracker {
    fn drop(&mut self) {
        self.count.set(self.count.get() + 1);
    }
}

#[test]
fn drop_safety_success() {
    let drop_tracker = DropTracker::new();
    {
        let items: Vec<DropTracker> = (0..3).map(|_| drop_tracker.clone()).collect();
        let res: Result<[DropTracker; 3], _> = <[DropTracker; 3]>::try_from_iter(items);
        assert!(res.is_ok());
        assert_eq!(drop_tracker.count(), 0);
    }
    assert_eq!(drop_tracker.count(), 3);
}

#[test]
fn drop_safety_failure_too_few() {
    let drop_tracker = DropTracker::new();
    {
        let items: Vec<DropTracker> = (0..2).map(|_| drop_tracker.clone()).collect();
        <[DropTracker; 3]>::try_from_iter(items).expect_err("Should have failed to collect array");
        // Items consumed so far should be dropped
        assert_eq!(drop_tracker.count(), 2);
    }
    assert_eq!(drop_tracker.count(), 2);
}

#[test]
fn drop_safety_failure_too_many() {
    let drop_tracker = DropTracker::new();
    {
        let items: Vec<DropTracker> = (0..4).map(|_| drop_tracker.clone()).collect();
        // Use HideSize to ensure we hit the runtime check
        <[DropTracker; 3]>::try_from_iter(items.into_iter().hide_size()).expect_err("Should have failed to collect array");
        // All items consumed (3 for array) should be dropped
        assert_eq!(drop_tracker.count(), 4);
    }
    assert_eq!(drop_tracker.count(), 4);
}

#[test]
fn check_zst() {
    struct Zst;

    let items = vec![Zst, Zst, Zst];
    <[Zst; 3]>::try_from_iter(items).expect("Failed to collect array");
}

#[test]
fn check_panic_safety() {
    let drop_tracker = DropTracker::new();

    panic::catch_unwind(AssertUnwindSafe(|| {
        let items = (0..3).map(|i| {
            if i == 2 {
                panic!("oops");
            }
            drop_tracker.clone()
        });
        <[DropTracker; 3]>::try_from_iter(items)
    }))
    .expect_err("Should have panicked");

    // 2 items were created and consumed before panic. They should be dropped by InitGuard.
    assert_eq!(drop_tracker.count(), 2);
}

#[test]
fn check_panic_safety_too_many() {
    let drop_tracker = DropTracker::new();

    panic::catch_unwind(AssertUnwindSafe(|| {
        let items = (0..=4).map(|i| {
            if i == 3 {
                panic!("oops");
            }
            drop_tracker.clone()
        });
        <[DropTracker; 3]>::try_from_iter(items.into_iter().hide_size())
    }))
    .expect_err("Should have panicked");

    // 3 items were created and consumed before panic. 3 should be dropped by InitGuard.
    assert_eq!(drop_tracker.count(), 3);
}
