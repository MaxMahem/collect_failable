use fluent_result::into::IntoResult;

use crate::errors::ExtendError;
use crate::errors::capacity::error::CapacityError;
use crate::errors::capacity::traits::RemainingCap;
use crate::errors::types::SizeHint;

#[cfg(doc)]
use crate::errors::capacity::CapacityErrorKind;

/// Specialization of [`ExtendError`] for [`CapacityError`] errors.
///
/// This is used when collecting into a collection that has a limited capacity,
/// such as [`ArrayVec`](arrayvec::ArrayVec).
///
/// # Type Parameters
///
/// * `I` - The [`Iterator`] that violates the capacity bounds
impl<I: Iterator> ExtendError<I, CapacityError<I::Item>> {
    /// Creates a new [`Bounds`](CapacityErrorKind::Bounds) [`ExtendError`] for
    /// `iter`, indicating its [`size_hint`](Iterator::size_hint) is incompatible
    /// with `cap`
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] that failed the bounds check
    /// * `cap` - The allowed capacity range for the target collection
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `iter`'s [`size_hint`](Iterator::size_hint) is invalid.
    /// - `cap` and `iter`'s size hint [`overlap`](SizeHint::overlaps).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let error = ExtendError::bounds(1..=3, SizeHint::exact(2));
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    #[must_use]
    pub fn bounds(iter: I, cap: SizeHint) -> Self {
        let hint = iter.size_hint().try_into().expect("Invalid size hint");

        Self::new(iter, CapacityError::bounds(cap, hint))
    }

    /// Ensures `collection`'s [`remaining_cap`](RemainingCap::remaining_cap)
    /// [`overlaps`](SizeHint::overlaps) `iter`'s [`size_hint`](Iterator::size_hint).
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check
    /// * `collection` - The collection to check - also determines [`remaining_cap`](RemainingCap::remaining_cap)
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if
    /// `iter`'s [`size_hint`](Iterator::size_hint) is [`disjoint`](SizeHint::disjoint)
    /// with `collection`'s [`remaining_cap`](RemainingCap::remaining_cap).
    /// Otherwise returns `iter`.
    ///
    /// # Panics
    ///
    /// Panics if the `iter`'s [`size_hint`](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// # use arrayvec::ArrayVec;
    /// let array = ArrayVec::<i32, 5>::new();
    /// let error = ExtendError::ensure_fits_into(1..=6, &array)
    ///     .expect_err("Should fail bounds check");
    ///
    /// assert_eq!(error.remain, 1..=6);
    /// assert_eq!(error.error.capacity, SizeHint::at_most(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(6) });
    /// ```
    pub fn ensure_fits_into<C: RemainingCap>(iter: I, collection: &C) -> Result<I, Self> {
        match CapacityError::ensure_fits(&iter, collection.remaining_cap()) {
            Ok(()) => Ok(iter),
            Err(error) => Self::new(iter, error).into_err(),
        }
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`ExtendError`] for
    /// extension failures when `iter` produced more items than the target's capacity.
    ///
    /// This implies there is [zero](SizeHint::ZERO) remaining capacity.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [`Iterator`] after overflow occurred
    /// * `overflow` - The item that overflowed the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let error = ExtendError::overflow(1..=3, 4);
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.error.capacity, SizeHint::ZERO);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    #[must_use]
    pub fn overflow(iter: I, overflow: I::Item) -> Self {
        Self::new(iter, CapacityError::overflow(SizeHint::ZERO, overflow))
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`ExtendError`]
    /// for extension failures when `iter` produced more items than the
    /// [`remaining_cap`](RemainingCap::remaining_cap) of `collection`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [`Iterator`] after overflow occurred
    /// * `overflow` - The item that overflowed the collection
    /// * `collection` - The collection that overflowed
    ///
    /// # Panics
    ///
    /// Panics if `collection` has no upper bound.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// # use arrayvec::ArrayVec;
    /// let collection = ArrayVec::<i32, 5>::from_iter([1, 2]);
    /// let error = ExtendError::overflow_remaining_cap(1..=3, 3, &collection);
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.error.capacity, SizeHint::at_most(3));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 3 });
    /// ```
    #[must_use]
    pub fn overflow_remaining_cap<C: RemainingCap>(iter: I, overflow: I::Item, collection: &C) -> Self {
        Self::new(iter, CapacityError::overflow(collection.remaining_cap(), overflow))
    }
}
