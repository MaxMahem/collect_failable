#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::ops::Deref;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::{Full, Short};
use display_as_debug::wrap::TypeNameOption;
use nameof::name_of;
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
/// - `E`: The error type from the failing collection.
/// - `Failed`: The type of the collection that failed to extend.
/// - `Partial`: The type of the collection that successfully extended.
/// - `Pending`: The type of the pending item from the successful side.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct UnzipError<E, Failed, Partial, Pending, I> {
    #[cfg(doc)]
    /// The error that occurred during extension.
    pub error: E,
    #[cfg(doc)]
    /// The partial collection from the failed side.
    pub failed: Failed,
    #[cfg(doc)]
    /// The incomplete collection from the successful side.
    pub partial: Partial,
    #[cfg(doc)]
    /// The pending item from the successful side, if any.
    pub pending: Option<Pending>,
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(all(not(doc), feature = "alloc"))]
    data: Box<UnzipErrorData<E, Failed, Partial, Pending, I>>,
    #[cfg(all(not(doc), not(feature = "alloc")))]
    data: UnzipErrorData<E, Failed, Partial, Pending, I>,
}

/// The internal data of an [`UnzipError`].
#[doc(hidden)]
pub struct UnzipErrorData<E, Failed, Partial, Pending, I> {
    /// The error that occurred during extension.
    pub error: E,
    /// The partial collection from the failed side.
    pub failed: Failed,
    /// The incomplete collection from the successful side.
    pub partial: Partial,
    /// The pending item from the successful side, if any.
    pub pending: Option<Pending>,
    /// The remaining iterator after the error occurred
    pub remaining: I,
}

#[doc(hidden)]
impl<Failed: TryExtendOne, Partial: TryExtendOne, I: Iterator>
    UnzipError<Failed::Error, Failed, Partial, Partial::Item, I>
{
    /// Creates a new [`UnzipError`].
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn new(
        error: Failed::Error,
        failed: Failed,
        partial: Partial,
        pending: Option<Partial::Item>,
        remaining: I,
    ) -> Self {
        UnzipErrorData { error, failed, partial, pending, remaining }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Creates a new [`UnzipError`].
    #[must_use]
    #[cfg(not(feature = "alloc"))]
    pub fn new(
        error: Failed::Error,
        failed: Failed,
        partial: Partial,
        pending: Option<Partial::Item>,
        remaining: I,
    ) -> Self {
        UnzipErrorData { error, failed, partial, pending, remaining }.pipe(|data| Self { data })
    }
}

impl<E, Failed, Partial, Pending, I> UnzipError<E, Failed, Partial, Pending, I> {
    /// Consumes the error, returning the data.
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn into_data(self) -> UnzipErrorData<E, Failed, Partial, Pending, I> {
        *self.data
    }

    /// Consumes the error, returning the data.
    #[must_use]
    #[cfg(not(feature = "alloc"))]
    pub fn into_data(self) -> UnzipErrorData<E, Failed, Partial, Pending, I> {
        self.data
    }
}

#[doc(hidden)]
impl<E, Failed, Partial, Pending, I> Deref for UnzipError<E, Failed, Partial, Pending, I> {
    type Target = UnzipErrorData<E, Failed, Partial, Pending, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<E: Debug, Failed, Partial, Pending, I> Debug for UnzipErrorData<E, Failed, Partial, Pending, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UnzipErrorData")
            .field(name_of!(error in Self), &self.error)
            .field_type::<Failed, Short>(name_of!(failed in Self))
            .field_type::<Partial, Short>(name_of!(partial in Self))
            .field(name_of!(pending in Self), &TypeNameOption::borrow::<Short>(&self.pending))
            .field_type::<I, Full>(name_of!(remaining in Self))
            .finish()
    }
}

impl<E: Debug, Failed, Partial, Pending, I> Debug for UnzipError<E, Failed, Partial, Pending, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UnzipError")
            .field(name_of!(error in Self), &self.error)
            .field_type::<Failed, Short>(name_of!(failed in Self))
            .field_type::<Partial, Short>(name_of!(partial in Self))
            .field(name_of!(pending in Self), &TypeNameOption::borrow::<Short>(&self.pending))
            .field_type::<I, Full>(name_of!(remaining in Self))
            .finish()
    }
}

impl<E: Display, Failed, Partial, Pending, I> Display for UnzipError<E, Failed, Partial, Pending, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed while unzipping collection: {}", self.error)
    }
}

impl<E: Error + 'static, Failed, Partial, Pending, Remaining> Error
    for UnzipError<E, Failed, Partial, Pending, Remaining>
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
#[doc(hidden)]
impl<E: Display, Failed, Partial, Pending, I> Display for UnzipErrorData<E, Failed, Partial, Pending, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Failed while unzipping collection: {}", self.error)
    }
}

#[doc(hidden)]
impl<E: Error + 'static, Failed, Partial, Pending, I> Error for UnzipErrorData<E, Failed, Partial, Pending, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}
