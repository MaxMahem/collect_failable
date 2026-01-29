use alloc::boxed::Box;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::iter::Chain;
use core::ops::Deref;
use display_as_debug::fmt::DebugStructExt;
use display_as_debug::wrap::Short;

use tap::Pipe;

use super::SizeHint;
use crate::{FixedCap, RemainingCap};

use super::{CapacityError, Collision, ErrorItemProvider};

#[cfg(doc)]
use crate::errors::CapacityErrorKind;

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
/// implementation into a hidden `CollectErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`CollectError::into_data`] to get owned data.
#[subdef::subdef]
pub struct CollectError<I, C, E> {
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
    data: [Box<CollectErrorData<I, C, E>>; {
        /// The internal data of a [`CollectError`].
        #[doc(hidden)]
        pub struct CollectErrorData<I, C, E> {
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

impl<I, C, E> CollectError<I, C, E> {
    /// Creates a new [`CollectError`].
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
    /// # use collect_failable::errors::{CapacityError, CapacityErrorKind, CollectError, SizeHint};
    /// let error = CollectError::new(
    ///     1..=3,
    ///     vec![1, 2],
    ///     CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)),
    /// );
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// ```
    pub fn new(iterator: I, collected: C, error: E) -> Self {
        CollectErrorData { iterator, collected, error }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning a `CollectErrorData` containing the [`CollectError::iterator`],
    /// [`CollectError::collected`] values, and [`CollectError::error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CapacityErrorKind, CollectError, SizeHint};
    /// let error = CollectError::new(
    ///     1..=3,
    ///     vec![1, 2],
    ///     CapacityError::<i32>::bounds(SizeHint::exact(2), SizeHint::exact(3)),
    /// );
    ///
    /// let data = error.into_data();
    ///
    /// assert_eq!(data.iterator, 1..=3);
    /// assert_eq!(data.collected, vec![1, 2]);
    /// assert_eq!(data.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// assert_eq!(data.error.capacity, SizeHint::exact(2));
    /// ```
    #[must_use]
    pub fn into_data(self) -> CollectErrorData<I, C, E> {
        *self.data
    }
}

#[doc(hidden)]
impl<I, C, E> Deref for CollectError<I, C, E> {
    type Target = CollectErrorData<I, C, E>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[doc(hidden)]
#[allow(clippy::missing_fields_in_debug, reason = "All data is covered")]
impl<I, C, E: Debug> Debug for CollectErrorData<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CollectErrorData")
            .field_type::<I, Short>("iterator")
            .field_type::<C, Short>("collected")
            .field("error", &self.error)
            .finish()
    }
}

/// Specialization of [`CollectError`] for [`CapacityError`].
///
/// This type is used when collection or extension fails due to capacity constraints, such as
/// with fixed-size arrays or vectors. The [`CapacityError::kind`] field identifies the specific
/// failure mode.
///
/// # Types
///
/// - [`Bounds`](CapacityErrorKind::Bounds) - The iterator's [`size_hint`](Iterator::size_hint)
///   is incompatible with the specified capacity. The [`collected`](CollectError::collected)
///   will be [`Default`](Default::default())ed.
/// - [`Overflow`](CapacityErrorKind::Overflow) - The collection or extension operation overflowed.
///   The first item that overflowed is captured in [`rejected`](CollectError::rejected), but there
///   may be more overflowing items remaining in the iterator.
/// - [`Underflow`](CapacityErrorKind::Underflow) - The collection underflowed. The count of
///   underflowing items is derived from [`collected`](CollectError::collected).
///
/// # Type Parameters
///
/// - `I`: The type of the iterator that was used to produce the values.
/// - `C`: The type of the collection that was used to collect the values.
///
/// # Data Recovery
///
/// Since [`CapacityError`] implements [`ErrorItemProvider`], if `C` implements [`IntoIterator`],
/// rejected items can be recovered, allowing the original iterator to be reconstructed from
/// [`CollectError::iterator`], [`CollectError::collected`], and the rejected item.
///
/// # Examples
///
/// ```rust
/// # use collect_failable::errors::{CapacityError, CollectError, SizeHint};
/// let error = CollectError::new(1..=1, vec![2], CapacityError::overflow(SizeHint::exact(1), 3));
///
/// let values: Vec<_> = error.into_iter().collect();
///
/// assert_eq!(values, vec![3, 2, 1]);
/// ```
impl<I: Iterator, C> CollectError<I, C, CapacityError<I::Item>> {
    /// Creates a new [`Bounds`](CapacityErrorKind::Bounds) [`CollectError`] for iterators whose
    /// [`size_hint`](Iterator::size_hint) is incompatible with the specified capacity.
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
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::<_, Vec<()>, _>::bounds(1..=3, SizeHint::exact(2));
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    #[must_use]
    pub fn bounds(iterator: I, capacity: SizeHint) -> Self
    where
        C: Default,
    {
        let hint = iterator.size_hint().try_into().expect("Invalid size hint");
        Self::new(iterator, C::default(), CapacityError::bounds(capacity, hint))
    }

    /// Ensures an empty collection's [fixed capacity](FixedCap::CAP) is compatible with `iterator`'s
    /// [`size_hint`][`Iterator::size_hint`].
    ///
    /// # Arguments
    ///
    /// * `iterator` - The iterator to check
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if the iterator's size hint
    /// is incompatible with the collection's fixed capacity. Otherwise returns the given `iterator`.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's size hint is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::<_, [i32; 5], _>::ensure_fits_in(1..=3)
    ///     .expect_err("Should fail bounds check");
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, [0, 0, 0, 0, 0]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(3) });
    /// ```
    pub fn ensure_fits_in(iterator: I) -> Result<I, Self>
    where
        C: Default + FixedCap,
    {
        match CapacityError::ensure_fits_in::<C, _>(&iterator) {
            Ok(()) => Ok(iterator),
            Err(error) => Err(Self::new(iterator, C::default(), error)),
        }
    }

    /// Ensures `collection`'s [`remaining_cap`](RemainingCap::remaining_cap) is compatible with
    /// the `iterator`'s [`Iterator::size_hint`].
    ///
    /// # Arguments
    ///
    /// * `iterator` - The iterator to check
    /// * `collection` - The collection to check - also determines the remaining capacity
    ///
    /// # Errors
    ///
    /// Returns a [`Bounds`](CapacityErrorKind::Bounds) [`CapacityError`] if the iterator's size
    /// hint indicates that the iterator cannot fit within the `collection`'s
    /// [`remaining_cap`](RemainingCap::remaining_cap). Otherwise returns the given `iterator`.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's size hint is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// # use arrayvec::ArrayVec;
    /// let array = ArrayVec::<i32, 5>::new();
    /// let error = CollectError::ensure_fits_into(1..=6, &array)
    ///     .expect_err("Should fail bounds check");
    ///
    /// assert_eq!(error.iterator, 1..=6);
    /// assert_eq!(error.collected, ArrayVec::<i32, 5>::new());
    /// assert_eq!(error.error.capacity, SizeHint::at_most(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(6) });
    /// ```
    pub fn ensure_fits_into(iterator: I, collection: &C) -> Result<I, Self>
    where
        C: RemainingCap + Default,
    {
        match CapacityError::ensure_fits_into(&iterator, collection) {
            Ok(()) => Ok(iterator),
            Err(error) => Err(Self::new(iterator, C::default(), error)),
        }
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`] for collection
    /// failures when `iterator` produced more items than `capacity`.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `rejected` - The overflow item that was consumed but couldn't be added to the collection
    /// * `capacity` - The allowed capacity range for the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::overflow(1..=3, vec![1, 2], 3, SizeHint::exact(2));
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { rejected: 3 });
    /// ```
    #[must_use]
    pub fn overflow(iterator: I, collected: C, rejected: I::Item, capacity: SizeHint) -> Self {
        Self::new(iterator, collected, CapacityError::overflow(capacity, rejected))
    }

    /// Creates a new [`Overflow`](CapacityErrorKind::Overflow) [`CollectError`] for
    /// collection failures when `iterator` produced more items than the
    /// [fixed capacity](FixedCap::CAP) of an empty collection of type `C`.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `rejected` - The overflow item that was consumed but couldn't be added to the collection
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::collect_overflowed(1..=3, [1, 2], 3);
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, [1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { rejected: 3 });
    /// ```
    #[must_use]
    pub fn collect_overflowed(iterator: I, collected: C, rejected: I::Item) -> Self
    where
        C: FixedCap,
    {
        Self::overflow(iterator, collected, rejected, C::CAP)
    }

    /// Creates a new [`CollectError`] for failed extension operations that filled the target
    /// container to capacity, but more items are still available.
    ///
    /// In this case, [`CapacityError::capacity`] is [`SizeHint::ZERO`], and a [`Default`]ed
    /// collection is used for [`CollectError::collected`], since all values are collected into
    /// the target container, which is left in a full state.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after overflow occurred
    /// * `rejected` - The item that was consumed but couldn't be added to the collection
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::<_, Vec<()>, _>::extend_overflowed(1..=3, 3);
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![]);
    /// assert_eq!(error.error.capacity, SizeHint::ZERO);
    /// assert_eq!(error.error.kind, CapacityErrorKind::Overflow { rejected: 3 });
    /// ```
    #[must_use]
    pub fn extend_overflowed(iterator: I, rejected: I::Item) -> Self
    where
        C: Default,
    {
        Self::new(iterator, C::default(), CapacityError::extend_overflowed(rejected))
    }

    /// Creates a new [`Underflow`](CapacityErrorKind::Underflow) [`CollectError`] for
    /// collection failures when `iterator` produced fewer items than `capacity`.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after underflow occurred
    /// * `collected` - The values that were collected before underflow,
    ///   also provides the count of collected items
    /// * `capacity` - The allowed capacity range for the collection
    ///
    /// Note: Due to limitations of rust's type inference, it may be necessary to explicitly
    /// specify the type of `collected` when using this method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::<_, Vec<_>, _>::underflow(1..=3, vec![1, 2], SizeHint::exact(2));
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, vec![1, 2]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(2));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    pub fn underflow(iterator: I, collected: C, capacity: SizeHint) -> Self
    where
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        let count = (&collected).into_iter().len();
        Self::new(iterator, collected, CapacityError::underflow(capacity, count))
    }

    /// Creates a new [`Underflow`](CapacityErrorKind::Underflow) [`CollectError`] for
    /// collection failures when `iterator` produced fewer items than the
    /// [fixed capacity](FixedCap::CAP) of the `collected` type.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after underflow occurred
    /// * `collected` - The values that were collected before underflow,
    ///   also provides the count of collected items
    ///
    /// Note: Due to limitations of rust's type inference, it may be necessary to explicitly
    /// specify the type of `collected` when using this method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collect_failable::errors::{CapacityError, CollectError, CapacityErrorKind, SizeHint};
    /// let error = CollectError::<_, [i32; 5], _>::collect_underflowed(1..=3, [1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(error.iterator, 1..=3);
    /// assert_eq!(error.collected, [1, 2, 3, 4, 5]);
    /// assert_eq!(error.error.capacity, SizeHint::exact(5));
    /// assert_eq!(error.error.kind, CapacityErrorKind::Underflow { count: 5 });
    /// ```
    #[must_use]
    pub fn collect_underflowed(iterator: I, collected: C) -> Self
    where
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
        C: FixedCap,
    {
        Self::underflow(iterator, collected, C::CAP)
    }
}

/// Specialization of [`CollectError`] for [`Collision`].
///
/// This type is used when collection fails due to a collision. Such as a duplicate key in a map or set.
/// The [`Collision`] will contain the item that collided during collection.
///
/// Since [`Collision`] implements [`ErrorItemProvider`], rejected items can be recovered,
/// allowing the original iterator values to be reconstructed from [`CollectError::iterator`],
/// [`CollectError::collected`], and [`CollectError::error`], though the order may differ.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator that was used to collect the values.
/// - `C`: The type of the collection that was used to collect the values.
///
/// # Examples
///
/// ```rust
/// use collect_failable::errors::{CollectError, Collision};
/// use std::collections::HashSet;
///
/// let error = CollectError::<_, HashSet<_>, _>::collision(1..=1, HashSet::from([1]), 1);
///
/// let values = error.into_iter().collect::<Vec<_>>();
///
/// assert_eq!(values.len(), 3, "Should have 3 values");
/// assert!(values.iter().all(|v| v == &1), "Should only contain 1");
/// ```
impl<I: Iterator, C> CollectError<I, C, Collision<I::Item>> {
    /// Creates a new [`CollectError`] with a [`Collision`] error, for collection failures
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
    /// use collect_failable::errors::{CollectError, Collision};
    /// use std::collections::HashSet;
    ///
    /// let error = CollectError::<_, HashSet<_>, _>::collision(1..=3, HashSet::from([1, 2]), 3);
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

impl<I, C, E> IntoIterator for CollectError<I, C, E>
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

impl<I, C, E: Display> Display for CollectError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Collection Error: {}", self.error)
    }
}

impl<I, C, E: Error + 'static> Error for CollectError<I, C, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<I, C, E: Debug> Debug for CollectError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CollectError")
            .field("collected", &core::any::type_name::<C>())
            .field("error", &self.error)
            .field("iterator", &core::any::type_name::<I>())
            .finish()
    }
}
