use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use fluent_result::IntoResult;
use size_guess::SizeGuess;
use tap::Pipe;

use crate::{KeyCollision, TryFromIterator};

impl<K: Eq + Hash, V> TryFromIterator<(K, V)> for HashMap<K, V> {
    type Error = KeyCollision<K>;

    /// Converts an iterator of key-value pairs into a hash-map, failing if a key would collide.
    ///
    /// Note: In the case of a collision, technically the key returned by [KeyCollision] is the
    /// first key that was seen during iteration, not the second key that collided. This may be
    /// relevant for keys that are [Eq] but still have different values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use collect_failable::TryFromIterator;
    ///
    /// let err = HashMap::try_from_iter([(1, 2), (1, 3)]).expect_err("should be err");
    /// assert_eq!(err.key, 1);
    /// ```
    fn try_from_iter<I>(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
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
