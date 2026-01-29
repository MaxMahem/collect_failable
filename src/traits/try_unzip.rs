use fluent_result::into::IntoResult;
use no_drop::dbg::IntoNoDrop;
use tap::Pipe;

use crate::TryExtendOne;
use crate::errors::UnzipError;
use crate::errors::types::Either;

/// Extends [`Iterator`] with a failable unzip method.
///
/// This is similar to [`Iterator::unzip`], consumes an [Iterator] of pairs and extends two containers
/// with the elements of the pairs, element by element, in parallel. The created containers may be of
/// different types.
#[sealed::sealed]
pub trait TryUnzip {
    /// Tries to unzip the iterator into two collections.
    ///
    /// Both containers are extended, element by element, in parallel.
    ///
    /// # Type Parameters
    ///
    /// * `FromA`: The type of the first container.
    /// * `FromB`: The type of the second container.
    ///
    /// # Errors
    ///
    /// Returns an [`UnzipError`] wrapped in [`Either`] if either of the underlying
    /// collections fail to extend. The error preserves the partially constructed
    /// collection from the other side, along with the remaining unprocessed iterator.
    ///
    /// # Examples
    ///
    /// Different types of containers can be unzipped into.
    ///
    /// ```rust
    /// # use collect_failable::TryUnzip;
    /// # use std::collections::{BTreeSet, HashSet};
    /// let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    /// let (nums, chars): (BTreeSet<_>, HashSet<_>) = pairs.into_iter()
    ///     .try_unzip()
    ///     .expect("should succeed");
    ///
    /// assert_eq!(nums, BTreeSet::from([1, 2, 3]), "should contain all items");
    /// assert_eq!(chars, HashSet::from(['a', 'b', 'c']), "should contain all items");
    /// ```
    ///
    /// ## Error Recovery
    ///
    /// When an error occurs, the error contains the partially constructed collection from the
    /// partial side, allowing for recovery or reconstruction of the original iterator.
    ///
    /// ```rust
    /// # use collect_failable::TryUnzip;
    /// # use collect_failable::errors::collision::Collision;
    /// # use std::collections::HashSet;
    /// let err = vec![(1, 'a'), (1, 'c'), (3, 'd')]
    ///     .into_iter()
    ///     .try_unzip::<HashSet<_>, HashSet<_>>()
    ///     .expect_err("Should fail due to collision")
    ///     .left()
    ///     .expect("Should fail on number side");
    ///
    /// assert_eq!(err.error, Collision { item: 1 }, "should have error from number side");
    /// assert_eq!(err.failed, HashSet::from([1]), "should have partially constructed data");
    /// assert_eq!(err.partial, HashSet::from(['a']), "should have partially constructed data");
    /// assert_eq!(err.pending, Some('c'), "should have pending data from the error pair");
    /// assert_eq!(
    ///     err.into_data().remaining.collect::<Vec<_>>(),
    ///     vec![(3, 'd')],
    ///     "should have remaining items"
    /// );
    /// ```
    fn try_unzip<FromA, FromB>(self) -> UnzipResult<FromA, FromB, Self>
    where
        FromA: Default + TryExtendOne,
        FromB: Default + TryExtendOne,
        Self: Iterator<Item = (FromA::Item, FromB::Item)> + Sized;
}

/// Type alias for the result of [`TryUnzip::try_unzip`].
pub type UnzipResult<FromA, FromB, I> = Result<(FromA, FromB), Either<UnzipError<FromA, FromB, I>, UnzipError<FromB, FromA, I>>>;

type EitherUnzipError<FromA, FromB, I> = Either<UnzipError<FromA, FromB, I>, UnzipError<FromB, FromA, I>>;

#[sealed::sealed]
impl<I: Iterator> TryUnzip for I {
    fn try_unzip<FromA, FromB>(self) -> UnzipResult<FromA, FromB, Self>
    where
        FromA: Default + TryExtendOne,
        FromB: Default + TryExtendOne,
        Self: Iterator<Item = (FromA::Item, FromB::Item)> + Sized,
    {
        let mut from = (FromA::default().no_drop(), FromB::default().no_drop());
        let mut this = self.no_drop();

        for (a, b) in this.by_ref().map(|(a, b)| (a.no_drop(), b.no_drop())) {
            if let Err(error) = from.0.try_extend_one(a.unwrap()) {
                return UnzipError::<FromA, FromB, Self>::new(
                    error,
                    from.0.unwrap(),
                    from.1.unwrap(),
                    Some(b.unwrap()),
                    this.unwrap(),
                )
                .pipe(Either::Left)
                .into_err();
            }

            if let Err(error) = from.1.try_extend_one(b.unwrap()) {
                return UnzipError::<FromB, FromA, Self>::new(
                    error,
                    from.1.unwrap(),
                    from.0.unwrap(),
                    None::<FromA::Item>,
                    this.unwrap(),
                )
                .pipe::<EitherUnzipError<FromA, FromB, Self>>(Either::Right)
                .into_err();
            }
        }

        this.forget();
        (from.0.unwrap(), from.1.unwrap()).into_ok()
    }
}
