use core::hash::{BuildHasher, Hash};
use std::collections::HashMap;

use fluent_result::expect::dbg::ExpectNone;

use crate::errors::{CollectionError, Collision};
use crate::impls::try_extend_basic;
use crate::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

#[allow(clippy::implicit_hasher)]
impl<K: Eq + Hash, V, I> TryFromIterator<I> for HashMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionError<I::IntoIter, Self, Collision<(K, V)>>;

    /// Converts `iter` into a [`HashMap`], failing if a key would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut iter = into_iter.into_iter();
        let mut map = Self::with_capacity(iter.size_hint().0);

        match try_extend_basic(&mut map, &mut iter) {
            Ok(()) => Ok(map),
            Err(err) => Err(CollectionError::new(iter, map, err)),
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, I> TryExtend<I> for HashMap<K, V, S>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionError<I::IntoIter, HashMap<K, V>, Collision<(K, V)>>;

    /// Extends the map with `iter`, failing if a key would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        try_extend_basic(self, &mut iter).map_err(|err| CollectionError::new(iter, HashMap::new(), err))
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, I> TryExtendSafe<I> for HashMap<K, V, S>
where
    I: IntoIterator<Item = (K, V)>,
{
    /// Extends the map with `iter`, erroring if a key would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        let size_guess = iter.size_hint().0;

        iter.try_fold(HashMap::with_capacity(size_guess), |mut map, (key, value)| match self.contains_key(&key) {
            true => Err((map, Collision::new((key, value)))),
            false => match map.try_extend_one((key, value)) {
                Ok(()) => Ok(map),
                Err(err) => Err((map, err)),
            },
        })
        .map(|map| self.extend(map))
        .map_err(|(map, err)| CollectionError::new(iter, map, err))
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> TryExtendOne for HashMap<K, V, S> {
    type Item = (K, V);
    type Error = Collision<(K, V)>;

    fn try_extend_one(&mut self, item: (K, V)) -> Result<(), Self::Error> {
        let (key, value) = item;
        match self.contains_key(&key) {
            true => Err(Collision::new((key, value))),
            false => {
                self.insert(key, value).expect_none("should not be occupied");
                Ok(())
            }
        }
    }
}
