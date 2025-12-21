use no_drop::dbg::IntoNoDrop;

use crate::errors::TupleExtensionError;
use crate::{TryExtend, TryExtendOne};

#[cfg(doc)]
use crate::TryUnzip;

/// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
///
/// Note: Tuples do not implement [`TryExtendSafe`](crate::TryExtendSafe) because they cannot
/// provide a strong error guarantee. Extension has to proceed element by element and if the
/// second collection fails to extend, the first may have already been modified.
impl<TryFromA, TryFromB, I> TryExtend<I> for (TryFromA, TryFromB)
where
    I: IntoIterator<Item = (TryFromA::Item, TryFromB::Item)>,
    TryFromA: TryExtendOne + Default,
    TryFromB: TryExtendOne + Default,
{
    type Error = TupleExtensionError<TryFromA::Error, TryFromB::Error, TryFromA::Item, TryFromB::Item, I::IntoIter>;

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
            if let Err(error) = self.0.try_extend_one(a.unwrap()) {
                return Err(TupleExtensionError::new_a(error, Some(b.unwrap()), iter));
            }
            if let Err(error) = self.1.try_extend_one(b.unwrap()) {
                return Err(TupleExtensionError::new_b(error, None, iter));
            }
        }

        Ok(())
    }
}

// todo! implementations for more tuple types
