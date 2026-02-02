#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::iter::Chain;
use core::ops::Deref;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::wrap::Full;
use nameof::name_of;
use tap::Pipe;

use super::ErrorItemProvider;

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
/// the remaining [`remain`](ExtendError::remain), you can reconstruct the unconsumed portion
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
pub struct ExtendError<I, E> {
    #[cfg(doc)]
    /// The remaining iterator.
    pub remain: I,
    #[cfg(doc)]
    /// The error that occurred.
    pub error: E,

    #[cfg(all(not(doc), feature = "alloc"))]
    data: Box<ExtendErrorData<I, E>>,
    #[cfg(all(not(doc), not(feature = "alloc")))]
    data: ExtendErrorData<I, E>,
}

/// The internal data of an [`ExtendError`].
#[doc(hidden)]
pub struct ExtendErrorData<I, E> {
    /// The remaining iterator.
    pub remain: I,
    /// The error that occurred.
    pub error: E,
}

impl<I, E> ExtendError<I, E> {
    /// Creates a new [`ExtendError`].
    ///
    /// # Arguments
    ///
    /// * `remain` - The remaining iterator after the error occurred.
    /// * `error` - The error that occurred.
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
    #[cfg(feature = "alloc")]
    pub fn new(remain: I, error: E) -> Self {
        ExtendErrorData { remain, error }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Creates a new [`ExtendError`].
    ///
    /// # Arguments
    ///
    /// * `remain` - The remaining iterator after the error occurred.
    /// * `error` - The error that occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let error = ExtendError::new(1..=3, 4);
    ///
    /// assert_eq!(error.remain, 1..=3);
    /// assert_eq!(error.error.capacity, SizeHint::ZERO);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    #[must_use]
    #[cfg(not(feature = "alloc"))]
    pub fn new(remain: I, error: E) -> Self {
        ExtendErrorData { remain, error }.pipe(|data| Self { data })
    }

    /// Consumes the error, returning the internal data containing the
    /// [`remain`](ExtendError::remain) and [`error`](ExtendError::error).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let data = ExtendError::overflow(1..=3, 4).into_data();
    ///
    /// assert_eq!(data.remain, 1..=3);
    /// assert_eq!(data.error.capacity, SizeHint::ZERO);
    /// assert_eq!(data.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn into_data(self) -> ExtendErrorData<I, E> {
        *self.data
    }

    /// Consumes the error, returning the internal data containing the
    /// [`remain`](ExtendError::remain) and [`error`](ExtendError::error).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::ExtendError;
    /// let data = ExtendError::new(1..=3, 4).into_data();
    ///
    /// assert_eq!(data.remain, 1..=3);
    /// assert_eq!(data.error.capacity, SizeHint::ZERO);
    /// assert_eq!(data.error.kind, CapacityErrorKind::Overflow { overflow: 4 });
    /// ```
    #[must_use]
    #[cfg(not(feature = "alloc"))]
    pub fn into_data(self) -> ExtendErrorData<I, E> {
        self.data
    }
}

impl<I: Iterator, E> IntoIterator for ExtendError<I, E>
where
    E: ErrorItemProvider<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = Chain<core::option::IntoIter<I::Item>, I>;

    /// Consumes the error and returns an [`Iterator`] over the rejected item (if any)
    /// followed by the remaining [`remain`](ExtendError::remain).
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
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
        f.debug_struct("ExtendError")
            .field_type::<I, Full>(name_of!(remain in Self))
            .field(name_of!(error in Self), &self.error)
            .finish()
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
        f.debug_struct("ExtendErrorData")
            .field_type::<I, Full>(name_of!(remain in Self))
            .field(name_of!(error in Self), &self.error)
            .finish()
    }
}
#[doc(hidden)]
impl<I, E: Display> Display for ExtendErrorData<I, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Extension Error: {}", self.error)
    }
}

#[doc(hidden)]
impl<I, E: Error + 'static> Error for ExtendErrorData<I, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Consumes the error and returns an [`Iterator`] over the rejected item (if any)
/// followed by the remaining [`remain`](ExtendError::remain).
#[doc(hidden)]
impl<I: Iterator, E> IntoIterator for ExtendErrorData<I, E>
where
    E: ErrorItemProvider<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = Chain<core::option::IntoIter<I::Item>, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.error.into_item().into_iter().chain(self.remain)
    }
}
