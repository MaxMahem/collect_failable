/// Allows converting an iterator into a container that may fail to be constructed.
pub trait TryFromIterator<T> {
    /// The error that may occur when converting the iterator into the container.
    type Error;

    /// Converts an iterator into a container that may fail to be constructed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use collect_failable::TryFromIterator;
    ///
    /// let result = HashMap::try_from_iter([(1, 2), (1, 3)].into_iter());
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().key, 1);
    /// ```
    fn try_from_iter<I: Iterator<Item = T>>(iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// An error indicating that a duplicate key was found in the provided data.
#[derive(Debug, thiserror::Error, derive_more::Constructor)]
#[error("Key collision: {key}")]
pub struct KeyCollision<K> {
    /// The key that caused the collision.
    pub key: K,
}

#[cfg(feature = "hash_map")]
pub mod hash_map {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::hash::Hash;

    use fluent_result::IntoResult;
    use tap::Pipe;

    use crate::{KeyCollision, TryFromIterator};

    impl<K: Eq + Hash, V> TryFromIterator<(K, V)> for HashMap<K, V> {
        type Error = KeyCollision<K>;

        fn try_from_iter<I: Iterator<Item = (K, V)>>(mut iter: I) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            iter.try_fold(HashMap::new(), |mut map, (k, v)| match map.entry(k) {
                Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
                Entry::Vacant(entry) => {
                    entry.insert(v);
                    Ok(map)
                }
            })
        }
    }
}

#[cfg(feature = "btree_map")]
pub mod btree_map {
    use std::collections::btree_map::Entry;
    use std::collections::BTreeMap;
    use std::hash::Hash;

    use fluent_result::IntoResult;
    use tap::Pipe;

    use crate::{KeyCollision, TryFromIterator};

    impl<K: Eq + Hash + Ord, V> TryFromIterator<(K, V)> for BTreeMap<K, V> {
        type Error = KeyCollision<K>;

        fn try_from_iter<I: Iterator<Item = (K, V)>>(mut iter: I) -> Result<Self, Self::Error> {
            iter.try_fold(BTreeMap::new(), |mut map, (k, v)| match map.entry(k) {
                Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
                Entry::Vacant(entry) => {
                    entry.insert(v);
                    Ok(map)
                }
            })
        }
    }
}

#[cfg(feature = "hash_brown")]
pub mod hashbrown {
    use std::hash::Hash;

    use fluent_result::IntoResult;
    use hashbrown::hash_map::Entry;
    use hashbrown::HashMap;
    use tap::Pipe;

    use crate::{KeyCollision, TryFromIterator};

    impl<K: Eq + Hash, V> TryFromIterator<(K, V)> for HashMap<K, V> {
        type Error = KeyCollision<K>;

        fn try_from_iter<I: Iterator<Item = (K, V)>>(mut iter: I) -> Result<Self, Self::Error> {
            iter.try_fold(HashMap::new(), |mut map, (k, v)| match map.entry(k) {
                Entry::Occupied(entry) => entry.remove_entry().0.pipe(KeyCollision::new).into_err(),
                Entry::Vacant(entry) => {
                    entry.insert(v);
                    Ok(map)
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use hashbrown::HashMap as HashBrownMap;

    use crate::TryFromIterator;

    macro_rules! test_try_from_iter {
        ($module:ident, $map_type:ty) => {
            mod $module {
                use super::*;

                #[test]
                fn key_collision() {
                    let result = <$map_type>::try_from_iter([(1, 2), (1, 3)].into_iter());
                    assert!(result.is_err());
                    assert_eq!(result.unwrap_err().key, 1);
                }

                #[test]
                fn no_collision() {
                    let result = <$map_type>::try_from_iter([(1, 2), (2, 3)].into_iter());
                    assert!(result.is_ok());
                    let map = result.unwrap();
                    assert_eq!(map.len(), 2);
                }
            }
        };
    }

    test_try_from_iter!(hash_map, HashMap<_, _>);
    test_try_from_iter!(btree_map, BTreeMap<_, _>);
    test_try_from_iter!(hashbrown_map, HashBrownMap<_, _>);
}
