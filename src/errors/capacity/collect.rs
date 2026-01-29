use fluent_result::into::IntoResult;

use crate::errors::CollectError;
use crate::errors::capacity::error::CapacityError;
use crate::errors::capacity::traits::{FixedCap, RemainingCap};
use crate::errors::types::SizeHint;

#[cfg(doc)]
use crate::errors::capacity::CapacityErrorKind;

/// Specialization of [`CollectError`] for [`CapacityError`].
///
/// This type is for collection or extension errors due to a collection's capacity constraints.
/// [`CapacityError::kind`] identifies the specific failure.
///
/// # Types
///
/// - [`Bounds`](CapacityErrorKind::Bounds) - The iterator's [`size_hint`](Iterator::size_hint)
///   indicates it is incompatible with the containers capacity. In this case,
///   the iterator should be unmodified, and no items should be collected. The
///   [`collected`](CollectError::collected) will be [`Default`](Default::default())
///   constructed.
/// - [`Overflow`](CapacityErrorKind::Overflow) - The collection or extension
///   operation overflowed the collection bounds. The first item that overflowed
///   is captured in [`overflow`](CollectError::overflow), but there may be more
///   overflowing items remaining in the iterator.
/// - [`Underflow`](CapacityErrorKind::Underflow) - The collection or extension
///   operation underflowed the collection bounds. The count of underflowing
///   items is derived from [`collected`](CollectError::collected).
///
/// # Type Parameters
///
/// - `I`: The type of the iterator that was used to produce the values.
/// - `C`: The type of the collection that was used to collect the values.
///
/// # Data Recovery
///
/// If `C` implements [`IntoIterator`], this type implements [`IntoIterator`]
/// as well, allowing the data in the original iterator to be reconstructed
/// from [`CollectError::iter`], [`CollectError::collected`], and any
/// overflowing items.
///
/// # Examples
///
/// ```rust
/// # use collect_failable::errors::capacity::CapacityError;
/// # use collect_failable::errors::types::SizeHint;
/// # use collect_failable::errors::CollectError;
/// let error = CollectError::new(1..=1, vec![2], CapacityError::overflow(SizeHint::exact(1), 3));
///
/// let values: Vec<_> = error.into_iter().collect();
///
/// assert_eq!(values, vec![3, 2, 1]);
/// ```
impl<I: Iterator, C> CollectError<I, C, CapacityError<I::Item>> {
    /// Creates a new [`Bounds`](CapacityErrorKind::Bounds) [`CollectError`] for `iter` whose
    /// [`size_hint`](Iterator::size_hint) is incompatible with `cap`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] that failed the bounds check
    /// * `cap` - The allowed capacity of the collection
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s size hint is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::<_, Vec<()>, _>::bounds(1..=3, SizeHint::exact(2));
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, vec![]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    #[must_use]
    pub fn bounds(iter: I, cap: SizeHint) -> Self
    where
        C: Default,
    {
        let hint = iter.size_hint().try_into().expect("Invalid size hint");
        Self::new(iter, C::default(), CapacityError::bounds(cap, hint))
    }

    /// Ensures `C`'s [`FixedCap::CAP`] [`overlaps`](SizeHint::overlaps) `iter`'s
    /// [`size_hint`](`Iterator::size_hint`).
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if
    /// `iter`'s [`size_hint`](`Iterator::size_hint`) is [`disjoint`](SizeHint::disjoint)
    /// with `C`'s [`FixedCap::CAP`]. Otherwise returns the given `iter`.
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
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::<_, [i32; 5], _>::ensure_fits_in(1..=3)
    ///     .expect_err("Should fail bounds check");
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, [0, 0, 0, 0, 0]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    pub fn ensure_fits_in(iter: I) -> Result<I, Self>
    where
        C: Default + FixedCap,
    {
        match CapacityError::ensure_fits_in::<C, _>(&iter) {
            Ok(()) => Ok(iter),
            Err(error) => Self::new(iter, C::default(), error).into_err(),
        }
    }

    /// Ensures `collection`'s [`remaining_cap`](RemainingCap::remaining_cap)
    /// [`overlaps`](SizeHint::overlaps) `iter`'s [`size_hint`](Iterator::size_hint).
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check
    /// * `collection` - The collection to check - also determines the remaining capacity
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if the `iter`'s
    /// [`size_hint`](Iterator::size_hint) is [`disjoint`](SizeHint::disjoint) with the
    /// `collection`'s [`remaining_cap`](RemainingCap::remaining_cap). Otherwise returns the
    /// given `iter`.
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
    /// # use collect_failable::errors::CollectError;
    /// # use arrayvec::ArrayVec;
    /// let array = ArrayVec::<i32, 5>::new();
    /// let error = CollectError::ensure_fits_into(1..=6, &array)
    ///     .expect_err("Should fail bounds check");
    ///
    /// assert_eq!(error.iter, 1..=6);
    /// assert_eq!(error.collected, ArrayVec::<i32, 5>::new());
    /// assert_eq!(error.error.capacity, SizeHint::at_most(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(6) });
    /// ```
    pub fn ensure_fits_into(iter: I, collection: &C) -> Result<I, Self>
    where
        C: RemainingCap + Default,
    {
        match CapacityError::ensure_fits_into(&iter, collection) {
            Ok(()) => Ok(iter),
            Err(error) => Self::new(iter, C::default(), error).into_err(),
        }
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`]
    /// for collection failures when `iter` produced more items than `capacity`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `overflow` - The overflow item that was consumed but couldn't be added to the collection
    /// * `cap` - The allowed capacity of the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::overflow(1..=3, vec![1, 2], 3, SizeHint::exact(2));
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 3 });
    /// ```
    #[must_use]
    pub fn overflow(iter: I, collected: C, overflow: I::Item, cap: SizeHint) -> Self {
        Self::new(iter, collected, CapacityError::overflow(cap, overflow))
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`]
    /// for collection failures when `iter` produced more items than the
    /// [`FixedCap::CAP`] of an empty collection of type `C`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `overflow` - The overflow item that was consumed but couldn't be added
    ///   to the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::collect_overflow(1..=3, [1, 2], 3);
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, [1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 3 });
    /// ```
    #[must_use]
    pub fn collect_overflow(iter: I, collected: C, overflow: I::Item) -> Self
    where
        C: FixedCap,
    {
        Self::overflow(iter, collected, overflow, C::CAP)
    }

    /// Creates a new [`Underflow`](CapacityErrorKind::Underflow) [`CollectError`]
    /// for collection failures when `iter` produced fewer items than `cap`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after underflow occurred
    /// * `collected` - The values that were collected before underflow,
    ///   also provides the count of collected items
    /// * `cap` - The allowed capacity of the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::<_, Vec<_>, _>::underflow(1..=3, vec![1, 2], SizeHint::exact(2));
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    pub fn underflow(iter: I, collected: C, cap: SizeHint) -> Self
    where
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        let count = (&collected).into_iter().len();
        Self::new(iter, collected, CapacityError::underflow(cap, count))
    }

    /// Creates a new [`Underflow`](CapacityErrorKind::Underflow) [`CollectError`] for
    /// collection failures when `iter` produced fewer items than the
    /// [fixed capacity](FixedCap::CAP) of the `collected` type.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after underflow occurred
    /// * `collected` - The values that were collected before underflow,
    ///   also provides the count of collected items
    ///
    /// Note: Due to limitations of rust's type inference, it may be necessary to explicitly
    /// specify the type of `collected` when using this method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::<_, [i32; 5], _>::collect_underflow(1..=3, [1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, [1, 2, 3, 4, 5]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Underflow { count: 5 });
    /// ```
    #[must_use]
    pub fn collect_underflow(iter: I, collected: C) -> Self
    where
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
        C: FixedCap,
    {
        Self::underflow(iter, collected, C::CAP)
    }
}
