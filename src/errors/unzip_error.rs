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
#[subdef::subdef]
pub struct UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
{
    #[cfg(doc)]
    /// Which side failed: `Left(side_a)` when first collection fails,
    /// `Right(side_b)` when second collection fails
    pub side: [Either<UnzipSide<FromA::Error, FromA, FromB, B>, UnzipSide<FromB::Error, FromB, FromA, A>>; {
        /// Information about which side of an unzip operation failed.
        pub struct UnzipSide<Err, Failed, Successful, Unevaluated> {
            /// The error that occurred during extension.
            pub error: Err,
            /// The partial collection from the failed side.
            pub failed: Failed,
            /// The incomplete collection from the successful side.
            pub successful: Successful,
            /// The unevaluated item from the successful side, if any.
            pub unevaluated: Option<Unevaluated>,
        }
    }],
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: [Box<UnzipErrorData<A, B, FromA, FromB, I>>; {
        /// The internal data of an [`UnzipError`].
        #[doc(hidden)]
        pub struct UnzipErrorData<A, B, FromA, FromB, I>
        where
            FromA: TryExtendOne<A>,
            FromB: TryExtendOne<B>,
        {
            /// Which side failed: `Left(side_a)` when first collection fails,
            /// `Right(side_b)` when second collection fails
            pub side: Either<UnzipSide<FromA::Error, FromA, FromB, B>, UnzipSide<FromB::Error, FromB, FromA, A>>,
            /// The remaining iterator after the error occurred
            pub remaining: I,
        }
    }],
}

impl<A, B, FromA, FromB, I> UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
{
    /// Creates a new [`UnzipError`] with the A side (first collection) having failed.
    pub fn new_a(error: FromA::Error, failed: FromA, successful: FromB, unevaluated: Option<B>, remaining: I) -> Self {
        UnzipErrorData { side: Either::Left(UnzipSide { error, failed, successful, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Creates a new [`UnzipError`] with the B side (second collection) having failed.
    pub fn new_b(error: FromB::Error, failed: FromB, successful: FromA, unevaluated: Option<A>, remaining: I) -> Self {
        UnzipErrorData { side: Either::Right(UnzipSide { error, failed, successful, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Consumes the error, returning the data containing the [`UnzipError::side`]
    /// and the remaining [`UnzipError::remaining`] iterator.
    #[must_use]
    pub fn into_data(self) -> UnzipErrorData<A, B, FromA, FromB, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<A, B, FromA, FromB, I> Deref for UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
{
    type Target = UnzipErrorData<A, B, FromA, FromB, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Err: Debug, Failed, Successful, Unevaluated> Debug for UnzipSide<Err, Failed, Successful, Unevaluated> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnzipSide")
            .field("error", &self.error)
            .field("failed", &"...")
            .field("successful", &"...")
            .field("unevaluated", &OpaqueOptionDbg(&self.unevaluated))
            .finish()
    }
}

impl<A, B, FromA, FromB, I> Debug for UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
    FromA::Error: Debug,
    FromB::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnzipError").field("side", &self.data.side).field("remaining", &std::any::type_name::<I>()).finish()
    }
}

impl<A, B, FromA, FromB, I> Display for UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
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

impl<A, B, FromA, FromB, I> Error for UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
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
