use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::ops::Deref;

use alloc::boxed::Box;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::{Full, Short};
use nameof::name_of;
use tap::Pipe;

/// An error that occurs when extending a tuple of collections fails.
///
/// This error preserves the error from the side that failed, any pending item from the other side,
/// and the remaining iterator for error recovery.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`]
/// implementation into a hidden `TupleExtendErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`TupleExtendError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `E`: The error type from the failing collection.
/// - `P`: The type of the pending item from the other collection.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct TupleExtendError<E, P, I> {
    #[cfg(doc)]
    /// The error that occurred during extension.
    pub error: E,
    #[cfg(doc)]
    /// The pending item from the other side, if any.
    pub pending: Option<P>,
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: [Box<TupleExtendErrorData<E, P, I>>; {
        /// The internal data of a [`TupleExtendError`].
        #[doc(hidden)]
        pub struct TupleExtendErrorData<E, P, I> {
            /// The error that occurred during extension.
            pub error: E,
            /// The pending item from the other side, if any.
            pub pending: Option<P>,
            /// The remaining iterator after the error occurred
            pub remaining: I,
        }
    }],
}

impl<E, P, I> TupleExtendError<E, P, I> {
    /// Creates a new [`TupleExtendError`].
    #[doc(hidden)]
    #[must_use]
    pub fn new(error: E, pending: Option<P>, remaining: I) -> Self {
        TupleExtendErrorData { error, pending, remaining }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning the data.
    #[must_use]
    pub fn into_data(self) -> TupleExtendErrorData<E, P, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<E, P, I> Deref for TupleExtendError<E, P, I> {
    type Target = TupleExtendErrorData<E, P, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<E: Debug, P, I> Debug for TupleExtendError<E, P, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TupleExtendError")
            .field(name_of!(error in Self), &self.error)
            .field_type::<P, Short>(name_of!(pending in Self))
            .field_type::<I, Full>(name_of!(remaining in Self))
            .finish()
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All fields actually covered")]
impl<E: Debug, P, I> Debug for TupleExtendErrorData<E, P, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TupleExtendErrorData")
            .field(name_of!(error in Self), &self.error)
            .field_type::<P, Short>(name_of!(pending in Self))
            .field_type::<I, Full>(name_of!(remaining in Self))
            .finish()
    }
}

impl<E: Display, P, I> Display for TupleExtendError<E, P, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error while extending collection: {}", self.error)
    }
}

impl<E: Error + 'static, P, I> Error for TupleExtendError<E, P, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

#[doc(hidden)]
impl<E: Display, P, I> Display for TupleExtendErrorData<E, P, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error while extending collection: {}", self.error)
    }
}

#[doc(hidden)]
impl<E: Error + 'static, P, I> Error for TupleExtendErrorData<E, P, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
