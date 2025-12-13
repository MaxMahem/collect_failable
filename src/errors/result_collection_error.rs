use std::error::Error;
use std::fmt::{Debug, Display};

use display_as_debug::result::OpaqueResultDbg;
use itertools::Either;
use tap::Pipe;

/// An error that occurs when collecting an iterator of [`Result`]s fails.
///
/// This error preserves both the iterator error (the first [`Err`] encountered),
/// the partial collection result (which may be [`Ok`] with a partial collection
/// or [`Err`] with a collection error), and the remaining iterator.
#[subdef::subdef]
#[derive(derive_more::Deref)]
#[deref(forward)]
pub struct ResultIterError<E, C, CErr, I>(
    [Box<ResultIterErrorData<E, C, CErr, I>>; {
        /// The internal data of a [`ResultCollectionError`].
        pub struct ResultIterErrorData<E, C, CErr, I> {
            /// The error from the Result iterator (first Err encountered)
            pub iteration_error: E,
            /// The partial collection result (Ok with partial data, or Err with collection error)
            pub collection_result: Result<C, CErr>,
            /// The remaining iterator (items not yet consumed when the error occurred)
            pub result_iter: I,
        }
    }],
);

impl<E, C, CErr, I> ResultIterError<E, C, CErr, I> {
    /// Creates a new [`ResultCollectionError`] from an iterator error and collection result.
    pub fn new(iteration_error: E, collection_result: Result<C, CErr>, iter: I) -> Self {
        ResultIterErrorData { iteration_error, collection_result, result_iter: iter }
            .pipe(Box::new)
            .pipe(ResultIterError)
    }

    /// Consumes the error, returning the iterator error.
    #[must_use]
    pub fn into_iteration_error(self) -> E {
        self.0.iteration_error
    }

    /// Consumes the error, returning the collection result.
    ///
    /// # Errors
    ///
    /// Returns the collection error if the collection result is an error.
    pub fn into_collection_result(self) -> Result<C, CErr> {
        self.0.collection_result
    }

    /// Consumes the error, returning the remaining iterator.
    #[must_use]
    pub fn into_result_iter(self) -> I {
        self.0.result_iter
    }

    /// Consumes the error, returning a [`ResultCollectionErrorData`] containing the
    /// `iterator_error`, `collection_result`, and `iter`.
    #[must_use]
    pub fn into_parts(self) -> ResultIterErrorData<E, C, CErr, I> {
        *self.0
    }
}

impl<T, E, C, CErr, I> IntoIterator for ResultIterError<E, C, CErr, I>
where
    C: IntoIterator<Item = T>,
    CErr: IntoIterator<Item = T>,
{
    type Item = T;
    type IntoIter = Either<C::IntoIter, CErr::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0.collection_result {
            Ok(c) => Either::Left(c.into_iter()),
            Err(e) => Either::Right(e.into_iter()),
        }
    }
}

impl<E: Debug, C, CErr: Debug, I> Debug for ResultIterError<E, C, CErr, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultIterError")
            .field("iteration_error", &self.iteration_error)
            .field("collection_result", &OpaqueResultDbg(&self.collection_result))
            .field("result_iter", &std::any::type_name::<I>())
            .finish()
    }
}

impl<E: Display, C, CErr: Display, I> Display for ResultIterError<E, C, CErr, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iterator error: {}", self.iteration_error)?;
        if let Err(e) = &self.collection_result {
            write!(f, "; Collection error: {e}")?;
        }
        Ok(())
    }
}

impl<E: Error + 'static, C, CErr: Error, I> Error for ResultIterError<E, C, CErr, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.iteration_error)
    }
}
