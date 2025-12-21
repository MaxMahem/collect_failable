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
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref)
/// implementation into a hidden `ResultCollectionErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`ResultCollectionError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `E`: The type of the iterator error.
/// - `C`: The type of the collection.
/// - `CErr`: The type of the collection error.
/// - `I`: The type of the remaining iterator.
#[subdef::subdef]
pub struct ResultCollectionError<E, C, CErr, I> {
    #[cfg(doc)]
    /// The first [`Err`] encountered from the [`Result`] [`Iterator`]
    pub iteration_error: E,
    #[cfg(doc)]
    /// The partial collection result ([`Ok`] with partial collection, or [`Err`] with collection error)
    pub collection_result: Result<C, CErr>,
    #[cfg(doc)]
    /// The remaining [`Iterator`] (items not yet consumed when the error occurred)
    pub result_iter: I,

    #[cfg(not(doc))]
    data: [Box<ResultCollectionErrorData<E, C, CErr, I>>; {
        /// The internal data of a [`ResultCollectionError`].
        #[doc(hidden)]
        pub struct ResultCollectionErrorData<E, C, CErr, I> {
            /// The first [`Err`] encountered from the [`Result`] [`Iterator`]
            pub iteration_error: E,
            /// The partial collection result ([`Ok`] with partial collection, or [`Err`] with collection error)
            pub collection_result: Result<C, CErr>,
            /// The remaining [`Iterator`] (items not yet consumed when the error occurred)
            pub result_iter: I,
        }
    }],
}

impl<E, C, CErr, I> ResultCollectionError<E, C, CErr, I> {
    /// Creates a new [`ResultCollectionError`] from an iterator error and collection result.
    pub fn new(iteration_error: E, collection_result: Result<C, CErr>, iter: I) -> Self {
        ResultCollectionErrorData { iteration_error, collection_result, result_iter: iter }
            .pipe(Box::new)
            .pipe(|data| Self { data })
    }

    /// Consumes the error, returning the iterator error.
    #[must_use]
    pub fn into_iteration_error(self) -> E {
        self.data.iteration_error
    }

    /// Consumes the error, returning the collection result.
    ///
    /// # Errors
    ///
    /// Returns the collection error if the collection result is an error.
    pub fn into_collection_result(self) -> Result<C, CErr> {
        self.data.collection_result
    }

    /// Consumes the error, returning the remaining iterator.
    #[must_use]
    pub fn into_result_iter(self) -> I {
        self.data.result_iter
    }

    /// Consumes the error, returning a `ResultCollectionErrorData` containing the
    /// [`ResultCollectionError::iteration_error`], [`ResultCollectionError::collection_result`],
    /// and [`ResultCollectionError::result_iter`].
    #[must_use]
    pub fn into_data(self) -> ResultCollectionErrorData<E, C, CErr, I> {
        *self.data
    }
}

#[doc(hidden)]
impl<E, C, CErr, I> std::ops::Deref for ResultCollectionError<E, C, CErr, I> {
    type Target = ResultCollectionErrorData<E, C, CErr, I>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, E, C, CErr, I> IntoIterator for ResultCollectionError<E, C, CErr, I>
where
    C: IntoIterator<Item = T>,
    CErr: IntoIterator<Item = T>,
{
    type Item = T;
    type IntoIter = Either<C::IntoIter, CErr::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        match self.data.collection_result {
            Ok(c) => Either::Left(c.into_iter()),
            Err(e) => Either::Right(e.into_iter()),
        }
    }
}

impl<E: Debug, C, CErr: Debug, I> Debug for ResultCollectionError<E, C, CErr, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultIterError")
            .field("iteration_error", &self.data.iteration_error)
            .field("collection_result", &OpaqueResultDbg(&self.data.collection_result))
            .field("result_iter", &std::any::type_name::<I>())
            .finish()
    }
}

impl<E: Display, C, CErr: Display, I> Display for ResultCollectionError<E, C, CErr, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iterator error: {}", self.data.iteration_error).and_then(|()| match &self.data.collection_result {
            Err(e) => write!(f, "; Collection error: {e}"),
            _ => Ok(()),
        })
    }
}

impl<E: Error + 'static, C, CErr: Error, I> Error for ResultCollectionError<E, C, CErr, I> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.data.iteration_error)
    }
}
