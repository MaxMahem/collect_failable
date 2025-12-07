use std::collections::BTreeSet;

use fluent_result::into::IntoResult;

use crate::utils::FoldMut;
use crate::{TryExtend, TryExtendSafe, TryFromIterator, ValueCollision};

/// Converts an iterator of values into a [`BTreeSet`], failing if a value would collide.
impl<T: Ord, I> TryFromIterator<T, I> for BTreeSet<T> 
where
    I: IntoIterator<Item = T>
{
    type Error = ValueCollision<T>;

    /// Converts an iterator of values into a [`BTreeSet`], failing if a key would collide.
    ///
    /// In the case of a collision, the value held by [`ValueCollision`] is the second value that was
    /// seen during iteration. This may be relevant for keys that compare the same but still have
    /// different values.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        into_iter.into_iter().try_fold_mut(BTreeSet::new(), |set, value| match set.contains(&value) {
            true => ValueCollision::new(value).into_err(),
            false => Ok(_ = set.insert(value)),
        })
    }
}

/// Appends an iterator of values to the [`BTreeSet`], failing if a value would collide.
impl<T: Ord> TryExtend<T> for BTreeSet<T> {
    type Error = ValueCollision<T>;

    /// Appends an iterator of values to the set, failing if a value would collide.
    ///
    /// This implementation provides a basic error guarantee. If the method returns an error, the
    /// set may be modified. However, it will still be in a valid state, and the specific
    /// collision that caused the error will not take effect.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().try_for_each(|value| match self.contains(&value) {
            true => ValueCollision::new(value).into_err(),
            false => Ok(_ = self.insert(value)),
        })
    }
}

/// Appends an iterator of values to the [`BTreeSet`] with a strong error guarantee.
impl<T: Ord> TryExtendSafe<T> for BTreeSet<T> {
    /// Appends an iterator of values pairs to the [`BTreeSet`], failing if a value would collide.
    ///
    /// This implementation provides a strong error guarantee. If the method returns an error, the
    /// set is not modified.
    ///
    /// See [trait level documentation](trait@TryExtendSafe) for an example.
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter()
            .try_fold_mut(BTreeSet::new(), |set, value| match self.contains(&value) {
                true => Err(ValueCollision::new(value)),
                false => match set.contains(&value) {
                    true => ValueCollision::new(value).into_err(),
                    false => Ok(_ = set.insert(value)),
                },
            })
            .map(|set| self.extend(set))
    }
}
