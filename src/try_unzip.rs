use crate::{OneOf2, TryExtend};

/// Extends [Iterator] with a failable unzip method.
pub trait TryUnzip {
    /// Tries to unzip the iterator into two collections.
    ///
    /// This is a failable version of [`Iterator::unzip`].
    ///
    /// # Examples
    ///
    /// ```
    /// use collect_failable::{TryUnzip, OneOf2, ValueCollision};
    /// use std::collections::HashSet;
    ///
    /// let data = vec![(1, 2), (2, 3)];
    /// let (a, b): (HashSet<i32>, HashSet<i32>) = data.into_iter().try_unzip().unwrap();
    ///
    /// assert_eq!(a, HashSet::from([1, 2]));
    /// assert_eq!(b, HashSet::from([2, 3]));
    /// ```
    fn try_unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), OneOf2<FromA::Error, FromB::Error>>
    where
        FromA: Default + TryExtend<A>,
        FromB: Default + TryExtend<B>,
        Self: Iterator<Item = (A, B)>;
}

impl<I> TryUnzip for I
where
    I: Iterator,
{
    fn try_unzip<A, B, FromA, FromB>(mut self) -> Result<(FromA, FromB), OneOf2<FromA::Error, FromB::Error>>
    where
        FromA: Default + TryExtend<A>,
        FromB: Default + TryExtend<B>,
        Self: Iterator<Item = (A, B)>,
    {
        self.try_fold((FromA::default(), FromB::default()), |mut tuple, (a, b)| {
            tuple.0.try_extend(std::iter::once(a)).map_err(OneOf2::A)?;
            tuple.1.try_extend(std::iter::once(b)).map_err(OneOf2::B)?;
            Ok(tuple)
        })
    }
}
