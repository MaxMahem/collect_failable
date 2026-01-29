use alloc::boxed::Box;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::iter::Chain;
use core::ops::Deref;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::wrap::Short;
use tap::Pipe;

use super::ErrorItemProvider;
use crate::errors::capacity::{CapacityError, RemainingCap};
use crate::errors::types::SizeHint;

#[cfg(doc)]
use crate::errors::capacity::CapacityErrorKind;

/// An error that occurs when extending a collection fails.
///
/// This error type is used by [`TryExtend`](crate::TryExtend) operations, which provide a
/// **basic error guarantee** — the collection may be modified on error, but remains valid.
///
/// # Type Parameters
///
/// - `I`: The type of the remaining iterator after the error occurred.
/// - `E`: The type of the nested error.
///
/// # Data Recovery
///
/// If `E` implements [`ErrorItemProvider`], the rejected item can be recovered. Combined with
/// the remaining [`iter`](ExtendError::iter), you can reconstruct the unconsumed portion
/// of the original iterator. However, items already added to the collection are not recoverable
/// from this error — they remain in the (modified) collection.
///
/// # Read-Only
///
/// This type is *read-only*. Fields are accessible via a hidden [`Deref`] implementation into
/// `ExtendErrorData`. Use [`ExtendError::into_data`] to get owned data.
///
/// # Examples
///
/// ```rust
/// # use collect_failable::TryExtend;
/// # use collect_failable::errors::ExtendError;
/// # use arrayvec::ArrayVec;
/// # use size_hinter::SizeHinter;
/// let mut array = ArrayVec::<i32, 3>::from_iter([1, 2]);
///
/// // The size hint is hidden to bypass bounds check and trigger overflow during iteration
/// let err = array.try_extend([3, 4, 5].into_iter().hide_size()).expect_err("should overflow");
///
/// let remaining: Vec<_> = err.into_iter().collect();
/// assert_eq!(remaining, vec![4, 5], "Rejected item and remaining items should be captured");
///
/// assert_eq!(*array, [1, 2, 3], "the collection is modified, but valid, no data is lost");
/// ```
#[subdef::subdef]
pub struct ExtendError<I, E> {
    #[cfg(doc)]
    /// The remaining iterator after the error occurred.
    pub iter: I,
    #[cfg(doc)]
    /// The error that occurred.
    pub error: E,

    #[cfg(not(doc))]
    data: [Box<ExtendErrorData<I, E>>; {
        /// The internal data of an [`ExtendError`].
        #[doc(hidden)]
        pub struct ExtendErrorData<I, E> {
            /// The remaining iterator after the error occurred.
            pub iter: I,
            /// The error that occurred.
            pub error: E,
        }
    }],
}

impl<I, E> ExtendError<I, E> {
    /// Creates a new [`ExtendError`].
    ///
    /// # Arguments
    ///
    /// * `iter` - The remaining iterator after the error occurred.
    /// * `error` - The error that occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let error = ExtendError::new(1..=3, CapacityError::<i32>::extend_overflow(4));
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.error.capacity, SizeHint::ZERO);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    pub fn new(iter: I, error: E) -> Self {
        ExtendErrorData { iter, error }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning the internal data containing the
    /// [`iter`](ExtendError::iter) and [`error`](ExtendError::error).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let data = ExtendError::new(1..=3, CapacityError::<i32>::extend_overflow(4)).into_data();
    ///
    /// assert_eq!(data.iter, 1..=3);
    /// assert_eq!(data.error.capacity, SizeHint::ZERO);
    /// assert_eq!(data.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    #[must_use]
    pub fn into_data(self) -> ExtendErrorData<I, E> {
        *self.data
    }
}

impl<I: Iterator> ExtendError<I, CapacityError<I::Item>> {
    /// Creates a new [`Bounds`](CapacityErrorKind::Bounds) [`ExtendError`] for `iter`,
    /// indicating its [`size_hint`](Iterator::size_hint) is incompatible with `cap`
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] that failed the bounds check
    /// * `cap` - The allowed capacity range for the target collection
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
    /// let error = ExtendError::bounds(1..=3, SizeHint::exact(2));
    ///
    /// assert_eq!(error.iter, 1..=3);
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
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if `iter`'s
    /// [`size_hint`](Iterator::size_hint) is [`disjoint`](SizeHint::disjoint) with `collection`'s
    /// [`remaining_cap`](RemainingCap::remaining_cap). Otherwise returns `iter`.
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
    /// assert_eq!(error.iter, 1..=6);
    /// assert_eq!(error.error.capacity, SizeHint::at_most(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(6) });
    /// ```
    pub fn ensure_fits_into<C: RemainingCap>(iter: I, collection: &C) -> Result<I, Self> {
        match CapacityError::ensure_fits_into(&iter, collection) {
            Ok(()) => Ok(iter),
            Err(error) => Err(Self::new(iter, error)),
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
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.error.capacity, SizeHint::ZERO);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    #[must_use]
    pub fn overflow(iter: I, overflow: I::Item) -> Self {
        Self::new(iter, CapacityError::overflow(SizeHint::ZERO, overflow))
    }
}

impl<I, E> IntoIterator for ExtendError<I, E>
where
    I: Iterator,
    E: ErrorItemProvider<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = Chain<core::option::IntoIter<I::Item>, I>;

    /// Consumes the error and returns an [`Iterator`] over the rejected item (if any)
    /// followed by the remaining [`iter`](ExtendError::iter).
    fn into_iter(self) -> Self::IntoIter {
        let rejected = self.data.error.into_item();
        rejected.into_iter().chain(self.data.iter)
    }
}

impl<I, E: Display> Display for ExtendError<I, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Extension Error: {}", self.error)
    }
}

impl<I, E: Error + 'static> Error for ExtendError<I, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<I, E: Debug> Debug for ExtendError<I, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ExtendError").field_type::<I, Short>("iter").field("error", &self.error).finish()
    }
}

#[doc(hidden)]
impl<I, E> Deref for ExtendError<I, E> {
    type Target = ExtendErrorData<I, E>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<I, E: Debug> Debug for ExtendErrorData<I, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ExtendErrorData").field_type::<I, Short>("iter").field("error", &self.error).finish()
    }
}
