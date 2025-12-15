use std::collections::BTreeMap;

use fluent_result::expect::dbg::ExpectNone;

use crate::{CollectionCollision, TryExtend, TryExtendSafe, TryFromIterator};

impl<K: Ord, V, I> TryFromIterator<(K, V), I> for BTreeMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionCollision<(K, V), I::IntoIter, Self>;

    /// Converts `iter` into a [`BTreeMap`], failing if a key would collide.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();
        iter.try_fold(Self::new(), |mut map, (k, v)| match map.contains_key(&k) {
            true => Err((map, (k, v))),
            false => {
                map.insert(k, v).expect_none("should not be occupied");
                Ok(map)
            }
        })
        .map_err(|(map, kvp)| CollectionCollision::new(iter, map, kvp))
    }
}

impl<K: Ord, V, I> TryExtend<(K, V), I> for BTreeMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    type Error = CollectionCollision<(K, V), I::IntoIter, Self>;

    /// Extends the map with `iter`, failing if a key would collide, with a basic error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        iter.try_for_each(|(key, value)| match self.contains_key(&key) {
            true => Err((key, value)),
            false => {
                self.insert(key, value).expect_none("should not be occupied");
                Ok(())
            }
        })
        .map_err(|kvp| CollectionCollision::new(iter, Self::new(), kvp))
    }
}

impl<K: Ord, V, I> TryExtendSafe<(K, V), I> for BTreeMap<K, V>
where
    I: IntoIterator<Item = (K, V)>,
{
    /// Extends the map with `iter`, erroring if a key would collide, with a strong error guarantee.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
        let mut iter = iter.into_iter();
        iter.try_fold(Self::new(), |mut map, (key, value)| match self.contains_key(&key) {
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
