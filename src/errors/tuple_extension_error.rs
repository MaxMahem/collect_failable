use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;
use core::ops::Deref;

use alloc::boxed::Box;

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::Short;
use tap::Pipe;

#[cfg(doc)]
use crate::TryExtend;

use crate::TryExtendOne;
use crate::errors::Either;

/// An error that occurs when extending a tuple of collections fails.
///
/// This error preserves the error from whichever side failed, any unevaluated item,
/// and the remaining iterator for error recovery.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`]
/// implementation into a hidden `TupleExtensionErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`TupleExtensionError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `CollA`: The type of the first collection implementing [`TryExtendOne`].
/// - `CollB`: The type of the second collection implementing [`TryExtendOne`].
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct TupleExtensionError<CollA: TryExtendOne, CollB: TryExtendOne, I> {
    #[cfg(doc)]
    /// Which side failed: `Left(side_a)` when first collection fails,
    /// `Right(side_b)` when second collection fails
    pub side: Either<TupleExtensionErrorSide<CollA::Error, CollB::Item>, TupleExtensionErrorSide<CollB::Error, CollA::Item>>,
    #[cfg(doc)]
    /// The remaining iterator after the error occurred
    pub remaining: I,

    #[cfg(not(doc))]
    data: [Box<TupleExtensionErrorData<CollA, CollB, I>>; {
        /// The internal data of a [`TupleExtensionError`].
        #[doc(hidden)]
        #[derive(Debug)]
        pub struct TupleExtensionErrorData<CollA, CollB, I>
        where
            CollA: TryExtendOne,
            CollB: TryExtendOne,
        {
            /// Which side failed: `Left(side_a)` when first collection fails,
            /// `Right(side_b)` when second collection fails
            #[allow(clippy::type_complexity)]
            pub side:
                [Either<TupleExtensionErrorSide<CollA::Error, CollB::Item>, TupleExtensionErrorSide<CollB::Error, CollA::Item>>;
                    {
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
    #[cfg(not(doc))]
    _phantom: PhantomData<(CollA, CollB)>,
}

impl<CollA: TryExtendOne, CollB: TryExtendOne, I> TupleExtensionError<CollA, CollB, I> {
    /// Creates a new [`TupleExtensionError`] with the A side (first collection) having failed.
    pub fn new_a(error: CollA::Error, unevaluated: Option<CollB::Item>, remaining: I) -> Self {
        TupleExtensionErrorData { side: Either::Left(TupleExtensionErrorSide { error, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data, _phantom: PhantomData })
    }

    /// Creates a new [`TupleExtensionError`] with the B side (second collection) having failed.
    pub fn new_b(error: CollB::Error, unevaluated: Option<CollA::Item>, remaining: I) -> Self {
        TupleExtensionErrorData { side: Either::Right(TupleExtensionErrorSide { error, unevaluated }), remaining }
            .pipe(Box::new)
            .pipe(|data| Self { data, _phantom: PhantomData })
    }

    /// Consumes the error, returning the data containing the [`TupleExtensionError::side`]
    /// and the remaining [`TupleExtensionError::remaining`] iterator.
    #[must_use]
    pub fn into_data(self) -> TupleExtensionErrorData<CollA, CollB, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<CollA: TryExtendOne, CollB: TryExtendOne, I> Deref for TupleExtensionError<CollA, CollB, I> {
    type Target = TupleExtensionErrorData<CollA, CollB, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<Err: core::fmt::Debug, Unevaluated> core::fmt::Debug for TupleExtensionErrorSide<Err, Unevaluated> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Side").field("error", &self.error).field_type::<Unevaluated, Short>("unevaluated").finish()
    }
}

impl<CollA: TryExtendOne, CollB: TryExtendOne, I> Debug for TupleExtensionError<CollA, CollB, I>
where
    CollA::Error: Debug,
    CollB::Error: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TupleExtensionError").field("side", &self.data.side).field_type::<I, Short>("remaining").finish()
    }
}

impl<CollA: TryExtendOne, CollB: TryExtendOne, I> Display for TupleExtensionError<CollA, CollB, I>
where
    CollA::Error: Display,
    CollB::Error: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.data.side {
            Either::Left(side) => write!(f, "Failed while extending first collection: {}", side.error),
            Either::Right(side) => write!(f, "Failed while extending second collection: {}", side.error),
        }
    }
}

impl<CollA: TryExtendOne, CollB: TryExtendOne, I> Error for TupleExtensionError<CollA, CollB, I>
where
    CollA::Error: Error + 'static,
    CollB::Error: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.data.side {
            Either::Left(side) => Some(&side.error),
            Either::Right(side) => Some(&side.error),
        }
    }
}
