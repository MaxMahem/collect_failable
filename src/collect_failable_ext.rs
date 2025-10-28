use crate::TryFromIterator;

/// Extends [Iterator] with a failable collect method.
pub trait FailableCollectExt {
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
    /// use collect_failable::{FailableCollectExt, TryFromIterator};
    ///
    /// let result = [(1, 2), (2, 3)].into_iter().try_collect_ext::<HashMap<_, _>>();
    /// assert!(result.is_ok());
    ///
    /// let result = [(1, 2), (1, 3)].into_iter().try_collect_ext::<HashMap<_, _>>();
    /// assert!(result.is_err());
    /// ```
    fn try_collect_ext<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
        Self: Sized;
}

impl<I, T> FailableCollectExt for I
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn try_collect_ext<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
    {
        C::try_from_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::FailableCollectExt;

    use std::collections::HashMap;

    #[test]
    fn failable_collect_no_collision_matches_collect() {
        let collect = [(1, 2), (2, 3)].into_iter().collect::<HashMap<_, _>>();

        let try_collect = [(1, 2), (2, 3)]
            .into_iter()
            .try_collect_ext::<HashMap<_, _>>();

        assert!(try_collect.is_ok());
        assert_eq!(collect, try_collect.unwrap());
    }

    #[test]
    fn failable_collect_collision_fails() {
        let try_collect = [(1, 2), (1, 3)]
            .into_iter()
            .try_collect_ext::<HashMap<_, _>>();

        assert!(try_collect.is_err());

        assert_eq!(try_collect.unwrap_err().key, 1);
    }
}
