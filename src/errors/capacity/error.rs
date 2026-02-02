use crate::errors::capacity::FixedCap;
use crate::errors::types::SizeHint;

use crate::errors::ErrorItemProvider;

/// An error indicating that a collection or extension operation failed because
/// of a conflict between the collection's capacity and the number of items
/// produced by an [`Iterator`].
///
/// This error represents three failure modes, identified by [`CapacityError::kind`].
///
/// - [`CapacityErrorKind::Bounds`] — the iterator’s reported size bounds are
///   incompatible with the required capacity, making the operation impossible
///   to satisfy even before iteration begins.
///
/// - [`CapacityErrorKind::Underflow`] — the iterator produced fewer items than
///   the minimum required by the capacity.
///
/// - [`CapacityErrorKind::Overflow`] — the iterator produced more items than
///   the maximum allowed by the capacity. The item that caused the overflow is
///   preserved, and may be recovered via [`ErrorItemProvider`]. The number of
///   overflowing items is implied to be at least one, but may be more.
///
/// # Type Parameters
///
/// - `T`: The type of the item in the collection.
#[subdef::subdef]
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Collected items out of bounds ({capacity:?}): {kind}")]
pub struct CapacityError<T> {
    /// The capacity constraint that was violated.
    pub capacity: SizeHint,
    /// The specific kind of capacity mismatch that occurred.
    pub kind: [CapacityErrorKind<T>; {
        /// Describes the specific type of capacity mismatch.
        #[derive(Debug, PartialEq, Eq, derive_more::Display)]
        pub enum CapacityErrorKind<T> {
            /// The iterator's [`size_hint`](Iterator::size_hint) is incompatible with the required capacity.
            #[display("Iterator ({hint:?}) cannot satisfy capacity")]
            Bounds {
                /// The iterator's [`size_hint`](Iterator::size_hint).
                hint: SizeHint,
            },
            /// The iterator produced fewer items than the minimum required capacity.
            #[display("Iterator produced ({count}), less than the minimum capacity")]
            Underflow {
                /// The actual number of items produced.
                count: usize,
            },
            /// The iterator produced more items than the maximum allowed capacity.
            #[display("Iterator exceeded capacity")]
            Overflow {
                /// The item that exceeded capacity.
                overflow: T,
            },
        }
    }],
}

impl<T> CapacityError<T> {
    /// Creates a new [`Bounds`](`CapacityErrorKind::Bounds`) [`CapacityError`]
    /// indicating that `hint` bounds are incompatible with `capacity`.
    ///
    /// # Arguments
    ///
    /// * `capacity` - the capacity of the collection
    /// * `hint` - the size hint of the iterator
    ///
    /// # Panics
    ///
    /// Panics if `capacity` and `hint` overlap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::bounds(SizeHint::exact(5), SizeHint::exact(2));
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(2) });
    /// ```
    #[must_use]
    pub const fn bounds(capacity: SizeHint, hint: SizeHint) -> Self {
        assert!(SizeHint::disjoint(capacity, hint), "Bounds must not overlap");
        Self { capacity, kind: CapacityErrorKind::Bounds { hint } }
    }

    /// Ensures that `iter`'s [`size_hint`](Iterator::size_hint) [`overlaps`](SizeHint::overlaps)
    /// `capacity`.
    ///
    /// # Arguments
    ///
    /// * `iter` - the [`Iterator`] to check.
    /// * `capacity` - the capacity of the collection
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](`CapacityErrorKind::Bounds`) [`CapacityError`] if
    /// `iter`'s [`size_hint`](Iterator::size_hint) is [`disjoint`](SizeHint::disjoint)
    /// with `cap`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [`size_hint`](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// CapacityError::<i32>::ensure_fits(&(1..=5), SizeHint::at_most(5)).expect("Should fit");
    ///
    /// let err = CapacityError::<i32>::ensure_fits(&(1..=5), SizeHint::at_most(4))
    ///     .expect_err("Should fit");
    ///
    /// assert_eq!(err.capacity, SizeHint::at_most(4));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(5) });
    /// ```
    pub fn ensure_fits<I: Iterator<Item = T>>(iter: &I, capacity: SizeHint) -> Result<(), Self> {
        let hint = iter.size_hint().try_into().expect("Invalid size hint");

        match SizeHint::disjoint(hint, capacity) {
            true => Err(Self { capacity, kind: CapacityErrorKind::Bounds { hint } }),
            false => Ok(()),
        }
    }

    /// Creates a new [`CapacityError`] indicating that the collection
    /// overflowed `capacity`.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity that overflowed.
    /// * `overflow` - The item that overflowed `capacity`.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is not bounded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::overflow(SizeHint::exact(5), 2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Overflow { overflow: 2 });
    /// ```
    #[must_use]
    pub const fn overflow(capacity: SizeHint, overflow: T) -> Self {
        assert!(capacity.upper().is_some(), "Capacity must have an upper bound to overflow");
        Self { capacity, kind: CapacityErrorKind::Overflow { overflow } }
    }

    /// Creates a new [`CapacityError`] indicating that fewer items were produced
    /// than the minimum required by `capacity`.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity that was underflowed.
    /// * `count` - The number of items that were produced.
    ///
    /// # Panics
    ///
    /// Panics if `count` is greater than or equal to `capacity.lower()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::underflow(SizeHint::exact(5), 2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    pub const fn underflow(capacity: SizeHint, count: usize) -> Self {
        assert!(count < capacity.lower(), "count must be less than capacity");
        Self { capacity, kind: CapacityErrorKind::Underflow { count } }
    }

    /// Creates a new [`CapacityError`] indicating that fewer items were produced
    /// than the minimum required by `C`'s [`FixedCap::CAP`].
    ///
    /// # Arguments
    ///
    /// * `count` - The number of items that were produced.
    ///
    /// # Type Parameters
    ///
    /// * `C` - A collection with a fixed capaciy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::underflow_of::<[i32; 5]>(2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Underflow { count: 2 });
    #[must_use]
    pub const fn underflow_of<C: FixedCap>(count: usize) -> Self {
        Self::underflow(C::CAP, count)
    }
}

impl<T> ErrorItemProvider for CapacityError<T> {
    type Item = T;

    fn into_item(self) -> Option<Self::Item> {
        match self.kind {
            CapacityErrorKind::Overflow { overflow } => Some(overflow),
            _ => None,
        }
    }

    fn item(&self) -> Option<&Self::Item> {
        match &self.kind {
            CapacityErrorKind::Overflow { overflow } => Some(overflow),
            _ => None,
        }
    }
}

#[cfg(feature = "arrayvec")]
impl<T> From<arrayvec::CapacityError<T>> for CapacityError<T> {
    fn from(err: arrayvec::CapacityError<T>) -> Self {
        Self::overflow(SizeHint::ZERO, err.element())
    }
}
