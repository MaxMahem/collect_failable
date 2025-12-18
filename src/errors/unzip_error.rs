use std::error::Error;
use std::fmt::{Debug, Display};

use display_as_debug::as_debug::DisplayDebug;
use display_as_debug::option::OpaqueOptionDbg;
use tap::Pipe;

use crate::TryExtendOne;

#[cfg(doc)]
use crate::TryUnzip;

/// An error that occurs when unzipping an iterator into two collections fails.
///
/// This error preserves the incomplete collection from the side that succeeded,
/// along with the error from the side that failed.
#[subdef::subdef]
#[derive(derive_more::TryUnwrap, derive_more::IsVariant, derive_more::Unwrap)]
pub enum UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
{
    /// Failed to extend the first collection (`FromA`).
    A(
        [UnzipErrorSide<FromA::Error, FromA, FromB, B, I>; {
            #[derive(derive_more::Deref)]
            #[deref(forward)]
            /// The incomplete collections from a failed [`TryUnzip::try_unzip`] operation.
            pub struct UnzipErrorSide<Err, Failed, Successful, T, I>(
                [Box<UnzipErrorSideData<Err, Failed, Successful, T, I>>; {
                    /// The internal data of a [`UnzipErrorSideData`].
                    pub struct UnzipErrorSideData<Err, Failed, Successful, T, I> {
                        /// The error caused during extension
                        pub error: Err,
                        /// The partial collection from the failed side
                        pub failed: Failed,
                        /// The incomplete collection from the successful side
                        pub successful: Successful,
                        /// The unevaluated item from the successful side
                        pub unevaluated: Option<T>,
                        /// The remaining iterator
                        pub remaining: I,
                    }
                }],
            );
        }],
    ),
    /// Failed to extend the second collection (`FromB`).
    B(UnzipErrorSide<FromB::Error, FromB, FromA, A, I>),
}

impl<Err, Failed, Successful, T, I> UnzipErrorSide<Err, Failed, Successful, T, I> {
    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_error(self) -> Err {
        self.0.error
    }

    /// Consumes the error, returning a [`UnzipErrorSideData`] containing the `error`,
    /// `failed` and `successful` collections, the optional `unevaluated` item, and the remaining `iterator`.
    #[must_use]
    pub fn into_data(self) -> UnzipErrorSideData<Err, Failed, Successful, T, I> {
        *self.0
    }
}

impl<Err, Failed, Successful, T, I> Debug for UnzipErrorSide<Err, Failed, Successful, T, I>
where
    Err: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZipErrorSide")
            .field("error", &self.error)
            .field("failed", &"..".as_debug())
            .field("successful", &"..".as_debug())
            .field("unevaluated", &OpaqueOptionDbg(&self.unevaluated))
            .field("remaining", &std::any::type_name::<I>())
            .finish()
    }
}

impl<A, B, FromA, FromB, I> UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
{
    /// Creates a new [`UnzipError::A`] variant.
    pub fn new_a(error: FromA::Error, failed: FromA, successful: FromB, unevaluated: Option<B>, remaining: I) -> Self {
        UnzipErrorSideData { error, failed, successful, unevaluated, remaining }.pipe(Box::new).pipe(UnzipErrorSide).pipe(Self::A)
    }

    /// Creates a new [`UnzipError::B`] variant.
    pub fn new_b(error: FromB::Error, failed: FromB, successful: FromA, unevaluated: Option<A>, remaining: I) -> Self {
        UnzipErrorSideData { error, failed, successful, unevaluated, remaining }.pipe(Box::new).pipe(UnzipErrorSide).pipe(Self::B)
    }

    /// Unwraps the [`UnzipError::A`] variant, or panics with `msg`.
    ///
    /// # Panics
    ///
    /// Panics if the error is [`UnzipError::B`].
    #[must_use]
    pub fn expect_a(self, msg: &str) -> UnzipErrorSide<FromA::Error, FromA, FromB, B, I>
    where
        FromA::Error: Debug,
        FromB::Error: Debug,
    {
        self.try_unwrap_a().expect(msg)
    }

    /// Unwraps the [`UnzipError::B`] variant, or panics with `msg`.
    ///
    /// # Panics
    ///
    /// Panics if the error is [`UnzipError::A`].
    #[must_use]
    pub fn expect_b(self, msg: &str) -> UnzipErrorSide<FromB::Error, FromB, FromA, A, I>
    where
        FromA::Error: Debug,
        FromB::Error: Debug,
    {
        self.try_unwrap_b().expect(msg)
    }
}

impl<A, B, FromA, FromB, I> Debug for UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
    FromA::Error: Debug,
    FromB::Error: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(side) => f.debug_tuple("UnzipError::A").field(side).finish(),
            Self::B(side) => f.debug_tuple("UnzipError::B").field(side).finish(),
        }
    }
}

impl<A, B, FromA, FromB, I> Display for UnzipError<A, B, FromA, FromB, I>
where
    FromA: TryExtendOne<A>,
    FromB: TryExtendOne<B>,
    FromA::Error: Display,
    FromB::Error: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(side) => write!(f, "Failed while extending first collection: {}", side.error),
            Self::B(side) => write!(f, "Failed while extending second collection: {}", side.error),
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
        match self {
            Self::A(side) => Some(&side.error),
            Self::B(side) => Some(&side.error),
        }
    }
}
