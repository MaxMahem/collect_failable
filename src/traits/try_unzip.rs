use std::iter::Once;

use no_drop::{dbg::Consume, dbg::IntoNoDrop};

use crate::{TryExtend, UnzipError};

/// The Result of a [`TryUnzip::try_unzip`] operation.
type TryUnzipResult<A, B, FromA, FromB, I> = Result<(FromA, FromB), UnzipError<A, B, FromA, FromB, I>>;

/// Extends [`Iterator`] with a failable unzip method.
///
/// This is similar to [`Iterator::unzip`], but allows for failable construction. The created
/// containers may be of different types, but both must implement [`Default`] and [`TryExtend`].
#[sealed::sealed]
pub trait TryUnzip {
    /// Tries to unzip the iterator into two collections.
    ///
    /// Both containers are extended, element by element, in parallel.
    ///
    /// # Errors
    ///
    /// Returns an [`UnzipError`] if either of the underlying collections fail to extend. The error
    /// preserves the partially constructed collection from the other side, along with the remaining
    /// unprocessed iterator.
    ///
    /// # Examples
    ///
    /// Different types of containers can be unzipped into.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/try_unzip.rs", try_unzip_different_containers_example, [])]
    /// ```
    ///
    /// ## Error Recovery
    ///
    /// When an error occurs, the error contains the partially constructed collection from the
    /// successful side, allowing for recovery or reconstruction of the original iterator.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/try_unzip.rs", try_unzip_collision_example, [])]
    /// ```
    fn try_unzip<A, B, FromA, FromB>(self) -> TryUnzipResult<A, B, FromA, FromB, Self>
    where
        FromA: Default + TryExtend<A, Once<A>> + IntoIterator<Item = A>,
        FromB: Default + TryExtend<B, Once<B>> + IntoIterator<Item = B>,
        Self: Iterator<Item = (A, B)> + Sized;
}

#[sealed::sealed]
impl<I> TryUnzip for I
where
    I: Iterator,
{
    fn try_unzip<A, B, FromA, FromB>(mut self) -> TryUnzipResult<A, B, FromA, FromB, Self>
    where
        FromA: Default + TryExtend<A, Once<A>> + IntoIterator<Item = A>,
        FromB: Default + TryExtend<B, Once<B>> + IntoIterator<Item = B>,
        Self: Iterator<Item = (A, B)> + Sized,
    {
        let mut a_collection = FromA::default();
        let mut b_collection = FromB::default();

        while let Some((a, b)) = self.next() {
            let a = a.no_drop();
            let b = b.no_drop();

            if let Err(error_a) = a_collection.try_extend(std::iter::once(a.consume())) {
                return Err(UnzipError::new_a(error_a, b_collection, Some(b.consume()), self));
            }

            if let Err(error_b) = b_collection.try_extend(std::iter::once(b.consume())) {
                return Err(UnzipError::new_b(error_b, a_collection, None, self));
            }
        }

        Ok((a_collection, b_collection))
    }
}
