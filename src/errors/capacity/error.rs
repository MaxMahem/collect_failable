use fluent_result::bool::Then;
use tap::TryConv;

use crate::errors::capacity::{FixedCap, RemainingCap};
use crate::errors::types::SizeHint;

use crate::errors::ErrorItemProvider;

/// An error indicating that a collection operation could not proceed because the number of items
/// produced by an [`Iterator`] violated the collection’s declared capacity.
///
/// This error distinguishes three failure modes, identified by [`CapacityError::kind`].
///
/// - [`CapacityErrorKind::Bounds`] — the iterator’s reported size bounds are incompatible with
///   the required capacity, making the operation impossible to satisfy even before iteration begins.
///
/// - [`CapacityErrorKind::Underflow`] — the iterator produced fewer items than the minimum required
///   by the capacity.
///
/// - [`CapacityErrorKind::Overflow`] — the iterator produced more items than the maximum allowed by
///   the capacity. The first overflowing item is preserved, and may be recovered via
///   [`ErrorItemProvider`]. The size of the overflow is at least one item, but may be more.
///
/// # Type Parameters
///
/// - `T`: The type of the item produced by the iterator. It is only present in the `Overflow` case,
///   where the overflow item must be returned to the caller.
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
    /// Creates a new [`CapacityError`] indicating that the bounds provided by `hint`
    /// were incompatible with the required `capacity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::bounds(SizeHint::exact(5), SizeHint::exact(2));
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(2) });
    /// ```
    #[must_use]
    pub const fn bounds(capacity: SizeHint, hint: SizeHint) -> Self {
        Self { capacity, kind: CapacityErrorKind::Bounds { hint } }
    }

    /// Ensures `iterator`'s [`size_hint`](`Iterator::size_hint`) [`overlaps`](SizeHint::overlaps)
    /// `capacity`.
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if `hint` is disjoint
    /// from `capacity`.
    ///
    /// # Panics
    ///
    /// Panics if `hint` is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::ensure_fits(SizeHint::exact(5), &(1..=3))
    ///     .expect_err("bounds should be disjoint");
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    pub fn ensure_fits<I: Iterator>(capacity: SizeHint, iter: &I) -> Result<(), Self> {
        let hint = iter.size_hint().try_conv::<SizeHint>().expect("Invalid size hint");
        SizeHint::disjoint(capacity, hint).then_err(Self::bounds(capacity, hint))
    }

    /// Ensures `iter`'s [`size_hint`](`Iterator::size_hint`) [`overlaps`](SizeHint::overlaps)
    /// the [`FixedCap::CAP`] of `C`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator whose [`size_hint`](`Iterator::size_hint`) is checked.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the collection who's [`FixedCap::CAP`] is checked.
    /// * `I` - The type of the iterator to check.
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if `iter`'s
    /// [`size_hint`](`Iterator::size_hint`) is [`disjoint`](SizeHint::disjoint) with the
    /// [`FixedCap::CAP`] of `C`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [`size_hint`](`Iterator::size_hint`) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::ensure_fits_in::<[i32; 5], _>(&(1..=3))
    ///     .expect_err("bounds should be disjoint");
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    pub fn ensure_fits_in<C: FixedCap, I: Iterator>(iter: &I) -> Result<(), Self> {
        Self::ensure_fits(C::CAP, iter)
    }

    /// Ensures `iter`'s [`size_hint`](`Iterator::size_hint`) is compatible with
    /// the [`remaining_cap`](RemainingCap::remaining_cap) of `collection`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator whose [`size_hint`](`Iterator::size_hint`) is checked.
    /// * `collection` - The collection whose [`remaining_cap`](RemainingCap::remaining_cap)
    ///   is checked.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the collection to check.
    /// * `I` - The type of the iterator to check.
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if `iter`'s
    /// [`size_hint`](`Iterator::size_hint`) is [`disjoint`](SizeHint::disjoint) with the
    /// [`remaining_cap`](RemainingCap::remaining_cap) of `collection`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [`size_hint`](`Iterator::size_hint`) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// # use arrayvec::ArrayVec;
    /// let err = CapacityError::<i32>::ensure_fits_into::<ArrayVec<i32, 5>, _>(&(1..=7), &ArrayVec::new())
    ///     .expect_err("bounds should be disjoint");
    ///
    /// assert_eq!(err.capacity, SizeHint::at_most(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(7) });
    /// ```
    pub fn ensure_fits_into<C: RemainingCap, I: Iterator>(iter: &I, collection: &C) -> Result<(), Self> {
        Self::ensure_fits(collection.remaining_cap(), iter)
    }

    /// Creates a new [`CapacityError`] indicating that the iterator overflowed
    /// `capacity`.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity that was overflowed.
    /// * `overflow` - The item that overflowed `capacity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::overflow(SizeHint::exact(5), 2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Overflow { overflow: 2 });
    /// ```
    #[must_use]
    pub const fn overflow(capacity: SizeHint, overflow: T) -> Self {
        Self { capacity, kind: CapacityErrorKind::Overflow { overflow } }
    }

    /// Creates a new [`CapacityError`] indicating that the iterator collection
    /// overflowed the [`FixedCap::CAP`] of an empty collection of type `C`.
    ///
    /// # Arguments
    ///
    /// * `overflow` - The item that overflowed.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the collection who's [`FixedCap::CAP`] overflowed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::collect_overflow::<[i32; 5]>(2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Overflow { overflow: 2 });
    /// ```
    #[must_use]
    pub const fn collect_overflow<C: FixedCap>(overflow: T) -> Self {
        Self { capacity: C::CAP, kind: CapacityErrorKind::Overflow { overflow } }
    }

    /// Creates a new [`CapacityError`] indicating that collection extension overflowed
    /// the maximum capacity, and the target collection is left in a full state,
    /// with [`ZERO`](SizeHint::ZERO) capacity.
    ///
    /// # Arguments
    ///
    /// * `overflow` - The item that was overflow due to overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::extend_overflow(2);
    ///
    /// assert_eq!(err.capacity, SizeHint::ZERO);
    /// assert_eq!(err.kind, CapacityErrorKind::Overflow { overflow: 2 });
    /// ```
    #[must_use]
    pub const fn extend_overflow(overflow: T) -> Self {
        Self { capacity: SizeHint::ZERO, kind: CapacityErrorKind::Overflow { overflow } }
    }

    /// Creates a new [`CapacityError`] indicating that the iterator produced
    /// fewer items than the minimum required by `capacity`.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The capacity that was underflowed.
    /// * `count` - The number of items that were produced.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::underflow(SizeHint::exact(5), 2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    pub const fn underflow(capacity: SizeHint, count: usize) -> Self {
        Self { capacity, kind: CapacityErrorKind::Underflow { count } }
    }

    /// Creates a new [`CapacityError`] indicating that the iterator produced
    /// fewer items than the minimum required by the [`FixedCap::CAP`] of an
    /// empty collection of type `C`.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of items that were produced.
    ///
    /// # Type Parameters
    ///
    /// * `C` - The type of the collection to check.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// use collect_failable::errors::types::SizeHint;
    /// let err = CapacityError::<i32>::collect_underflow::<[i32; 5]>(2);
    ///
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    pub const fn collect_underflow<C: FixedCap>(count: usize) -> Self {
        Self { capacity: C::CAP, kind: CapacityErrorKind::Underflow { count } }
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
        Self::extend_overflow(err.element())
    }
}
