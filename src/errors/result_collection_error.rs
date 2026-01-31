use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

use alloc::boxed::Box;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::{Full, Short};
use display_as_debug::wrap::TypeNameResult;
use tap::Pipe;

/// An error that occurs when collecting an iterator of [`Result`]s fails.
///
/// This error preserves both the iterator error (the first [`Err`] encountered),
/// the partial collection result (which may be [`Ok`] with a partial collection
/// or [`Err`] with a collection error), and the remaining iterator.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref)
/// implementation into a hidden `ResultCollectErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`ResultCollectError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `E`: The type of the iterator error.
/// - `C`: The type of the collection.
/// - `CErr`: The type of the collection error.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct ResultCollectError<E, C, CErr, I> {
    #[cfg(doc)]
    /// The first [`Err`] encountered from the [`Result`] [`Iterator`]
    pub error: E,
    #[cfg(doc)]
    /// The partial collection result ([`Ok`] with partial collection, or [`Err`] with collection error)
    pub result: Result<C, CErr>,
    #[cfg(doc)]
    /// The remaining [`Iterator`] (items not yet consumed when the error occurred)
    pub iter: I,

    #[cfg(not(doc))]
    data: [Box<ResultCollectErrorData<E, C, CErr, I>>; {
        /// The internal data of a [`ResultCollectError`].
        #[doc(hidden)]
        pub struct ResultCollectErrorData<E, C, CErr, I> {
            /// The first [`Err`] encountered from the [`Result`] [`Iterator`]
            pub error: E,
            /// The partial collection result ([`Ok`] with partial collection, or [`Err`] with collection error)
            pub result: Result<C, CErr>,
            /// The remaining [`Iterator`] (items not yet consumed when the error occurred)
            pub iter: I,
        }
    }],
}

impl<E, C, CErr, I> ResultCollectError<E, C, CErr, I> {
    /// Creates a new [`ResultCollectError`] from an iterator error and collection result.
    #[must_use]
    #[doc(hidden)]
    pub fn new(error: E, result: Result<C, CErr>, iter: I) -> Self {
        ResultCollectErrorData { error, result, iter }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning a `ResultCollectErrorData` containing the
    /// [`ResultCollectError::error`], [`ResultCollectError::result`],
    /// and [`ResultCollectError::iter`].
    #[must_use]
    pub fn into_data(self) -> ResultCollectErrorData<E, C, CErr, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<E, C, CErr, I> core::ops::Deref for ResultCollectError<E, C, CErr, I> {
    type Target = ResultCollectErrorData<E, C, CErr, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<E: Debug, C, CErr: Debug, I> Debug for ResultCollectErrorData<E, C, CErr, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ResultCollectErrorData")
            .field("error", &self.error)
            .field("result", &TypeNameResult::borrow::<Short>(&self.result))
            .field_type::<I, Full>("iter")
            .finish()
    }
}

impl<E: Debug, C, CErr: Debug, I> Debug for ResultCollectError<E, C, CErr, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ResultCollectError")
            .field("error", &self.data.error)
            .field("result", &TypeNameResult::borrow::<Short>(&self.data.result))
            .field_type::<I, Full>("iter")
            .finish()
    }
}

impl<E: Display, C, CErr: Display, I> Display for ResultCollectError<E, C, CErr, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Iterator error: {}", self.data.error).and_then(|()| match &self.data.result {
            Err(e) => write!(f, "; Collection error: {e}"),
            _ => Ok(()),
        })
    }
}

impl<E: Error + 'static, C, CErr: Error, I> Error for ResultCollectError<E, C, CErr, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.data.error)
    }
}
