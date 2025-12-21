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
            /// The incomplete collections from a failed [`TryUnzip::try_unzip`] operation.
            ///
            /// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref)
            /// implementation into a hidden `UnzipErrorSideData` type, with identical fields. If necessary,
            /// you can consume an instance of this type via [`UnzipErrorSide::into_data`] to get owned data.
            pub struct UnzipErrorSide<Err, Failed, Successful, T, I> {
                #[cfg(doc)]
                /// The error caused during extension
                pub error: Err,
                #[cfg(doc)]
                /// The partial collection from the failed side
                pub failed: Failed,
                #[cfg(doc)]
                /// The incomplete collection from the successful side
                pub successful: Successful,
                #[cfg(doc)]
                /// The unevaluated item from the successful side
                pub unevaluated: Option<T>,
                #[cfg(doc)]
                /// The remaining iterator
                pub remaining: I,

                #[cfg(not(doc))]
                data: [Box<UnzipErrorSideData<Err, Failed, Successful, T, I>>; {
                    /// The internal data of a [`UnzipErrorSideData`].
                    #[doc(hidden)]
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
            }
        }],
    ),
    /// Failed to extend the second collection (`FromB`).
    B(UnzipErrorSide<FromB::Error, FromB, FromA, A, I>),
}

impl<Err, Failed, Successful, T, I> UnzipErrorSide<Err, Failed, Successful, T, I> {
    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_error(self) -> Err {
        self.data.error
    }

    /// Consumes the error, returning a `UnzipErrorSideData` containing the [`UnzipErrorSide::error`],
    /// [`UnzipErrorSide::failed`] and [`UnzipErrorSide::successful`] collections, the optional
    /// [`UnzipErrorSide::unevaluated`] item, and the remaining [`UnzipErrorSide::remaining`] iterator.
    #[must_use]
    pub fn into_data(self) -> UnzipErrorSideData<Err, Failed, Successful, T, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<Err, Failed, Successful, T, I> std::ops::Deref for UnzipErrorSide<Err, Failed, Successful, T, I> {
    type Target = UnzipErrorSideData<Err, Failed, Successful, T, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Err, Failed, Successful, T, I> Debug for UnzipErrorSide<Err, Failed, Successful, T, I>
where
    Err: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZipErrorSide")
            .field("error", &self.data.error)
            .field("failed", &"..".as_debug())
            .field("successful", &"..".as_debug())
            .field("unevaluated", &OpaqueOptionDbg(&self.data.unevaluated))
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
        UnzipErrorSideData { error, failed, successful, unevaluated, remaining }
            .pipe(Box::new)
            .pipe(|data| UnzipErrorSide { data })
            .pipe(Self::A)
    }

    /// Creates a new [`UnzipError::B`] variant.
    pub fn new_b(error: FromB::Error, failed: FromB, successful: FromA, unevaluated: Option<A>, remaining: I) -> Self {
        UnzipErrorSideData { error, failed, successful, unevaluated, remaining }
            .pipe(Box::new)
            .pipe(|data| UnzipErrorSide { data })
            .pipe(Self::B)
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
            Self::A(side) => write!(f, "Failed while extending first collection: {}", side.data.error),
            Self::B(side) => write!(f, "Failed while extending second collection: {}", side.data.error),
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
            Self::A(side) => Some(&side.data.error),
            Self::B(side) => Some(&side.data.error),
        }
    }
}
