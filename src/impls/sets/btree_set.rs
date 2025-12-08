use std::collections::BTreeSet;

use fluent_result::bool::Expect;

use crate::{CollectionCollision, TryExtend, TryExtendSafe, TryFromIterator};

impl<T: Ord, I> TryFromIterator<T, I> for BTreeSet<T>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionCollision<T, I::IntoIter, BTreeSet<T>>;

    /// Converts `iter` into a [`BTreeSet`], failing if a value would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut iter = into_iter.into_iter();
        iter.try_fold(BTreeSet::new(), |mut set, value| match set.contains(&value) {
            true => Err((set, value)),
            false => {
                set.insert(value).expect_true("should not be occupied");
                Ok(set)
            }
        })
        .map_err(|(set, value)| CollectionCollision::new(iter, set, value))
    }
}

impl<T: Ord, I> TryExtend<T, I> for BTreeSet<T>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionCollision<T, I::IntoIter, BTreeSet<T>>;

    /// Extends the set with `iter`, failing if a value would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        iter.try_for_each(|value| match self.contains(&value) {
            true => Err(value),
            false => {
                self.insert(value).expect_true("Should not be occupied");
                Ok(())
            }
        })
        .map_err(|value| CollectionCollision::new(iter, BTreeSet::new(), value))
    }
}

impl<T: Ord, I> TryExtendSafe<T, I> for BTreeSet<T>
where
    I: IntoIterator<Item = T>,
{
    /// Extends the set with `iter`, erroring if a value would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        iter.try_fold(BTreeSet::new(), |mut set, value| match self.contains(&value) {
            true => Err((set, value)),
            false => match set.contains(&value) {
                true => Err((set, value)),
                false => {
                    set.insert(value).expect_true("should not be occupied");
                    Ok(set)
                }
            },
        })
        .map(|set| self.extend(set))
        .map_err(|(set, value)| CollectionCollision::new(iter, set, value))
    }
}
