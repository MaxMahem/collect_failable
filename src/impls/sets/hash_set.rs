use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

use fluent_result::bool::dbg::Expect;

use crate::{CollectionCollision, TryExtend, TryExtendSafe, TryFromIterator};

#[allow(clippy::implicit_hasher)]
impl<T: Eq + Hash, I> TryFromIterator<T, I> for HashSet<T>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionCollision<T, I::IntoIter, HashSet<T>>;

    /// Converts `iter` into a [`HashSet`], failing if a value would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_hint().0;

        iter.try_fold(HashSet::with_capacity(size_guess), |mut set, value| match set.contains(&value) {
            true => Err((set, value)),
            false => {
                set.insert(value).expect_true("should not be occupied");
                Ok(set)
            }
        })
        .map_err(|(set, value)| CollectionCollision::new(iter, set, value))
    }
}

impl<T: Eq + Hash, S: BuildHasher, I> TryExtend<T, I> for HashSet<T, S>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionCollision<T, I::IntoIter, HashSet<T>>;

    /// Extends the set with `iter`, failing if a value would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        iter.try_for_each(|value| match self.contains(&value) {
            true => Err(value),
            false => Ok(_ = self.insert(value)),
        })
        .map_err(|value| CollectionCollision::new(iter, HashSet::new(), value))
    }
}

impl<T: Eq + Hash, S: BuildHasher, I> TryExtendSafe<T, I> for HashSet<T, S>
where
    I: IntoIterator<Item = T>,
{
    /// Extends the set with `iter`, erroring if a value would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        let size_guess = iter.size_hint().0;

        iter.try_fold(HashSet::with_capacity(size_guess), |mut set, value| match self.contains(&value) {
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
