use std::iter::Once;

use no_drop::dbg::IntoNoDrop;

use crate::{TryExtend, TryFromIterator, TupleCollectionError, TupleExtensionError};

#[cfg(doc)]
use crate::TryUnzip;

/// Converts an iterator of `(A, B)` into a `(TryFromA, TryFromB)`, upholding the
/// [`TryFromIterator`] contract of both types.
impl<A, B, TryFromA, TryFromB, I> TryFromIterator<I> for (TryFromA, TryFromB)
where
    I: IntoIterator<Item = (A, B)>,
    TryFromA: TryFromIterator<Vec<A>>,
    TryFromB: TryFromIterator<Vec<B>>,
{
    type Error = TupleCollectionError<TryFromA::Error, TryFromB::Error, TryFromA, Vec<B>>;

    /// Converts an iterator of `(A, B)` into a `(TryFromA, TryFromB)`.
    ///
    /// This implementation is suboptimal. If possible, prefer [`TryUnzip::unzip`] or
    /// [`TryExtend::try_extend`] instead.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = include_doc::function_body!("tests/doc/tuples.rs", try_from_iter_tuple_example, [])]
    /// ```
    fn try_from_iter(iter: I) -> Result<Self, Self::Error> {
        let (vec_a, vec_b): (Vec<A>, Vec<B>) = iter.into_iter().unzip();
        let (vec_a, vec_b) = (vec_a.no_drop(), vec_b.no_drop());

        let collection_a = match TryFromA::try_from_iter(vec_a.unwrap()) {
            Ok(coll) => coll.no_drop(),
            Err(error) => return Err(TupleCollectionError::new_a(error, vec_b.unwrap())),
        };

        let collection_b = match TryFromB::try_from_iter(vec_b.unwrap()) {
            Ok(coll) => coll.no_drop(),
            Err(error) => return Err(TupleCollectionError::new_b(error, collection_a.unwrap())),
        };

        Ok((collection_a.unwrap(), collection_b.unwrap()))
    }
}

/// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
///
/// Note: Tuples do not implement [`TryExtendSafe`](crate::TryExtendSafe) because they cannot
/// provide a strong error guarantee. Extension has to proceed element by element and if the
/// second collection fails to extend, the first may have already been modified.
impl<A, B, TryFromA, TryFromB, I> TryExtend<I> for (TryFromA, TryFromB)
where
    I: IntoIterator<Item = (A, B)>,
    TryFromA: TryExtend<Once<A>> + Default,
    TryFromB: TryExtend<Once<B>> + Default,
{
    type Error = TupleExtensionError<TryFromA::Error, TryFromB::Error, A, B, I::IntoIter>;

    /// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
    ///
    /// This method provides a basic error guarantee. If the method returns an error, one or both
    /// collections may have been partially modified.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/tuples.rs", try_extend_tuple_example, [])]
    /// ```
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        for (a, b) in iter.by_ref().map(|(a, b)| (a.no_drop(), b.no_drop())) {
            if let Err(error) = self.0.try_extend(std::iter::once(a.unwrap())) {
                return Err(TupleExtensionError::new_a(error, Some(b.unwrap()), iter));
            }
            if let Err(error) = self.1.try_extend(std::iter::once(b.unwrap())) {
                return Err(TupleExtensionError::new_b(error, None, iter));
            }
        }

        Ok(())
    }
}

// todo! implementations for more tuple types
