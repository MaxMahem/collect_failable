use alloc::collections::BTreeMap;

use fluent_result::expect::dbg::ExpectNone;

use crate::errors::{CollectionError, Collision};
use crate::impls::try_extend_basic;
use crate::{TryExtend, TryExtendOne, TryExtendSafe, TryFromIterator};

impl<K: Ord, V, I> TryFromIterator<I> for BTreeMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionError<I::IntoIter, Self, Collision<(K, V)>>;

    /// Converts `iter` into a [`BTreeMap`], failing if a key would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();
        let mut map = Self::new();
        match try_extend_basic(&mut map, &mut iter) {
            Ok(()) => Ok(map),
            Err(err) => Err(CollectionError::new(iter, map, err)),
        }
    }
}

impl<K: Ord, V, I> TryExtend<I> for BTreeMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionError<I::IntoIter, Self, Collision<(K, V)>>;

    /// Extends the map with `iter`, failing if a key would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        try_extend_basic(self, &mut iter).map_err(|err| CollectionError::new(iter, Self::new(), err))
    }
}

impl<K: Ord, V, I> TryExtendSafe<I> for BTreeMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    /// Extends the map with `iter`, erroring if a key would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        iter.try_fold(Self::new(), |mut map, (key, value)| match self.contains_key(&key) {
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

impl<K: Ord, V> TryExtendOne for BTreeMap<K, V> {
    type Item = (K, V);
    type Error = Collision<(K, V)>;

    fn try_extend_one(&mut self, (key, value): (K, V)) -> Result<(), Self::Error> {
        match self.contains_key(&key) {
            true => Err(Collision::new((key, value))),
            false => {
                self.insert(key, value).expect_none("should not be occupied");
                Ok(())
            }
        }
    }
}
