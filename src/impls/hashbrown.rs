use std::hash::Hash;

use fluent_result::IntoResult;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use size_guess::SizeGuess;
use tap::Pipe;

use crate::{KeyCollision, NonUniqueKey, TryExtend, TryFromIterator};

impl<K: Eq + Hash, V> TryFromIterator<(K, V)> for HashMap<K, V> {
    type Error = KeyCollision<K>;

    /// Converts an iterator of key-value pairs into a [`HashMap`] failing if a key would collide.
    ///
    /// Note: In the case of a collision, technically the key returned by [`KeyCollision`] is the
    /// first key that was seen during iteration, not the second key that collided. This may be
    /// relevant for keys that are [`Eq`] but still have different values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hashbrown::HashMap;
    /// use collect_failable::TryFromIterator;
    ///
    /// let err = HashMap::try_from_iter([(1, 2), (1, 3)]).expect_err("should be err");
    /// assert_eq!(err.key, 1);
    ///
    /// let map = HashMap::try_from_iter([(1, 2), (2, 3)]).expect("should be ok");
    /// assert_eq!(map.len(), 2);
    /// assert_eq!(map.get(&1), Some(&2));
    /// assert_eq!(map.get(&2), Some(&3));
    /// ```
    fn try_from_iter<I>(into_iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_guess();

        iter.try_fold(
            HashMap::with_capacity(size_guess),
            |mut map, (k, v)| match map.entry(k) {
                Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
                Entry::Vacant(entry) => {
                    entry.insert(v);
                    Ok(map)
                }
            },
        )
    }
}

impl<K: Eq + Hash, V> TryExtend<(K, V)> for HashMap<K, V> {
    type Error = NonUniqueKey;

    /// Appends an iterator of key-value pairs to the map, failing if a key would collide.
    ///
    /// This implementation provides a strong error guarantee. If the method returns an error, the
    /// map is not modified.
    ///
    /// Note: Due to the limitations of the `Entry` API, it is not possible to return the key that
    /// caused a collision.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hashbrown::HashMap;
    /// use collect_failable::TryExtend;
    ///
    /// let mut map = HashMap::from([(1, 2)]);
    /// map.try_extend([(1, 3)]).expect_err("should be err");
    ///
    /// // map is unchanged
    /// assert_eq!(map.len(), 1);
    /// assert_eq!(map.get(&1), Some(&2));
    ///
    /// // functions as normal extend if there are no collisions
    /// map.try_extend([(2, 3)]).expect("should be err");
    ///
    /// assert_eq!(map.len(), 2);
    /// assert_eq!(map.get(&1), Some(&2));
    /// assert_eq!(map.get(&2), Some(&3));
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let iter = iter.into_iter();
        let size_guess = iter.size_guess();

        let mut insert_map = HashMap::with_capacity(size_guess);

        for (key, value) in iter {
            match self.entry(key) {
                Entry::Vacant(entry) => match insert_map.insert(entry.into_key(), value) {
                    None => (),
                    Some(_) => return Err(NonUniqueKey),
                },
                Entry::Occupied(_) => return Err(NonUniqueKey),
            }
        }

        self.extend(insert_map);

        Ok(())
    }
}
