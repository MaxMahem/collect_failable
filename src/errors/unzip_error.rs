use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::ops::Deref;

use alloc::boxed::Box;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::Short;
use display_as_debug::wrap::TypeNameOption;
use tap::Pipe;

use crate::TryExtendOne;

/// An error that occurs when unzipping an iterator into two collections fails.
///
/// This error preserves the incomplete collection from the side that succeeded,
/// along with the error from the side that failed, and the remaining iterator.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`]
/// implementation into a hidden `UnzipErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`UnzipError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `Failed`: The type of the collection that failed to extend.
/// - `Partial`: The type of the collection that successfully extended.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct UnzipError<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
{
    #[cfg(doc)]
    /// The error that occurred during extension.
    pub error: Failed::Error,
    #[cfg(doc)]
    /// The partial collection from the failed side.
    pub failed: Failed,
    #[cfg(doc)]
    /// The incomplete collection from the successful side.
    pub partial: Partial,
    #[cfg(doc)]
    /// The pending item from the successful side, if any.
    pub pending: Option<Partial::Item>,
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: [Box<UnzipErrorData<Failed, Partial, I>>; {
        /// The internal data of an [`UnzipError`].
        #[doc(hidden)]
        pub struct UnzipErrorData<Failed, Partial, I>
        where
            Failed: TryExtendOne,
            Partial: TryExtendOne,
        {
            /// The error that occurred during extension.
            pub error: Failed::Error,
            /// The partial collection from the failed side.
            pub failed: Failed,
            /// The incomplete collection from the successful side.
            pub partial: Partial,
            /// The pending item from the successful side, if any.
            pub pending: Option<Partial::Item>,
            /// The remaining iterator after the error occurred
            pub remaining: I,
        }
    }],
}

impl<Failed, Partial, I> UnzipError<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
{
    /// Creates a new [`UnzipError`].
    #[doc(hidden)]
    pub fn new(error: Failed::Error, failed: Failed, partial: Partial, pending: Option<Partial::Item>, remaining: I) -> Self {
        UnzipErrorData { error, failed, partial, pending, remaining }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning the data.
    #[must_use]
    pub fn into_data(self) -> UnzipErrorData<Failed, Partial, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<Failed, Partial, I> Deref for UnzipError<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
{
    type Target = UnzipErrorData<Failed, Partial, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<Failed, Partial, I> Debug for UnzipErrorData<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
    Failed::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UnzipErrorData")
            .field("error", &self.error)
            .field_type::<Failed, Short>("failed")
            .field_type::<Partial, Short>("partial")
            .field("pending", &TypeNameOption::borrow::<Short>(&self.pending))
            .field_type::<I, Short>("remaining")
            .finish()
    }
}

impl<Failed, Partial, I> Debug for UnzipError<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
    Failed::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UnzipError")
            .field("error", &self.error)
            .field_type::<Failed, Short>("failed")
            .field_type::<Partial, Short>("partial")
            .field("pending", &TypeNameOption::borrow::<Short>(&self.pending))
            .field_type::<I, Short>("remaining")
            .finish()
    }
}

impl<Failed, Partial, I> Display for UnzipError<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
    Failed::Error: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed while unzipping collection: {}", self.error)
    }
}

impl<Failed, Partial, I> Error for UnzipError<Failed, Partial, I>
where
    Failed: TryExtendOne,
    Partial: TryExtendOne,
    Failed::Error: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
