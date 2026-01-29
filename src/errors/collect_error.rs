use alloc::boxed::Box;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::iter::Chain;
use core::ops::Deref;
use display_as_debug::fmt::DebugStructExt;
use display_as_debug::wrap::Short;

use tap::Pipe;

use super::ErrorItemProvider;

#[cfg(doc)]
use crate::errors::capacity::CapacityErrorKind;

/// An error that occurs when collecting an [`IntoIterator`] fails during its collection.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator.
/// - `C`: The type of the container for collected values. Note this this may or may not be
///   different from the container type that was the target of the collection operation.
/// - `E`: The type of the nested error.
///
/// # Data Recovery
///
/// This type is designed to capture all state in the event of a collection failure. If `E`
/// implements [`ErrorItemProvider`] and `C` implements [`IntoIterator`], (which all
/// implementations in this crate do) then this information can be used to recreate an iterator with
/// the same values as was originally provided via [`IntoIterator::into_iter`].
///
/// # Read-Only
///
/// Note that this type is *read-only*. The fields may be borrowed via a hidden [`Deref`]
/// implementation into a hidden `CollectErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`CollectError::into_data`] to get owned data.
#[subdef::subdef]
pub struct CollectError<I, C, E> {
    #[cfg(doc)]
    /// The iterator used to produce the collected values.
    /// This iterator may be untouched, partially or fully
    /// iterated, depending on the error.
    pub iter: I,
    #[cfg(doc)]
    /// The values that were collected.
    /// It may be empty if no items were collected.
    pub collected: C,
    #[cfg(doc)]
    /// The error that occurred.
    /// This value may contain an additional element which was produced
    /// by the iterator, but could not be added to the collection.
    pub error: E,

    #[cfg(not(doc))]
    data: [Box<CollectErrorData<I, C, E>>; {
        /// The internal data of a [`CollectError`].
        #[doc(hidden)]
        pub struct CollectErrorData<I, C, E> {
            /// The iterator used to produce the collected values.
            /// This iterator may be untouched, partially or fully
            /// iterated, depending on the error.
            pub iter: I,
            /// The values that were collected.
            /// It may be empty if no items were collected.
            pub collected: C,
            /// The error that occurred.
            /// This value may contain an additional element which was produced
            /// by the iterator, but could not be added to the collection.
            pub error: E,
        }
    }],
}

impl<I, C, E> CollectError<I, C, E> {
    /// Creates a new [`CollectError`].
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator used to produce the collected values.
    /// * `collected` - The values that were collected.
    /// * `error` - The error that occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::new(
    ///     1..=3,
    ///     vec![1, 2],
    ///     CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)),
    /// );
    ///
    /// assert_eq!(error.iter, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// ```
    pub fn new(iter: I, collected: C, error: E) -> Self {
        CollectErrorData { iter, collected, error }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning a `CollectErrorData` containing the [`CollectError::iter`],
    /// [`CollectError::collected`] values, and [`CollectError::error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::capacity::{CapacityError, CapacityErrorKind};
    /// # use collect_failable::errors::types::SizeHint;
    /// # use collect_failable::errors::CollectError;
    /// let error = CollectError::new(
    ///     1..=3,
    ///     vec![1, 2],
    ///     CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)),
    /// );
    ///
    /// let data = error.into_data();
    ///
    /// assert_eq!(data.iter, 1..=3);
    /// assert_eq!(data.collected, vec![1, 2]);
    /// assert_eq!(data.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// assert_eq!(data.error.capacity, SizeHint::exact(2));
    /// ```
    #[must_use]
    pub fn into_data(self) -> CollectErrorData<I, C, E> {
        *self.data
    }
}

impl<I, C, E> IntoIterator for CollectError<I, C, E>
where
    I: Iterator,
    C: IntoIterator<Item = I::Item>,
    E: ErrorItemProvider<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = Chain<Chain<core::option::IntoIter<I::Item>, C::IntoIter>, I>;

    /// Consumes the error and creates a new [Iterator] with the data it was
    /// created from, including the overflow item (if any), the [`collected`](CollectError::collected),
    /// values and the remaining [`iter`](CollectError::iter), in that order.
    fn into_iter(self) -> Self::IntoIter {
        let rejected = self.data.error.into_item();
        rejected.into_iter().chain(self.data.collected).chain(self.data.iter)
    }
}

impl<I, C, E: Display> Display for CollectError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Collection Error: {}", self.error)
    }
}

impl<I, C, E: Error + 'static> Error for CollectError<I, C, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<I, C, E: Debug> Debug for CollectError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CollectError")
            .field_type::<I, Short>("iter")
            .field_type::<C, Short>("collected")
            .field("error", &self.error)
            .finish()
    }
}

#[doc(hidden)]
impl<I, C, E> Deref for CollectError<I, C, E> {
    type Target = CollectErrorData<I, C, E>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<I, C, E: Debug> Debug for CollectErrorData<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CollectErrorData")
            .field_type::<I, Short>("iter")
            .field_type::<C, Short>("collected")
            .field("error", &self.error)
            .finish()
    }
}
