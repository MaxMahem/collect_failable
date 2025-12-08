use crate::{OneOf2, TryExtend, TryFromIterator};

/// Converts an iterator of `(A, B)` into a `(TryFromA, TryFromB)`, upholding the
/// [`TryFromIterator`] contract of both types.
impl<A, B, TryFromA, TryFromB, I> TryFromIterator<(A, B), I> for (TryFromA, TryFromB)
where
    I: IntoIterator<Item = (A, B)>,
    TryFromA: TryFromIterator<A, Vec<A>>,
    TryFromB: TryFromIterator<B, Vec<B>>,
{
    type Error = OneOf2<TryFromA::Error, TryFromB::Error>;

    /// Converts an iterator of `(A, B)` into a `(TryFromA, TryFromB)`.
    ///
    /// This implementation is suboptimal. If possible, prefer [`TryExtend::try_extend`] instead.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = include_doc::function_body!("tests/doc/tuples.rs", try_from_iter_tuple_example, [])]
    /// ```
    fn try_from_iter(iter: I) -> Result<Self, Self::Error> {
        let items: (Vec<A>, Vec<B>) = iter.into_iter().unzip();
        Ok((
            TryFromA::try_from_iter(items.0).map_err(OneOf2::A)?, //
            TryFromB::try_from_iter(items.1).map_err(OneOf2::B)?,
        ))
    }
}

/// Extends an `(TryFromA, TryFromB)` collection with the contents of an iterator of `(A, B)`.
///
/// Note: Tuples do not implement [`TryExtendSafe`](crate::TryExtendSafe) because they cannot
/// provide a strong error guarantee. If the second collection fails to extend, the first
/// may have already been modified.
impl<A, B, TryFromA, TryFromB, I> TryExtend<(A, B), I> for (TryFromA, TryFromB)
where
    I: IntoIterator<Item = (A, B)>,
    TryFromA: TryExtend<A, std::iter::Once<A>>,
    TryFromB: TryExtend<B, std::iter::Once<B>>,
{
    type Error = OneOf2<TryFromA::Error, TryFromB::Error>;

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
        for (a, b) in iter {
            self.0.try_extend(std::iter::once(a)).map_err(OneOf2::A)?;
            self.1.try_extend(std::iter::once(b)).map_err(OneOf2::B)?;
        }

        Ok(())
    }
}

// todo! implementations for more tuple types
