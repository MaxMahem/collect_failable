use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::ops::Deref;

use alloc::boxed::Box;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::Short;
use tap::Pipe;

/// An error that occurs when extending a tuple of collections fails.
///
/// This error preserves the error from the side that failed, any unevaluated item from the other side,
/// and the remaining iterator for error recovery.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`]
/// implementation into a hidden `TupleExtendErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`TupleExtendError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `E`: The error type from the failing collection.
/// - `U`: The type of the unevaluated item from the other collection.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct TupleExtendError<E, U, I> {
    #[cfg(doc)]
    /// The error that occurred during extension.
    pub error: E,
    #[cfg(doc)]
    /// The unevaluated item from the other side, if any.
    pub unevaluated: Option<U>,
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: Box<TupleExtendErrorData<E, U, I>>,
    #[cfg(not(doc))]
    _subdef: [(); {
        /// The internal data of a [`TupleExtendError`].
        #[doc(hidden)]
        pub struct TupleExtendErrorData<E, U, I> {
            /// The error that occurred during extension.
            pub error: E,
            /// The unevaluated item from the other side, if any.
            pub unevaluated: Option<U>,
            /// The remaining iterator after the error occurred
            pub remaining: I,
        }
    }],
}

impl<E, U, I> TupleExtendError<E, U, I> {
    /// Creates a new [`TupleExtendError`].
    #[doc(hidden)]
    #[must_use]
    pub fn new(error: E, unevaluated: Option<U>, remaining: I) -> Self {
        TupleExtendErrorData { error, unevaluated, remaining }.pipe(Box::new).pipe(|data| Self { data, _subdef: () })
    }

    /// Consumes the error, returning the data.
    #[must_use]
    pub fn into_data(self) -> TupleExtendErrorData<E, U, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<E, U, I> Deref for TupleExtendError<E, U, I> {
    type Target = TupleExtendErrorData<E, U, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<E: Debug, U, I> Debug for TupleExtendError<E, U, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TupleExtendError")
            .field("error", &self.error)
            .field_type::<U, Short>("unevaluated")
            .field_type::<I, Short>("remaining")
            .finish()
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All fields actually covered")]
impl<E: Debug, U, I> Debug for TupleExtendErrorData<E, U, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TupleExtendErrorData")
            .field("error", &self.error)
            .field_type::<U, Short>("unevaluated")
            .field_type::<I, Short>("remaining")
            .finish()
    }
}

impl<E: Display, U, I> Display for TupleExtendError<E, U, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error while extending collection: {}", self.error)
    }
}

impl<E, U, I> Error for TupleExtendError<E, U, I>
where
    E: Error + 'static,
    U: Debug,
    I: Debug,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
