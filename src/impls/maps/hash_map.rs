use core::hash::{BuildHasher, Hash};
use std::collections::HashMap;

use fluent_result::expect::dbg::ExpectNone;

use crate::errors::{CollectionCollision, ItemCollision};
use crate::{TryExtend, TryExtendSafe, TryFromIterator};

#[allow(clippy::implicit_hasher)]
impl<K: Eq + Hash, V, I> TryFromIterator<I> for HashMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionCollision<I::IntoIter, Self>;

    /// Converts `iter` into a [`HashMap`], failing if a key would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_hint().0;

        iter.try_fold(Self::with_capacity(size_guess), |mut map, (k, v)| match map.contains_key(&k) {
            true => Err((map, (k, v))),
            false => {
                map.insert(k, v).expect_none("should not be occupied");
                Ok(map)
            }
        })
        .map_err(|(map, kvp)| CollectionCollision::new(iter, map, kvp))
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, I> TryExtend<I> for HashMap<K, V, S>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionCollision<I::IntoIter, HashMap<K, V>>;

    /// Extends the map with `iter`, failing if a key would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        self.reserve(iter.size_hint().0);

        iter.try_for_each(|(key, value)| match self.contains_key(&key) {
            true => Err((key, value)),
            false => Ok(_ = self.insert(key, value)),
        })
        .map_err(|kvp| CollectionCollision::new(iter, HashMap::new(), kvp))
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
            true => Err((map, (key, value))),
            false => match map.contains_key(&key) {
                true => Err((map, (key, value))),
                false => {
                    map.insert(key, value).expect_none("should not be occupied");
                    Ok(map)
                }
            },
        })
        .map(|map| self.extend(map))
        .map_err(|(map, kvp)| CollectionCollision::new(iter, map, kvp))
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> crate::TryExtendOne for HashMap<K, V, S> {
    type Item = (K, V);
    type Error = ItemCollision<(K, V)>;

    fn try_extend_one(&mut self, item: (K, V)) -> Result<(), Self::Error> {
        let (key, value) = item;
        match self.contains_key(&key) {
            true => Err(ItemCollision::new((key, value))),
            false => {
                self.insert(key, value).expect_none("should not be occupied");
                Ok(())
            }
        }
    }
}
