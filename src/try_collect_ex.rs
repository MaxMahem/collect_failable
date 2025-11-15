use crate::TryFromIterator;

/// Extends [Iterator] with a failable collect method.
pub trait TryCollectEx {
    type Item;

    /// Collects the iterator into a container, returning an error if construcing the container fails.
    ///
    /// This is a wrapper for [TryFromIterator::try_from_iter].
    ///
    /// This is useful if you want to collect an iterator into a container that may fail to be constructed.
    ///
    /// Ideally this would be called `try_collect` but there is a method with that name in nightly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use collect_failable::{TryCollectEx, TryFromIterator, KeyCollision};
    ///
    /// let result = [(1, 2), (2, 3)].into_iter().try_collect_ex::<HashMap<_, _>>();
    /// assert_eq!(result, Ok(HashMap::from([(1, 2), (2, 3)])));
    ///
    /// let result = [(1, 2), (1, 3)].into_iter().try_collect_ex::<HashMap<_, _>>();
    /// assert_eq!(result, Err(KeyCollision { key: 1 }));
    /// ```
    fn try_collect_ex<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
        Self: Sized;
}

impl<I, T> TryCollectEx for I
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn try_collect_ex<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
    {
        C::try_from_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::TryCollectEx;

    use std::collections::HashMap;

    #[test]
    fn failable_collect_no_collision_matches_collect() {
        let data = [(1, 2), (2, 3)];

        let collect_map = data.into_iter().collect::<HashMap<_, _>>();
        let try_collect_map = data
            .into_iter()
            .try_collect_ex::<HashMap<_, _>>()
            .expect("should be ok");

        // matches collect implementation
        assert_eq!(collect_map, try_collect_map);
    }

    #[test]
    fn failable_collect_collision_fails() {
        // colliding keys errors
        let try_collect_err = [(1, 2), (1, 3)]
            .into_iter()
            .try_collect_ex::<HashMap<_, _>>()
            .expect_err("should be err");

        assert_eq!(try_collect_err.key, 1);
    }
}
