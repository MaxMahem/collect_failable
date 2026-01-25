use crate::TryExtendOne;

/// Extends a collection via [`TryExtendOne::try_extend_one`], erroring if any call fails.
pub fn try_extend_basic<T, C, I>(map: &mut C, iter: &mut I) -> Result<(), C::Error>
where
    I: Iterator<Item = T>,
    C: TryExtendOne<Item = T>,
{
    iter.try_for_each(|item| map.try_extend_one(item))
}

/// Implements [`TryFromIterator`] via [`TryExtendOne::try_extend_one`].
///
/// Useful for collections that can be constructed empty and extended element
/// by element, and where a failure during [`TryExtendOne::try_extend_one`]
/// indicates the collection failed construction.
///
/// Failed collections return a [`CollectionError`] with the original iterator,
/// the partially constructed collection, and the error returned by
/// [`TryExtendOne::try_extend_one`].
///
/// ```text
/// impl_try_from_iter_via_try_extend_one!(
///     type: $type where [$($generics)*] of $item;
///     ctor: $ctor
/// );
/// ```
///
/// # Arguments
///
/// - `type`: The type to implement [`TryFromIterator`] for.
/// - `generics`: The generics for the type.
/// - `item`: The item type.
/// - `ctor`: A function that creates a new collection from an iterator.
///   - `fn(&I::IntoIter) -> Self`
macro_rules! impl_try_from_iter_via_try_extend_one {
    (
        type: $type:ty where [$($generics:tt)*] of $item:ty;
        ctor: $ctor:expr
    ) => {
        impl<$($generics)*, I> $crate::TryFromIterator<I> for $type
        where
            I: IntoIterator<Item = $item>,
        {
            type Error = $crate::errors::CollectionError<I::IntoIter, Self, <Self as $crate::TryExtendOne>::Error>;

            fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
            where
                Self: Sized,
            {
                let mut iter = into_iter.into_iter();

                let mut collection = $crate::impls::macros::infer_ctor::<I, Self, _>($ctor)(&iter);

                match $crate::impls::macros::try_extend_basic(&mut collection, &mut iter) {
                    Ok(()) => Ok(collection),
                    Err(err) => Err($crate::errors::CollectionError::new(iter, collection, err)),
                }
            }
        }
    };
}

/// Helper function to infer the type of the ctor function.
pub const fn infer_ctor<I: IntoIterator, C, F: Fn(&I::IntoIter) -> C>(f: F) -> F {
    f
}

/// Implements [`TryExtend`] via [`TryExtendOne::try_extend_one`].
///
/// Useful for collections that can be extended element by element, and where
/// a failure during [`TryExtendOne::try_extend_one`] indicates the collection
/// failed extension.
///
/// Failed collections return a [`CollectionError`] with the original iterator,
/// an empty collection, and the error returned by [`TryExtendOne::try_extend_one`].
/// Since [`TryExtend`] provides only a basic error guarantee, the collection
/// will be mutated in the event of a failure.
///
/// ```text
/// impl_try_extend_via_try_extend_one!(
///     type: $type where [$($generics)*] of $item;
///     reserve: $reserve;
///     build_empty: $build_empty
/// );
/// ```
///
/// # Arguments
///
/// - `type`: The type to implement [`TryExtend`] for.
/// - `generics`: The generics for the type.
/// - `item`: The item type.
/// - `reserve`: A function that reserves space in the collection.
///   - `fn(&mut Self, &I::IntoIter)`
/// - `build_empty`: A function that builds an empty collection.
///   - `fn(&mut Self) -> Self`
macro_rules! impl_try_extend_via_try_extend_one {
    (
        type: $type:ty where [$($generics:tt)*] of $item:ty;
        reserve: $reserve:expr;
        build_empty: $build_empty:expr
    ) => {
        impl<$($generics)*, I> $crate::TryExtend<I> for $type
        where
            I: IntoIterator<Item = $item>,
        {
            type Error = $crate::errors::CollectionError<I::IntoIter, Self, <Self as $crate::TryExtendOne>::Error>;

            fn try_extend(&mut self, into_iter: I) -> Result<(), Self::Error>
            where
                Self: Sized,
            {
                let mut iter = into_iter.into_iter();

                $crate::impls::macros::infer_reserve::<I, Self, _>($reserve)(self, &mut iter);

                $crate::impls::macros::try_extend_basic(self, &mut iter)
                    .map_err(|err| $crate::errors::CollectionError::new(
                        iter,
                        $crate::impls::macros::infer_build_empty::<Self, _>($build_empty)(self),
                        err
                    ))
            }
        }
    };
}

/// Helper function to infer the type of the reserve function.
pub const fn infer_reserve<I: IntoIterator, C, F: Fn(&mut C, &I::IntoIter)>(f: F) -> F {
    f
}

/// Helper function to infer the type of the `build_empty` function.
pub const fn infer_build_empty<C, F: Fn(&mut C) -> C>(f: F) -> F {
    f
}

/// Implements [`TryExtendSafe`] for types that cannot contain colliding items.
///
/// ```text
/// impl_try_extend_safe_for_colliding_type!(
///     type: $type where [$($generics)*] of $item:ty;
///     build_staging: $build_staging;
///     contains: $contains
/// );
/// ```
///
/// # Arguments
///
/// * `type`: The type to implement [`TryExtendSafe`] for.
/// * `generics`: The generics for the type.
/// * `item`: The item type.
/// * `build_staging`: A function that builds an empty collection.
///   - `fn(iter: &I::IntoIter, set: &Self) -> Self`
/// * `contains`: A function that checks if the collection contains an item.
///   - `fn(&Self, &$item) -> bool`
macro_rules! impl_try_extend_safe_for_colliding_type {
    (
        type: $type:ty where [$($generics:tt)*] of $item:ty;
        build_staging: $build_staging:expr;
        contains: $contains:expr
    ) => {
        impl<$($generics)*, I> $crate::TryExtendSafe<I> for $type
        where
            I: IntoIterator<Item = $item>,
        {
            fn try_extend_safe(&mut self, iter: I) -> Result<(), Self::Error> {
                let mut iter = iter.into_iter();

                let staging = $crate::impls::macros::infer_build_staging::<Self, I, _>($build_staging)(&iter, self);

                let contains = $crate::impls::macros::infer_contains::<Self, $item, _>($contains);

                iter.try_fold(staging, |mut staging, item| {
                    // check for an entry in the main map
                    match contains(self, &item) {
                        true => Err((staging, $crate::errors::Collision::new(item))),
                        // try to add to the staging map
                        false => match $crate::TryExtendOne::try_extend_one(&mut staging, item) {
                            Ok(()) => Ok(staging),
                            Err(err) => Err((staging, err)),
                        },
                    }
                })
                .map(|staging| ::core::iter::Extend::extend(self, staging))
                .map_err(|(staging, err)| $crate::errors::CollectionError::new(iter, staging, err))
            }
        }
    };
}

/// Helper function to infer the type of the `build_staging` function.
pub const fn infer_build_staging<C, I: IntoIterator, F: Fn(&I::IntoIter, &mut C) -> C>(f: F) -> F {
    f
}

/// Helper function to infer the type of the contains function.
pub const fn infer_contains<C, T, F: Fn(&C, &T) -> bool>(f: F) -> F {
    f
}

/// Implements [`TryExtendOne`] for types that cannot contain colliding items.
///
/// ```text
/// impl_try_extend_one_for_colliding_type(
///     type: $type where [$generics] of $item;
///     contains: $contains;
///     insert: $insert
/// );
/// ```
///
/// # Arguments
///
/// * `type`: The type to implement [`TryExtendOne`] for.
/// * `generics`: The generics for the type.
/// * `item`: The item type.
/// * `contains`: A function that checks if the collection contains an item.
///   - `fn(&Self, &$item) -> bool`
/// * `insert`: A function that inserts an item into the collection.
///   - `fn(&mut Self, $item)`
macro_rules! impl_try_extend_one_for_colliding_type {
    (
        type: $type:ty where [$($generics:tt)*] of $item:ty;
        contains: $contains:expr;
        insert: $insert:expr
    ) => {
        impl<$($generics)*> $crate::TryExtendOne for $type {
            type Item = $item;
            type Error = $crate::errors::Collision<$item>;

            fn try_extend_one(&mut self, item: Self::Item) -> Result<(), Self::Error> {

                match $crate::impls::macros::infer_contains::<Self, $item, _>($contains)(self, &item) {
                    true => Err($crate::errors::Collision::new(item)),
                    false => {
                        $crate::impls::macros::infer_insert::<Self, $item, _>($insert)(self, item);
                        Ok(())
                    }
                }
            }
        }
    };
}

pub const fn infer_insert<C, T, F: Fn(&mut C, T)>(f: F) -> F {
    f
}

pub(crate) use impl_try_extend_one_for_colliding_type;
pub(crate) use impl_try_extend_safe_for_colliding_type;
pub(crate) use impl_try_extend_via_try_extend_one;
pub(crate) use impl_try_from_iter_via_try_extend_one;
