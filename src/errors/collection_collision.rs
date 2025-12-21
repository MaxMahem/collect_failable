#[cfg(doc)]
use crate::TryFromIterator;

#[cfg(doc)]
use std::collections::HashMap;

use std::fmt::{Debug, Formatter};
use std::iter::{Chain, Once};

use tap::Pipe;

/// An error that occurs when an iterated value collides with a value mid way through iteration.
///
/// For example, when using [`TryFromIterator::try_from_iter`] on an a type (such as a [`HashMap`])
/// that does not allow duplicate keys, this error can return the values collected so far, the
/// partially iterated iter, and the colliding item, allowing those values to be handled as desired,
/// or even the initial iterator to be reconstructed from those components.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref)
/// implementation into a hidden `CollectionCollisionData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`CollectionCollision::into_data`] to get an owned data.
#[subdef::subdef]
#[derive(thiserror::Error)]
#[error("Collection collision")]
pub struct CollectionCollision<I: Iterator, C> {
    #[cfg(doc)]
    /// The iterator that was partially iterated
    pub iterator: I,
    #[cfg(doc)]
    /// The values that were collected
    pub collected: C,
    #[cfg(doc)]
    /// The item that caused the collision
    pub item: I::Item,

    #[cfg(not(doc))]
    data: [Box<CollectionCollisionData<I, C>>; {
        /// A read only version of [`CollectionCollision`].
        #[doc(hidden)]
        pub struct CollectionCollisionData<I: Iterator, C> {
            /// The iterator that was partially iterated
            pub iterator: I,
            /// The values that were collected
            pub collected: C,
            /// The item that caused the collision
            pub item: I::Item,
        }
    }],
}

impl<I: Iterator, C> CollectionCollision<I, C> {
    /// Creates a new [`CollectionCollision`] from an `iterator`, `collected` values, and a colliding `item`.
    pub fn new(iterator: I, collected: C, item: I::Item) -> Self {
        CollectionCollisionData { iterator, collected, item }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning the colliding item.
    #[must_use]
    pub fn into_item(self) -> I::Item {
        self.data.item
    }

    /// Consumes the error, returning a `CollectionCollisionData` containing the [`CollectionCollision::iterator`],
    /// [`CollectionCollision::collected`] values, and colliding [`CollectionCollision::item`].
    #[must_use]
    pub fn into_data(self) -> CollectionCollisionData<I, C> {
        *self.data
    }

    /// Returns the number of elements in the `iterator` and `collected` values.
    #[must_use]
    pub fn len(&self) -> usize
    where
        I: ExactSizeIterator,
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        (&self.data.collected).into_iter().len() + self.data.iterator.len() + 1
    }

    /// Always returns `false` (presence of a colliding item precludes an empty collection).
    #[must_use]
    pub const fn is_empty(&self) -> bool
    where
        I: ExactSizeIterator,
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        false
    }
}

#[doc(hidden)]
impl<I: Iterator, C> std::ops::Deref for CollectionCollision<I, C> {
    type Target = CollectionCollisionData<I, C>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I: Iterator, C: IntoIterator<Item = I::Item>> IntoIterator for CollectionCollision<I, C> {
    type Item = I::Item;
    type IntoIter = Chain<Chain<Once<I::Item>, C::IntoIter>, I>;

    /// Consumes the error, returning an iterator over the colliding `item`, the `collected` values,
    /// and the remaining `iterator`, in that order.
    ///
    /// The exact iteration order depends on the implementation of `IntoIterator` for `C`, and may
    /// not be the same as the order in which the values were collected.
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.data.item).chain(self.data.collected).chain(self.data.iterator)
    }
}

impl<I: Iterator, C> Debug for CollectionCollision<I, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionCollision")
            .field("collected", &std::any::type_name::<C>())
            .field("item", &std::any::type_name::<I::Item>())
            .field("iterator", &std::any::type_name::<I>())
            .finish()
    }
}
