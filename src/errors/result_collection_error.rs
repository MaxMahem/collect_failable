use std::error::Error;
use std::fmt::{Debug, Display};

use itertools::Either;
use tap::Pipe;

use crate::utils::ResultTypeDebug;
use crate::TryFromIterator;

/// An error that occurs when collecting an iterator of [`Result`]s fails.
///
/// This error preserves both the iterator error (the first [`Err`] encountered)
/// and the partial collection result (which may be [`Ok`] with a partial collection
/// or [`Err`] with a collection error).
#[subdef::subdef]
#[derive(derive_more::Deref, PartialEq, Eq)]
#[deref(forward)]
pub struct ResultIterError<E, C, CErr>(
    [Box<ResultIterErrorData<E, C, CErr>>; {
        /// The internal data of a [`ResultCollectionError`].
        #[derive(PartialEq, Eq)]
        pub struct ResultIterErrorData<E, C, CErr> {
            /// The error from the Result iterator (first Err encountered)
            pub iteration_error: E,
            /// The partial collection result (Ok with partial data, or Err with collection error)
            pub collection_result: Result<C, CErr>,
        }
    }],
);

impl<E, C, CErr> ResultIterError<E, C, CErr> {
    /// Creates a new [`ResultCollectionError`] from an iterator error and collection result.
    pub fn new(iteration_error: E, collection_result: Result<C, CErr>) -> Self {
        ResultIterErrorData { iteration_error, collection_result }.pipe(Box::new).pipe(ResultIterError)
    }

    /// Consumes the error, returning the iterator error.
    #[must_use]
    pub fn into_iteration_error(self) -> E {
        self.0.iteration_error
    }

    /// Consumes the error, returning the collection result.
    #[must_use]
    pub fn into_collection_result(self) -> Result<C, CErr> {
        self.0.collection_result
    }

    /// Consumes the error, returning a [`ResultCollectionErrorData`] containing the
    /// `iterator_error` and `collection_result`.
    #[must_use]
    pub fn into_parts(self) -> ResultIterErrorData<E, C, CErr> {
        *self.0
    }
}

impl<T, E, C, CErr> IntoIterator for ResultIterError<E, C, CErr>
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

impl<E: Debug, C, CErr: Debug> Debug for ResultIterError<E, C, CErr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultIterError")
            .field("iteration_error", &self.iteration_error)
            .field("collection_result", &ResultTypeDebug(&self.collection_result))
            .finish()
    }
}

impl<E: Display, C, CErr: Display> Display for ResultIterError<E, C, CErr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iterator error: {}", self.iteration_error)?;
        if let Err(e) = &self.collection_result {
            write!(f, "; Collection error: {e}")?;
        }
        Ok(())
    }
}

impl<E: Error + 'static, C, CErr: Error> Error for ResultIterError<E, C, CErr> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.iteration_error)
    }
}
