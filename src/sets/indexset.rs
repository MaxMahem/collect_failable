use std::hash::{BuildHasher, Hash};

use fluent_result::IntoResult;
use indexmap::IndexSet;
use size_guess::SizeGuess;

use crate::{TryExtend, TryFromIterator, ValueCollision};

impl<T: Eq + Hash> TryFromIterator<T> for IndexSet<T> {
    type Error = ValueCollision<T>;

    /// Converts an iterator of values into a [`IndexSet`], failing if a key would collide.
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

        iter.try_fold(IndexSet::with_capacity(size_guess), |mut set, t| {
            match set.contains(&t) {
                true => return Err(ValueCollision::new(t)),
                false => set.insert(t),
            };

            Ok(set)
        })
    }
}

impl<T: Eq + Hash, S: BuildHasher> TryExtend<T> for IndexSet<T, S> {
    type Error = ValueCollision<T>;

    /// Appends an iterator of values pairs to the set, failing if a value would collide.
    ///
    /// This implementation provides a strong error guarantee. If the method returns an error, the
    /// set is not modified.
    ///
    /// See [trait level documentation](trait@TryExtend) for an example.
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let size_guess = iter.size_guess();

        let mut insert_set = IndexSet::with_capacity(size_guess);

        for value in iter {
            match self.contains(&value) {
                true => return Err(ValueCollision::new(value)),
                false => match insert_set.contains(&value) {
                    true => return ValueCollision::new(value).into_err(),
                    false => _ = insert_set.insert(value),
                },
            }
        }

        self.extend(insert_set);

        Ok(())
    }
}
