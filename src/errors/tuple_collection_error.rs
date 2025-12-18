use tap::Pipe;

#[cfg(doc)]
use crate::TryFromIterator;

/// An error that occurs when collecting a tuple from an iterator fails.
///
/// This error preserves either the unconverted Vec or the successfully created collection
/// depending on which side failed.
#[subdef::subdef]
#[derive(derive_more::TryUnwrap, derive_more::IsVariant, derive_more::Unwrap)]
pub enum TupleCollectionError<ErrA, ErrB, FromA, FromB> {
    /// Failed to collect the first collection. Preserves the unconverted `FromB`.
    A(
        [TupleCollectionErrorSide<ErrA, FromB>; {
            /// Container for the data of a failed collection.
            #[derive(derive_more::Deref)]
            #[deref(forward)]
            pub struct TupleCollectionErrorSide<Err, From>(
                [Box<TupleCollectionErrorSideData<Err, From>>; {
                    /// Data of a failed collection.
                    pub struct TupleCollectionErrorSideData<Err, From> {
                        /// The error from trying to create the collection
                        pub error: Err,
                        /// The preserved unprocessed data
                        pub from: From,
                    }
                }],
            );
        }],
    ),
    /// Failed to collect the second collection. Preserves the converted `FromA`.
    B(TupleCollectionErrorSide<ErrB, FromA>),
}

impl<ErrA, ErrB, FromA, FromB> TupleCollectionError<ErrA, ErrB, FromA, FromB> {
    /// Creates a new [`TupleCollectionError::A`] variant.
    pub fn new_a(error: ErrA, from: FromB) -> Self {
        TupleCollectionErrorSideData { error, from }.pipe(Box::new).pipe(TupleCollectionErrorSide).pipe(Self::A)
    }

    /// Creates a new [`TupleCollectionError::B`] variant.
    pub fn new_b(error: ErrB, from: FromA) -> Self {
        TupleCollectionErrorSideData { error, from }.pipe(Box::new).pipe(TupleCollectionErrorSide).pipe(Self::B)
    }

    /// Unwraps the [`TupleCollectionError::A`] variant, or panics with `msg`.
    ///
    /// # Panics
    ///
    /// Panics if the error is [`TupleCollectionError::B`].
    #[must_use]
    pub fn expect_a(self, msg: &str) -> TupleCollectionErrorSide<ErrA, FromB>
    where
        ErrA: std::fmt::Debug,
        ErrB: std::fmt::Debug,
    {
        self.try_unwrap_a().expect(msg)
    }

    /// Unwraps the [`TupleCollectionError::B`] variant, or panics with `msg`.
    ///
    /// # Panics
    ///
    /// Panics if the error is [`TupleCollectionError::A`].
    #[must_use]
    pub fn expect_b(self, msg: &str) -> TupleCollectionErrorSide<ErrB, FromA>
    where
        ErrA: std::fmt::Debug,
        ErrB: std::fmt::Debug,
    {
        self.try_unwrap_b().expect(msg)
    }
}

impl<Err, From> TupleCollectionErrorSide<Err, From> {
    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_error(self) -> Err {
        self.0.error
    }

    /// Consumes the error, returning the data.
    #[must_use]
    pub fn into_data(self) -> TupleCollectionErrorSideData<Err, From> {
        *self.0
    }
}

impl<Err, From> std::fmt::Debug for TupleCollectionErrorSide<Err, From>
where
    Err: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TupleCollectionErrorSide")
            .field("error", &self.error)
            .field("from", &std::any::type_name::<From>())
            .finish()
    }
}

impl<ErrA, ErrB, FromA, FromB> std::fmt::Debug for TupleCollectionError<ErrA, ErrB, FromA, FromB>
where
    ErrA: std::fmt::Debug,
    ErrB: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(side) => f.debug_tuple("TupleCollectionError::A").field(side).finish(),
            Self::B(side) => f.debug_tuple("TupleCollectionError::B").field(side).finish(),
        }
    }
}

impl<ErrA, ErrB, CollA, VecB> std::fmt::Display for TupleCollectionError<ErrA, ErrB, CollA, VecB>
where
    ErrA: std::fmt::Display,
    ErrB: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(side) => write!(f, "Failed while collecting first collection: {}", side.error),
            Self::B(side) => write!(f, "Failed while collecting second collection: {}", side.error),
        }
    }
}

impl<ErrA, ErrB, CollA, VecB> std::error::Error for TupleCollectionError<ErrA, ErrB, CollA, VecB>
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
