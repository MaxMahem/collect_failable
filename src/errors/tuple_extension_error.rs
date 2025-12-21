use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use display_as_debug::option::OpaqueOptionDbg;
use either::Either;
use tap::Pipe;

#[cfg(doc)]
use crate::TryExtend;

/// An error that occurs when extending a tuple of collections fails.
///
/// This error preserves the error from whichever side failed, any unevaluated item,
/// and the remaining iterator for error recovery.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref)
/// implementation into a hidden `TupleExtensionErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`TupleExtensionError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `ErrA`: The error type of the first collection.
/// - `ErrB`: The error type of the second collection.
/// - `A`: The type of the first collection.
/// - `B`: The type of the second collection.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct TupleExtensionError<ErrA, ErrB, A, B, I> {
    #[cfg(doc)]
    /// Which side failed: `Left(side_a)` when first collection fails,
    /// `Right(side_b)` when second collection fails
    pub side: Either<TupleExtensionErrorSide<ErrA, B>, TupleExtensionErrorSide<ErrB, A>>,
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: [Box<TupleExtensionErrorData<ErrA, ErrB, A, B, I>>; {
        /// The internal data of a [`TupleExtensionError`].
        #[doc(hidden)]
        #[derive(Debug)]
        pub struct TupleExtensionErrorData<ErrA, ErrB, A, B, I> {
            /// Which side failed: `Left(side_a)` when first collection fails,
            /// `Right(side_b)` when second collection fails
            pub side: [Either<TupleExtensionErrorSide<ErrA, B>, TupleExtensionErrorSide<ErrB, A>>; {
                /// Information about which side of a tuple extension failed.
                pub struct TupleExtensionErrorSide<Err, Unevaluated> {
                    /// The error that occurred during extension.
                    pub error: Err,
                    /// The unevaluated item from the other side, if any.
                    pub unevaluated: Option<Unevaluated>,
                }
            }],
            /// The remaining iterator after the error occurred
            pub remaining: I,
        }
    }],
}

impl<ErrA, ErrB, A, B, I> TupleExtensionError<ErrA, ErrB, A, B, I> {
    /// Creates a new [`TupleExtensionError`] with the A side (first collection) having failed.
    pub fn new_a(error: ErrA, unevaluated: Option<B>, remaining: I) -> Self {
        TupleExtensionErrorData { side: Either::Left(TupleExtensionErrorSide { error, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Creates a new [`TupleExtensionError`] with the B side (second collection) having failed.
    pub fn new_b(error: ErrB, unevaluated: Option<A>, remaining: I) -> Self {
        TupleExtensionErrorData { side: Either::Right(TupleExtensionErrorSide { error, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Consumes the error, returning the data containing the [`TupleExtensionError::side`]
    /// and the remaining [`TupleExtensionError::remaining`] iterator.
    #[must_use]
    pub fn into_data(self) -> TupleExtensionErrorData<ErrA, ErrB, A, B, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<ErrA, ErrB, A, B, I> Deref for TupleExtensionError<ErrA, ErrB, A, B, I> {
    type Target = TupleExtensionErrorData<ErrA, ErrB, A, B, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<Err: std::fmt::Debug, Unevaluated> std::fmt::Debug for TupleExtensionErrorSide<Err, Unevaluated> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Side").field("error", &self.error).field("unevaluated", &OpaqueOptionDbg(&self.unevaluated)).finish()
    }
}

impl<ErrA, ErrB, A, B, I> Debug for TupleExtensionError<ErrA, ErrB, A, B, I>
where
    ErrA: Debug,
    ErrB: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleExtensionError")
            .field("side", &self.data.side)
            .field("remaining", &std::any::type_name::<I>())
            .finish()
    }
}

impl<ErrA, ErrB, A, B, I> Display for TupleExtensionError<ErrA, ErrB, A, B, I>
where
    ErrA: Display,
    ErrB: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.data.side {
            Either::Left(side) => write!(f, "Failed while extending first collection: {}", side.error),
            Either::Right(side) => write!(f, "Failed while extending second collection: {}", side.error),
        }
    }
}

impl<ErrA, ErrB, A, B, I> Error for TupleExtensionError<ErrA, ErrB, A, B, I>
where
    ErrA: Error + 'static,
    ErrB: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.data.side {
            Either::Left(side) => Some(&side.error),
            Either::Right(side) => Some(&side.error),
        }
    }
}
