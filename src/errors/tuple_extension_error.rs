use tap::Pipe;

use crate::utils::OptionTypeDebug;

#[cfg(doc)]
use crate::TryExtend;

/// An error that occurs when extending a tuple of collections fails.
///
/// This error preserves the unevaluated item and remaining iterator for error recovery.
#[subdef::subdef]
#[derive(derive_more::TryUnwrap, derive_more::IsVariant, derive_more::Unwrap)]
pub enum TupleExtensionError<ErrA, ErrB, A, B, I> {
    /// Failed to extend the `FromA` collection.
    A(
        [TupleExtensionErrorSide<ErrA, B, I>; {
            /// Container for the data of a failed extension.
            #[derive(derive_more::Deref)]
            #[deref(forward)]
            pub struct TupleExtensionErrorSide<Err, T, I>(
                [Box<TupleExtensionErrorSideData<Err, T, I>>; {
                    /// The internal data of a [`TupleExtensionErrorSide`].
                    #[derive(Debug)]
                    pub struct TupleExtensionErrorSideData<Err, T, I> {
                        /// The error caused during extension
                        pub error: Err,
                        /// The unevaluated item from the other side
                        pub unevaluated: Option<T>,
                        /// The remaining iterator
                        pub remaining: I,
                    }
                }],
            );
        }],
    ),
    /// Failed to extend the `FromB` collection.
    B(TupleExtensionErrorSide<ErrB, A, I>),
}

impl<ErrA, ErrB, A, B, I> TupleExtensionError<ErrA, ErrB, A, B, I> {
    /// Creates a new [`TupleExtensionError::A`] variant.
    pub fn new_a(error: ErrA, unevaluated: Option<B>, remaining: I) -> Self {
        TupleExtensionErrorSideData { error, unevaluated, remaining }
            .pipe(Box::new)
            .pipe(TupleExtensionErrorSide)
            .pipe(Self::A)
    }

    /// Creates a new [`TupleExtensionError::B`] variant.
    pub fn new_b(error: ErrB, unevaluated: Option<A>, remaining: I) -> Self {
        TupleExtensionErrorSideData { error, unevaluated, remaining }
            .pipe(Box::new)
            .pipe(TupleExtensionErrorSide)
            .pipe(Self::B)
    }

    /// Unwraps the [`TupleExtensionError::A`] variant, or panics with `msg`.
    ///
    /// # Panics
    ///
    /// Panics if the error is [`TupleExtensionError::B`].
    #[must_use]
    pub fn expect_a(self, msg: &str) -> TupleExtensionErrorSide<ErrA, B, I>
    where
        ErrA: std::fmt::Debug,
        ErrB: std::fmt::Debug,
    {
        self.try_unwrap_a().expect(msg)
    }

    /// Unwraps the [`TupleExtensionError::B`] variant, or panics with `msg`.
    ///
    /// # Panics
    ///
    /// Panics if the error is [`TupleExtensionError::A`].
    #[must_use]
    pub fn expect_b(self, msg: &str) -> TupleExtensionErrorSide<ErrB, A, I>
    where
        ErrA: std::fmt::Debug,
        ErrB: std::fmt::Debug,
    {
        self.try_unwrap_b().expect(msg)
    }
}

impl<Err, T, I> TupleExtensionErrorSide<Err, T, I> {
    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_error(self) -> Err {
        self.0.error
    }

    /// Consumes the error, returning a [`TupleExtensionErrorSideData`] containing the `error`,
    /// the optional `unevaluated` item, and the remaining `iterator`.
    #[must_use]
    pub fn into_parts(self) -> TupleExtensionErrorSideData<Err, T, I> {
        *self.0
    }
}

impl<Err, T, I> std::fmt::Debug for TupleExtensionErrorSide<Err, T, I>
where
    Err: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleExtensionErrorSide")
            .field("error", &self.error)
            .field("unevaluated", &OptionTypeDebug(&self.unevaluated))
            .field("remaining", &std::any::type_name::<I>())
            .finish()
    }
}

impl<ErrA, ErrB, A, B, I> std::fmt::Debug for TupleExtensionError<ErrA, ErrB, A, B, I>
where
    ErrA: std::fmt::Debug,
    ErrB: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(side) => f.debug_tuple("TupleExtensionError::A").field(side).finish(),
            Self::B(side) => f.debug_tuple("TupleExtensionError::B").field(side).finish(),
        }
    }
}

impl<ErrA, ErrB, A, B, I> std::fmt::Display for TupleExtensionError<ErrA, ErrB, A, B, I>
where
    ErrA: std::fmt::Display,
    ErrB: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(side) => write!(f, "Failed while extending first collection: {}", side.error),
            Self::B(side) => write!(f, "Failed while extending second collection: {}", side.error),
        }
    }
}

impl<ErrA, ErrB, A, B, I> std::error::Error for TupleExtensionError<ErrA, ErrB, A, B, I>
where
    ErrA: std::error::Error + 'static,
    ErrB: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::A(side) => Some(&side.error),
            Self::B(side) => Some(&side.error),
        }
    }
}
