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
/// ```text
/// impl_try_from_iter_via_try_extend_one!(
///     type: $type where [$($generics)*] of $item;
///     ctor: $ctor
/// );
/// ```
///
/// # Arguments
///
/// * `type`: The type to implement [`TryFromIterator`] for.
/// * `generics`: The generics for the type.
/// * `item`: The item type.
/// * `ctor`: A function that creates a new collection from an iterator.
macro_rules! impl_try_from_iter_via_try_extend_one {
    (
        type: $type:ty where [$($generics:tt)*] of $item:ty;
        ctor: $ctor:expr
    ) => {
        impl<$($generics)*, I> $crate::TryFromIterator<I> for $type
        where
            I: IntoIterator<Item = $item>,
        {
            type Error = $crate::errors::CollectionError<I::IntoIter, Self, $crate::errors::Collision<$item>>;

            fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>
            where
                Self: Sized,
            {
                let mut iter = into_iter.into_iter();
                let new_fn: fn(&I::IntoIter) -> Self = $ctor;
                let mut collection = new_fn(&iter);

                match $crate::impls::macros::try_extend_basic(&mut collection, &mut iter) {
                    Ok(()) => Ok(collection),
                    Err(err) => Err($crate::errors::CollectionError::new(iter, collection, err)),
                }
            }
        }
    };
}

/// Implements [`TryExtend`] via [`TryExtendOne::try_extend_one`].
///
/// ```text
/// impl_try_extend_via_try_extend_one!(
///     type: $type where [$($generics)*] of $item;
///     reserve: $reserve;
///     build_empty_collection: $build_empty_collection
/// );
/// ```
///
/// # Arguments
///
/// * `type`: The type to implement [`TryExtend`] for.
/// * `generics`: The generics for the type.
/// * `item`: The item type.
/// * `reserve`: A function that reserves space in the collection.
/// * `build_empty_collection`: A function that builds an empty collection.
macro_rules! impl_try_extend_via_try_extend_one {
    (
        type: $type:ty where [$($generics:tt)*] of $item:ty;
        reserve: $reserve:expr;
        build_empty_collection: $build_empty_collection:expr
    ) => {
        impl<$($generics)*, I> $crate::TryExtend<I> for $type
        where
            I: IntoIterator<Item = $item>,
        {
            type Error = $crate::errors::CollectionError<I::IntoIter, Self, $crate::errors::Collision<$item>>;

            fn try_extend(&mut self, into_iter: I) -> Result<(), Self::Error>
            where
                Self: Sized,
            {
                let mut iter = into_iter.into_iter();
                let reserve: fn(&I::IntoIter, &mut Self) = $reserve;
                reserve(&mut iter, self);

                $crate::impls::macros::try_extend_basic(self, &mut iter).map_err(|err| $crate::errors::CollectionError::new(iter, $build_empty_collection(self), err))
            }
        }
    };
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
/// * `contains`: A function that checks if the collection contains an item.
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
                let build_staging_fn: fn(&mut Self, iter: &I::IntoIter) -> Self = $build_staging;
                let contains: fn(&Self, &$item) -> bool = $contains;

                iter.try_fold(build_staging_fn(self, &iter), |mut staging, item| {
                    match contains(self, &item) {
                        true => Err((staging, $crate::errors::Collision::new(item))),
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
/// * `insert`: A function that inserts an item into the collection.
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
                let contains: fn(&Self, &$item) -> bool = $contains;
                match contains(self, &item) {
                    true => Err($crate::errors::Collision::new(item)),
                    false => {
                        let insert: fn(&mut Self, $item) = $insert;
                        insert(self, item);
                        Ok(())
                    }
                }
            }
        }
    };
}

pub(crate) use impl_try_extend_one_for_colliding_type;
pub(crate) use impl_try_extend_safe_for_colliding_type;
pub(crate) use impl_try_extend_via_try_extend_one;
pub(crate) use impl_try_from_iter_via_try_extend_one;
