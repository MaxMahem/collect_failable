use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use display_as_debug::option::OpaqueOptionDbg;
use either::Either;
use tap::Pipe;

use crate::TryExtendOne;

#[cfg(doc)]
use crate::TryUnzip;

/// An error that occurs when unzipping an iterator into two collections fails.
///
/// This error preserves the incomplete collection from the side that succeeded,
/// along with the error from the side that failed, and the remaining iterator.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref)
/// implementation into a hidden `UnzipErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`UnzipError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `FromA`: The type of the first collection.
/// - `FromB`: The type of the second collection.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct UnzipError<FromA, FromB, I>
where
    FromA: TryExtendOne,
    FromB: TryExtendOne,
{
    #[cfg(doc)]
    /// Which side failed: `Left(side_a)` when first collection fails,
    /// `Right(side_b)` when second collection fails
    pub side: [Either<UnzipErrorSide<FromA, FromB>, UnzipErrorSide<FromB, FromA>>; {
        /// Information about which side of an unzip operation failed.
        pub struct UnzipErrorSide<Failed: TryExtendOne, Successful: TryExtendOne> {
            /// The error that occurred during extension.
            pub error: Failed::Error,
            /// The partial collection from the failed side.
            pub failed: Failed,
            /// The incomplete collection from the successful side.
            pub successful: Successful,
            /// The unevaluated item from the successful side, if any.
            pub unevaluated: Option<Successful::Item>,
        }
    }],
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: [Box<UnzipErrorData<FromA, FromB, I>>; {
        /// The internal data of an [`UnzipError`].
        #[doc(hidden)]
        pub struct UnzipErrorData<FromA, FromB, I>
        where
            FromA: TryExtendOne,
            FromB: TryExtendOne,
        {
            /// Which side failed: `Left(side_a)` when first collection fails,
            /// `Right(side_b)` when second collection fails
            pub side: Either<UnzipErrorSide<FromA, FromB>, UnzipErrorSide<FromB, FromA>>,
            /// The remaining iterator after the error occurred
            pub remaining: I,
        }
    }],
}

impl<FromA, FromB, I> UnzipError<FromA, FromB, I>
where
    FromA: TryExtendOne,
    FromB: TryExtendOne,
{
    /// Creates a new [`UnzipError`] with the A side (first collection) having failed.
    pub fn new_a(error: FromA::Error, failed: FromA, successful: FromB, unevaluated: Option<FromB::Item>, remaining: I) -> Self {
        UnzipErrorData { side: Either::Left(UnzipErrorSide { error, failed, successful, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Creates a new [`UnzipError`] with the B side (second collection) having failed.
    pub fn new_b(error: FromB::Error, failed: FromB, successful: FromA, unevaluated: Option<FromA::Item>, remaining: I) -> Self {
        UnzipErrorData { side: Either::Right(UnzipErrorSide { error, failed, successful, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Consumes the error, returning the data containing the [`UnzipError::side`]
    /// and the remaining [`UnzipError::remaining`] iterator.
    #[must_use]
    pub fn into_data(self) -> UnzipErrorData<FromA, FromB, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<FromA, FromB, I> Deref for UnzipError<FromA, FromB, I>
where
    FromA: TryExtendOne,
    FromB: TryExtendOne,
{
    type Target = UnzipErrorData<FromA, FromB, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Failed: TryExtendOne, Successful: TryExtendOne> Debug for UnzipErrorSide<Failed, Successful>
where
    Failed::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnzipSide")
            .field("error", &self.error)
            .field("failed", &"...")
            .field("successful", &"...")
            .field("unevaluated", &OpaqueOptionDbg(&self.unevaluated))
            .finish()
    }
}

impl<FromA, FromB, I> Debug for UnzipError<FromA, FromB, I>
where
    FromA: TryExtendOne,
    FromB: TryExtendOne,
    FromA::Error: Debug,
    FromB::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnzipError").field("side", &self.data.side).field("remaining", &std::any::type_name::<I>()).finish()
    }
}

impl<FromA, FromB, I> Display for UnzipError<FromA, FromB, I>
where
    FromA: TryExtendOne,
    FromB: TryExtendOne,
    FromA::Error: Display,
    FromB::Error: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.data.side {
            Either::Left(side) => write!(f, "Failed while unzipping into first collection: {}", side.error),
            Either::Right(side) => write!(f, "Failed while unzipping into second collection: {}", side.error),
        }
    }
}

impl<FromA, FromB, I> Error for UnzipError<FromA, FromB, I>
where
    FromA: TryExtendOne,
    FromB: TryExtendOne,
    FromA::Error: Error + 'static,
    FromB::Error: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.data.side {
            Either::Left(side) => Some(&side.error),
            Either::Right(side) => Some(&side.error),
        }
    }
}
