use alloc::boxed::Box;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::iter::Chain;
use core::ops::Deref;
use display_as_debug::fmt::DebugStructExt;
use display_as_debug::wrap::Short;

use tap::Pipe;

use crate::SizeHint;

use super::{CapacityError, Collision, ErrorItemProvider};

/// An error that occurs when collecting an [`IntoIterator`] fails during its collection.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator.
/// - `C`: The type of the container for collected values. Note this this may or may not be
///   different from the container type that was the target of the collection operation.
/// - `E`: The type of the nested error.
///
/// # Data Recovery
///
/// This type is designed to capture all state in the event of a collection failure. If `E`
/// implements [`ErrorItemProvider`] and `C` implements [`IntoIterator`], (which all
/// implementations in this crate do) then this information can be used to recreate an iterator with
/// the same values as was originally provided via [`IntoIterator::into_iter`].
///
/// # Read-Only
///
/// Note that this type is *read-only*. The fields may be borrowed via a hidden [`Deref`]
/// implementation into a hidden `CollectionErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`CollectionError::into_data`] to get owned data.
#[subdef::subdef]
pub struct CollectionError<I, C, E> {
    #[cfg(doc)]
    /// The iterator used to produce the collected values, which may be partially or fully iterated,
    /// depending on the error.
    pub iterator: I,
    #[cfg(doc)]
    /// The values that were collected - it may be empty if no items were collected.
    pub collected: C,
    #[cfg(doc)]
    /// The error that occurred - this value may contain an additional element which was produced
    /// by the iterator, but rejected by the collection.
    pub error: E,

    #[cfg(not(doc))]
    data: [Box<CollectionErrorData<I, C, E>>; {
        /// The internal data of a [`CollectionError`].
        #[doc(hidden)]
        pub struct CollectionErrorData<I, C, E> {
            /// The iterator used to produce the collected values, which may be partially or fully
            /// iterated, depending on the error.
            pub iterator: I,
            /// The values that were collected - it may be empty if no items were collected.
            pub collected: C,
            /// The error that occurred - this value may contain an additional element which was
            /// produced by the iterator, but rejected by the collection.
            pub error: E,
        }
    }],
}

impl<I, C, E> CollectionError<I, C, E> {
    /// Creates a new [`CollectionError`].
    ///
    /// # Arguments
    ///
    /// * `iterator` - The iterator used to produce the collected values.
    /// * `collected` - The values that were collected.
    /// * `error` - The error that occurred.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CollectionError};
    /// use collect_failable::SizeHint;
    ///
    /// let error = CollectionError::new(
    ///     1..=3,
    ///     vec![1, 2].into_iter().collect::<Vec<_>>(),
    ///     CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)),
    /// );
    /// ```
    pub fn new(iterator: I, collected: C, error: E) -> Self {
        CollectionErrorData { iterator, collected, error }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning a `CollectionErrorData` containing the [`CollectionError::iterator`],
    /// [`CollectionError::collected`] values, and [`CollectionError::error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CollectionError};
    /// use collect_failable::SizeHint;
    ///
    /// let error = CollectionError::new(
    ///     1..=3,
    ///     vec![1, 2].into_iter().collect::<Vec<_>>(),
    ///     CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)),
    /// );
    ///
    /// let data = error.into_data();
    /// assert_eq!(data.iterator, 1..=3);
    /// assert_eq!(data.collected, vec![1, 2]);
    /// assert_eq!(data.error, CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)));
    /// ```
    #[must_use]
    pub fn into_data(self) -> CollectionErrorData<I, C, E> {
        *self.data
    }
}

#[doc(hidden)]
impl<I, C, E> Deref for CollectionError<I, C, E> {
    type Target = CollectionErrorData<I, C, E>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<I, C, E: Debug> Debug for CollectionErrorData<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CollectionErrorData")
            .field_type::<I, Short>("iterator")
            .field_type::<C, Short>("collected")
            .field("error", &self.error)
            .finish()
    }
}

/// Specialization of [`CollectionError`] for [`CapacityError`].
///
/// This type is used when collection or extension fails due to capacity constraints, such as
/// with fixed-size arrays or vectors. The [`CapacityError::kind`] field identifies the specific
/// failure mode.
///
/// Since [`CapacityError`] implements [`ErrorItemProvider`], rejected items can be recovered,
/// allowing the original iterator to be reconstructed from [`CollectionError::iterator`],
/// [`CollectionError::collected`], and the rejected item.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator that was used to produce the values.
/// - `C`: The type of the collection that was used to collect the values.
///
/// # Examples
///
/// ```rust
/// use collect_failable::errors::{CapacityError, CollectionError};
/// use collect_failable::SizeHint;
///
/// let error = CollectionError::new(1..=1, vec![2], CapacityError::overflow(SizeHint::exact(1), 3));
///
/// let values: Vec<_> = error.into_iter().collect();
///
/// assert_eq!(values, vec![3, 2, 1]);
/// ```
impl<I: Iterator, C> CollectionError<I, C, CapacityError<I::Item>> {
    /// Creates a new [`CollectionError`] with a [`CapacityError`] of type
    /// [`CapacityErrorKind::Bounds`](crate::errors::CapacityErrorKind::Bounds) for collection
    /// failures when a pre-check of the iterator's [`size_hint`](Iterator::size_hint) indicates
    /// that the iterator cannot fit within the specified capacity.
    ///
    /// A [`Default`]ed collection is used for [`CollectionError::collected`] since no items are
    /// collected in this case.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The iterator that failed the bounds check
    /// * `capacity` - The allowed capacity range for the collection
    ///
    /// # Panics
    ///
    /// Panics if the iterator's size hint is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CollectionError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let error = CollectionError::<_, Vec<()>, _>::bounds(1..=3, SizeHint::exact(2));
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    #[must_use]
    #[inline]
    pub fn bounds(iterator: I, capacity: SizeHint) -> Self
    where
        C: Default,
    {
        let hint = iterator.size_hint().try_into().expect("Invalid size hint");
        Self::new(iterator, C::default(), CapacityError::bounds(capacity, hint))
    }

    /// Creates a new [`CollectionError`] with a [`CapacityError`] of type
    /// [`CapacityErrorKind::Overflow`](crate::errors::CapacityErrorKind::Overflow) for
    /// collection failures when `iterator` produced more items than `capacity`.
    ///
    /// For extension operations that fill a collection to capacity, and leave it in a full state,
    /// use [`CollectionError::overflowed`] instead.
    ///
    /// The exact number of items that overflowed is implied to be at least one, but may be greater.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `rejected` - The item that was consumed but couldn't be added to the collection
    /// * `capacity` - The allowed capacity range for the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CollectionError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let error = CollectionError::overflow(1..=3, vec![1, 2], 3, SizeHint::exact(2));
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { rejected: 3 });
    /// ```
    #[must_use]
    #[inline]
    pub fn overflow(iterator: I, collected: C, rejected: I::Item, capacity: SizeHint) -> Self {
        Self::new(iterator, collected, CapacityError::overflow(capacity, rejected))
    }

    /// Creates a new [`CollectionError`] for failed extension operations that filled the target
    /// container to capacity, but more items are still available.
    ///
    /// In this case, [`CapacityError::capacity`] is [`SizeHint::ZERO`], and a [`Default`]ed
    /// collection is used for [`CollectionError::collected`], since all values are collected into
    /// the target container, which is left in a full state.
    ///
    /// The number of items that overflowed is implied to be at least one, but may be greater.
    ///
    /// # When to Use
    ///
    /// This constructor is intended for **extension operations** that leave the target collection
    /// in a full state, such as [`TryExtend::try_extend`](crate::TryExtend::try_extend). For
    /// overflow failures on operations that do not leave the target collection in a full state,
    /// use [`CollectionError::overflow`] instead.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after overflow occurred
    /// * `rejected` - The item that was consumed but couldn't be added to the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CollectionError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let error = CollectionError::<_, Vec<()>, _>::overflowed(1..=3, 3);
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![]);
    /// assert_eq!(error.error.capacity, SizeHint::ZERO);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { rejected: 3 });
    /// ```
    #[must_use]
    #[inline]
    pub fn overflowed(iterator: I, rejected: I::Item) -> Self
    where
        C: Default,
    {
        Self::new(iterator, C::default(), CapacityError::overflowed(rejected))
    }

    /// Creates a new [`CollectionError`] with a [`CapacityError`] of type
    /// [`CapacityErrorKind::Underflow`](crate::errors::CapacityErrorKind::Underflow) for
    /// collection failures when `iterator` produced fewer items than `capacity`.
    ///
    /// Note: Due to limitations of rust's type inference, it may be necessary to explicitly
    /// specify the type of `collected` when using this method.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after underflow occurred
    /// * `collected` - The values that were collected before underflow
    /// * `capacity` - The allowed capacity range for the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CollectionError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let error = CollectionError::<_, Vec<_>, _>::underflow(1..=3, vec![1, 2], SizeHint::exact(2));
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    #[inline]
    pub fn underflow(iterator: I, collected: C, capacity: SizeHint) -> Self
    where
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        let count = (&collected).into_iter().len();
        Self::new(iterator, collected, CapacityError::underflow(capacity, count))
    }
}

/// Specialization of [`CollectionError`] for [`Collision`].
///
/// This type is used when collection fails due to a collision. Such as a duplicate key in a map or set.
/// The [`Collision`] will contain the item that collided during collection.
///
/// Since [`Collision`] implements [`ErrorItemProvider`], rejected items can be recovered,
/// allowing the original iterator values to be reconstructed from [`CollectionError::iterator`],
/// [`CollectionError::collected`], and [`CollectionError::error`], though the order may differ.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator that was used to collect the values.
/// - `C`: The type of the collection that was used to collect the values.
///
/// # Examples
///
/// ```rust
/// use collect_failable::errors::{CollectionError, Collision};
/// use std::collections::HashSet;
///
/// let error = CollectionError::<_, HashSet<_>, _>::collision(1..=1, HashSet::from([1]), 1);
///
/// let values = error.into_iter().collect::<Vec<_>>();
///
/// assert_eq!(values.len(), 3, "Should have 3 values");
/// assert!(values.iter().all(|v| v == &1), "Should only contain 1");
/// ```
impl<I: Iterator, C> CollectionError<I, C, Collision<I::Item>> {
    /// Creates a new [`CollectionError`] with a [`Collision`] error, for collection failures
    /// due to a collision.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after the collision occurred
    /// * `collected` - The values that were collected before the collision
    /// * `item` - The item that caused the collision
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CollectionError, Collision};
    /// use std::collections::HashSet;
    ///
    /// let error = CollectionError::<_, HashSet<_>, _>::collision(1..=3, HashSet::from([1, 2]), 3);
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, HashSet::from([1, 2]));
    /// assert_eq!(error.error.item, 3);
    /// ```
    #[must_use]
    #[inline]
    pub fn collision(iterator: I, collected: C, item: I::Item) -> Self {
        Self::new(iterator, collected, Collision::new(item))
    }
}

impl<I, C, E> IntoIterator for CollectionError<I, C, E>
where
    I: Iterator,
    C: IntoIterator<Item = I::Item>,
    E: ErrorItemProvider<Item = I::Item>,
{
    type Item = I::Item;
    type IntoIter = Chain<Chain<core::option::IntoIter<I::Item>, C::IntoIter>, I>;

    /// Consumes the error and reconstructs the iterator it was created from. This will include
    /// the rejected item (if any from the error), the `collected` values, and the remaining `iterator`, in that order.
    fn into_iter(self) -> Self::IntoIter {
        let rejected = self.data.error.into_item();
        rejected.into_iter().chain(self.data.collected).chain(self.data.iterator)
    }
}

impl<I, C, E: Display> Display for CollectionError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Collection Error: {}", self.error)
    }
}

impl<I, C, E: Error + 'static> Error for CollectionError<I, C, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<I, C, E: Debug> Debug for CollectionError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CollectionError")
            .field("collected", &core::any::type_name::<C>())
            .field("error", &self.error)
            .field("iterator", &core::any::type_name::<I>())
            .finish()
    }
}
