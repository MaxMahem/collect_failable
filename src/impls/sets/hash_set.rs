use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

use fluent_result::IntoResult;
use size_guess::SizeGuess;

use crate::{TryExtend, TryFromIterator, ValueCollision};

/// Converts an iterator of values into a [`HashSet`], failing if a key would collide.
#[allow(clippy::implicit_hasher)]
impl<T: Eq + Hash> TryFromIterator<T> for HashSet<T> {
    type Error = ValueCollision<T>;

    /// Converts an iterator of values into a [`HashSet`], failing if a key would collide.
    ///
    /// In the case of a collision, the value held by [`ValueCollision`] is the second value that was
    /// seen during iteration. This may be relevant for keys that compare the same but still have
    /// different values.
    ///
    /// See [trait level documentation](trait@TryFromIterator) for an example.
    fn try_from_iter<I>(into_iter: I) -> Result<Self, Self::Error>
    where
        Self: Sized,
        I: IntoIterator<Item = T>,
    {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_guess();

        iter.try_fold(HashSet::with_capacity(size_guess), |mut set, value| {
            match set.contains(&value) {
                true => return Err(ValueCollision::new(value)),
                false => set.insert(value),
            };

            Ok(set)
        })
    }
}

/// Appends an iterator of values to the [`HashSet`], failing if a value would collide.
impl<T: Eq + Hash, S: BuildHasher> TryExtend<T> for HashSet<T, S> {
    type Error = ValueCollision<T>;

    /// Appends an iterator of values pairs to the [`HashSet`], failing if a value would collide.
    ///
    /// This implementation provides a strong error guarantee. If the method returns an error, the
    /// set is not modified.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let size_guess = iter.size_guess();

        iter.try_fold(HashSet::with_capacity(size_guess), |mut set, value| match self.contains(&value) {
            true => ValueCollision::new(value).into_err(),
            false => match set.contains(&value) {
                true => ValueCollision::new(value).into_err(),
                #[rustfmt::skip]
                false => { _ = set.insert(value); Ok(set) }
            },
        })
        .map(|set| self.extend(set))
    }

    /// Appends an iterator of values to the set, failing if a value would collide.
    ///
    /// This implementation provides a weak error guarantee. If the method returns an error, the
    /// set may be modified. However, it will still be in a valid state, and the specific
    /// collision that caused the error will not take effect.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        self.reserve(iter.size_guess());

        iter.into_iter().try_for_each(|value| match self.contains(&value) {
            true => ValueCollision::new(value).into_err(),
            false => Ok(_ = self.insert(value)),
        })
    }
}
