#[cfg(doc)]
use crate::TryFromIterator;

#[cfg(doc)]
use std::collections::HashMap;

use std::iter::Chain;

use tap::Pipe;

/// An error that occurs when an iterated value collides with a value mid way through iteration.
///
/// For example, when using [`TryFromIterator::try_from_iter`] on an a type (such as a [`HashMap`])
/// that does not allow duplicate keys, this error can return the values collected so far, the
/// partially iterated iter, and the colliding item, allowing those values to be handled as desired,
/// or even the initial iterator to be reconstructed from those components.
#[derive(derive_more::Deref, thiserror::Error)]
#[error("Collection collision")]
#[deref(forward)]
pub struct CollectionCollision<T, I, C>(Box<ReadOnlyCollectionCollision<T, I, C>>)
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>;

impl<T, I, C> CollectionCollision<T, I, C>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
{
    /// Creates a new [`CollectionCollision`] from an `iterator`, `collected` values, and a colliding `item`.
    pub fn new(iterator: I, collected: C, item: T) -> Self {
        ReadOnlyCollectionCollision { iterator, collected, item }.pipe(Box::new).pipe(CollectionCollision)
    }

    /// Consumes the error, returning the colliding item.
    #[must_use]
    pub fn into_item(self) -> T {
        self.0.item
    }

    /// Consumes the error, returning a [`ReadOnlyCollectionCollision`] containing the `iterator`,
    /// `collected` values, and colliding `item`.
    #[must_use]
    pub fn into_parts(self) -> ReadOnlyCollectionCollision<T, I, C> {
        *self.0
    }

    /// Returns the number of elements in the `iterator` and `collected` values.
    #[must_use]
    pub fn len(&self) -> usize
    where
        I: ExactSizeIterator,
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        (&self.0.collected).into_iter().len() + self.0.iterator.len() + 1
    }

    /// Always returns `false` (presence of a colliding item precludes an empty collection).
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        I: ExactSizeIterator,
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        false
    }
}

impl<T, I, C> IntoIterator for CollectionCollision<T, I, C>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
{
    type Item = T;
    type IntoIter = Chain<Chain<std::option::IntoIter<T>, C::IntoIter>, I>;

    /// Consumes the error, returning an iterator over the colliding `item`, the `collected` values,
    /// and the remaining `iterator`, in that order.
    ///
    /// The exact iteration order depends on the implementation of `IntoIterator` for `C`, and may
    /// not be the same as the order in which the values were collected.
    fn into_iter(self) -> Self::IntoIter {
        Some(self.0.item).into_iter().chain(self.0.collected).chain(self.0.iterator)
    }
}

impl<T, I, C> std::fmt::Debug for CollectionCollision<T, I, C>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionCollision")
            .field("collected", &std::any::type_name::<C>())
            .field("item", &std::any::type_name::<T>())
            .field("iterator", &std::any::type_name::<I>())
            .finish()
    }
}

/// A read only version of [`CollectionCollision`].
pub struct ReadOnlyCollectionCollision<T, I, C>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
{
    /// The iterator that was partially iterated
    pub iterator: I,
    /// The values that were collected
    pub collected: C,
    /// The item that caused the collision
    pub item: T,
}
