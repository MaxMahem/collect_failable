use fluent_result::into::IntoResult;

use crate::errors::CollectError;
use crate::errors::capacity::error::CapacityError;
use crate::errors::capacity::traits::{FixedCap, RemainingCap};
use crate::errors::types::SizeHint;

#[cfg(doc)]
use crate::errors::capacity::CapacityErrorKind;

/// Specialization of [`CollectError`] for [`CapacityError`].
///
/// This type is for errors that occur during collection or extension because
/// the operation violates the collection's capacity constraints.
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
/// - `I`: The type of the [`Iterator`] that was used to iterate the values.
/// - `C`: The type of the collection that was used to collect the values.
///   Note the collection type may not be the same as the type that was being
///   extended or collected into.
///
/// # Data Recovery
///
/// If `C` implements [`IntoIterator`], this type implements [`IntoIterator`]
/// as well, allowing the data in the original iterator to be reconstructed
/// from [`CollectError::remain`], [`CollectError::collected`], and any
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
    /// assert_eq!(error.remain, 1..=3);
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

    /// Ensures `Col`'s [`FixedCap::CAP`] [`overlaps`](SizeHint::overlaps) `iter`'s
    /// [`size_hint`](`Iterator::size_hint`).
    ///
    /// Note that success on this method does not guarantee that `iter` will fit
    /// in `Col`'s [`FixedCap::CAP`], only that the [`size_hint`](`Iterator::size_hint`)
    /// does not indicate that it will not fit.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check
    ///
    /// # Type Parameters
    ///
    /// * `Col` - The collection type to check
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
    /// CollectError::<_, [i32; 5], _>::ensure_fits_in::<[i32; 5]>(1..=5).expect("Should fit");
    ///
    /// let error = CollectError::<_, [i32; 5], _>::ensure_fits_in::<[i32; 5]>(1..=3)
    ///     .expect_err("Should not fit");
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.collected, [0, 0, 0, 0, 0]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    pub fn ensure_fits_in<Col: FixedCap>(iter: I) -> Result<I, Self>
    where
        C: Default,
    {
        match CapacityError::ensure_fits(&iter, Col::CAP) {
            Err(error) => Self::new(iter, C::default(), error).into_err(),
            Ok(()) => Ok(iter),
        }
    }

    /// Ensures `collection`'s [`remaining_cap`](RemainingCap::remaining_cap)
    /// [`overlaps`](SizeHint::overlaps) `iter`'s [`size_hint`](Iterator::size_hint).
    ///
    /// Note that success on this method does not guarantee that `iter` will fit
    /// in `collection`'s [`remaining_cap`](RemainingCap::remaining_cap), only that the
    /// [`size_hint`](`Iterator::size_hint`) does not indicate that it will not fit.
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
    /// assert_eq!(error.remain, 1..=6);
    /// assert_eq!(error.collected, ArrayVec::<i32, 5>::new());
    /// assert_eq!(error.error.capacity, SizeHint::at_most(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(6) });
    /// ```
    pub fn ensure_fits_into(iter: I, collection: &C) -> Result<I, Self>
    where
        C: RemainingCap + Default,
    {
        match CapacityError::ensure_fits(&iter, collection.remaining_cap()) {
            Err(error) => Self::new(iter, C::default(), error).into_err(),
            Ok(()) => Ok(iter),
        }
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`]
    /// for collection failures when `iter` produced more items than `capacity`.
    ///
    /// # Arguments
    ///
    /// * `remain` - The remaining [Iterator] after overflow occurred
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
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 3 });
    /// ```
    #[must_use]
    pub fn overflow(remain: I, collected: C, overflow: I::Item, cap: SizeHint) -> Self {
        Self::new(remain, collected, CapacityError::overflow(cap, overflow))
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`]
    /// for collection failures when `iter` produced more items than the
    /// [`remaining_cap`](RemainingCap::remaining_cap) of `collection`.
    ///
    /// # Arguments
    ///
    /// * `remain` - The remaining [Iterator] after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `overflow` - The overflow item that was consumed but couldn't be added to the collection
    /// * `collection` - The collection that overflowed
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// # use arrayvec::ArrayVec;
    /// let collection = ArrayVec::<_, 5>::from_iter([1, 2]);
    /// let error = CollectError::overflow_remaining_cap(1..=3, [1, 2], 3, &collection);
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.collected, [1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::at_most(3));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 3 });
    /// ```
    #[must_use]
    pub fn overflow_remaining_cap<Col: RemainingCap>(remain: I, collected: C, overflow: I::Item, collection: &Col) -> Self {
        Self::overflow(remain, collected, overflow, collection.remaining_cap())
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`]
    /// for collection failures when `iter` produced more items than the
    /// [`FixedCap::CAP`] of an empty collection of type `Col`.
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining [Iterator] after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `overflow` - The overflow item that was consumed but couldn't be added
    ///   to the collection
    ///
    /// # Type Arguments
    ///
    /// * `Col` - The type of the collection that overflowed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::collect_overflow::<[i32; 2]>(1..=3, [1, 2], 3);
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.collected, [1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 3 });
    /// ```
    #[must_use]
    pub fn collect_overflow<Col: FixedCap>(iter: I, collected: C, overflow: I::Item) -> Self {
        Self::overflow(iter, collected, overflow, Col::CAP)
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
    /// let error = CollectError::<_, Vec<_>, _>::underflow(1..=3, vec![1, 2], SizeHint::exact(3));
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(3));
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
}
