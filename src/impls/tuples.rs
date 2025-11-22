use crate::{OneOf2, TryExtend, TryFromIterator};

/// Converts an iterator of `(A, B)` into a `(TryFromA, TryFromB)`, upholding the
/// [`TryFromIterator`] contract of both types.
impl<A, B, TryFromA, TryFromB> TryFromIterator<(A, B)> for (TryFromA, TryFromB)
where
    TryFromA: TryFromIterator<A>,
    TryFromB: TryFromIterator<B>,
{
    type Error = OneOf2<TryFromA::Error, TryFromB::Error>;

    /// Converts an iterator of `(A, B)` into a `(TryFromA, TryFromB)`.
    ///
    /// This implementation is suboptimal. If possible, prefer [`TryExtend::try_extend`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use collect_failable::TryFromIterator;
    ///
    /// let data = vec![(1, 2), (2, 3), (3, 4)];
    /// let (a, b): (HashSet<i32>, HashSet<i32>) = TryFromIterator::try_from_iter(data).unwrap();
    ///
    /// assert_eq!(a, HashSet::from([1, 2, 3]));
    /// assert_eq!(b, HashSet::from([2, 3, 4]));
    /// ```
    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = (A, B)>,
    {
        let items: (Vec<A>, Vec<B>) = iter.into_iter().unzip();
        Ok((
            TryFromA::try_from_iter(items.0).map_err(OneOf2::A)?, //
            TryFromB::try_from_iter(items.1).map_err(OneOf2::B)?,
        ))
    }
}

/// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
impl<A, B, TryFromA, TryFromB> TryExtend<(A, B)> for (TryFromA, TryFromB)
where
    TryFromA: TryExtend<A>,
    TryFromB: TryExtend<B>,
{
    type Error = OneOf2<TryFromA::Error, TryFromB::Error>;

    /// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
    ///
    /// This method should uphold any strong error guarantees of the underlying collections.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashSet;
    /// use collect_failable::TryExtend;
    ///
    /// let mut data = (HashSet::new(), HashSet::new());
    /// data.try_extend_safe([(1, 2), (2, 3)]).unwrap();
    ///
    /// assert_eq!(data.0, HashSet::from([1, 2]));
    /// assert_eq!(data.1, HashSet::from([2, 3]));
    /// ```
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (A, B)>,
    {
        let items: (Vec<A>, Vec<B>) = iter.into_iter().unzip();
        self.0.try_extend_safe(items.0).map_err(OneOf2::A)?;
        self.1.try_extend_safe(items.1).map_err(OneOf2::B)
    }

    /// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
    ///
    /// This method does not provide a strong error guarantee. But should uphold the basic error
    /// guarantee of the underlying collections.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashSet;
    /// use collect_failable::TryExtend;
    ///
    /// let mut data = (HashSet::new(), HashSet::new());
    /// data.try_extend([(1, 2), (2, 3)]).unwrap();
    ///
    /// assert_eq!(data.0, HashSet::from([1, 2]));
    /// assert_eq!(data.1, HashSet::from([2, 3]));
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (A, B)>,
    {
        for (a, b) in iter {
            self.0.try_extend(std::iter::once(a)).map_err(OneOf2::A)?;
            self.1.try_extend(std::iter::once(b)).map_err(OneOf2::B)?;
        }

        Ok(())
    }
}

// todo! implementations for more tuple types
