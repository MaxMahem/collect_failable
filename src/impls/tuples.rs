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
    type Error = TupleExtensionError<TryFromA, TryFromB, I::IntoIter>;

    /// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
    ///
    /// This method provides a basic error guarantee. If the method returns an error, one or both
    /// collections may have been partially modified.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::TryExtend;
    /// use std::collections::HashMap;
    ///
    /// let pairs = [((1, 1), (2, 2)), ((3, 3), (4, 4))];
    /// let mut tuple_of_maps = (HashMap::new(), HashMap::new());
    /// tuple_of_maps.try_extend(pairs).expect("should extend both collections");
    ///
    /// assert_eq!(tuple_of_maps.0, HashMap::from([(1, 1), (3, 3)]), "should contain all items");
    /// assert_eq!(tuple_of_maps.1, HashMap::from([(2, 2), (4, 4)]), "should contain all items");
    ///
    /// let colliding_pairs = [((5, 5), (6, 6)), ((1, 10), (7, 7))];
    /// let extend_err = tuple_of_maps.try_extend(colliding_pairs).expect_err("should collide on second item");
    /// let err_side = extend_err.side.as_ref().left().expect("should collide on left side");
    ///
    /// assert_eq!(err_side.error.item, (1, 10), "should contain colliding item");
    /// assert_eq!(tuple_of_maps.0[&1], 1, "value should not be modified");
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
