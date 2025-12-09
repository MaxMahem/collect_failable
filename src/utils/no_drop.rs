use std::mem::ManuallyDrop;
use std::ptr;

/// A wrapper that ensures a value is explicitly consumed before being dropped.
///
/// This type always uses unsafe code and will panic if the value is dropped
/// without calling [`NoDrop::consume()`]. This provides runtime safety checks
/// in all build modes.
///
/// For a version that only checks in debug mode, see [`NoDropDbg`].
#[derive(Debug, derive_more::Constructor)]
pub(crate) struct NoDrop<T>(T);

impl<T> NoDrop<T> {
    /// Consumes the wrapper and returns the inner value.
    pub(crate) fn consume(self) -> T {
        let this = ManuallyDrop::new(self);
        unsafe { ptr::read(&raw const this.0) }
    }
}

impl<T> Drop for NoDrop<T> {
    #[track_caller]
    fn drop(&mut self) {
        panic!("Value was dropped without being consumed");
    }
}

// In debug mode, NoDropDbg is an alias to NoDrop
#[cfg(debug_assertions)]
#[allow(unused_imports)]
pub(crate) use NoDrop as NoDropDbg;

// In release mode, NoDropDbg is a simple zero-cost wrapper
#[cfg(not(debug_assertions))]
mod release_impl {
    /// A wrapper that ensures a value is explicitly consumed before being dropped.
    ///
    /// In debug mode, this is an alias to [`NoDrop`] and will panic if dropped
    /// without consuming. In release mode, this is a zero-cost wrapper with no checks.
    #[derive(Debug, derive_more::Constructor)]
    pub(crate) struct NoDropDbg<T>(T);

    impl<T> NoDropDbg<T> {
        /// Consumes the wrapper and returns the inner value.
        pub(crate) fn consume(self) -> T {
            self.0
        }
    }
}

#[cfg(not(debug_assertions))]
pub(crate) use release_impl::NoDropDbg;

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for NoDrop (always checks)
    #[test]
    fn no_drop_consume_returns_value() {
        let value = NoDrop::new(42);
        assert_eq!(value.consume(), 42);
    }

    #[test]
    #[should_panic(expected = "Value was dropped without being consumed")]
    fn no_drop_panics_on_drop() {
        let _value = NoDrop::new(42);
        // value is dropped here without being consumed, should panic
    }

    // Tests for NoDropDbg (checks in debug, no-op in release)
    #[test]
    fn no_drop_dbg_consume_returns_value() {
        let value = NoDropDbg::new(42);
        assert_eq!(value.consume(), 42);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Value was dropped without being consumed")]
    fn no_drop_dbg_panics_on_drop_in_debug() {
        let _value = NoDropDbg::new(42);
        // In debug mode, this should panic
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn no_drop_dbg_does_not_panic_in_release() {
        let _value = NoDropDbg::new(42);
        // In release mode, this should not panic
    }
}
