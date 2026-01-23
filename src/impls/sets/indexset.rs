use core::hash::{BuildHasher, Hash};

use fluent_result::bool::dbg::Expect;
use indexmap::IndexSet;

use crate::errors::{CollectionError, Collision};
use crate::impls::try_extend_basic;
use crate::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

impl<T: Eq + Hash, I> TryFromIterator<I> for IndexSet<T>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, Self, Collision<T>>;

    /// Converts `iter` into a [`IndexSet`], failing if a value would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut iter = into_iter.into_iter();
        let mut set = Self::with_capacity(iter.size_hint().0);

        match try_extend_basic(&mut set, &mut iter) {
            Ok(()) => Ok(set),
            Err(err) => Err(CollectionError::new(iter, set, err)),
        }
    }
}

impl<T: Eq + Hash, S: BuildHasher, I> TryExtend<I> for IndexSet<T, S>
where
    I: IntoIterator<Item = T>,
{
    type Error = CollectionError<I::IntoIter, IndexSet<T>, Collision<T>>;

    /// Extends the set with `iter`, failing if a value would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        try_extend_basic(self, &mut iter).map_err(|err| CollectionError::new(iter, IndexSet::new(), err))
    }
}

impl<T: Eq + Hash, S: BuildHasher, I> TryExtendSafe<I> for IndexSet<T, S>
where
    I: IntoIterator<Item = T>,
{
    /// Extends the set with `iter`, erroring if a value would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        let size_guess = iter.size_hint().0;

        iter.try_fold(IndexSet::with_capacity(size_guess), |mut set, value| match self.contains(&value) {
            true => Err((set, Collision::new(value))),
            false => match set.try_extend_one(value) {
                Ok(()) => Ok(set),
                Err(err) => Err((set, err)),
            },
        })
        .map(|set| self.extend(set))
        .map_err(|(set, err)| CollectionError::new(iter, set, err))
    }
}

impl<T: Eq + Hash, S: BuildHasher> TryExtendOne for IndexSet<T, S> {
    type Item = T;
    type Error = Collision<T>;

    fn try_extend_one(&mut self, item: T) -> Result<(), Self::Error> {
        match self.contains(&item) {
            true => Err(Collision::new(item)),
            false => {
                self.insert(item).expect_true("should not be occupied");
                Ok(())
            }
        }
    }
}
