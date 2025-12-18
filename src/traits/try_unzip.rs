use fluent_result::into::IntoResult;
use no_drop::dbg::IntoNoDrop;

use crate::{TryExtendOne, UnzipError};

/// The Result of a [`TryUnzip::try_unzip`] operation.
type TryUnzipResult<A, B, FromA, FromB, I> = Result<(FromA, FromB), UnzipError<A, B, FromA, FromB, I>>;

/// Extends [`Iterator`] with a failable unzip method.
///
/// This is similar to [`Iterator::unzip`], but allows for failable construction. The created
/// containers may be of different types, but both must implement [`Default`] and [`TryExtendOne`].
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
        FromA: Default + TryExtendOne<A> + IntoIterator<Item = A>,
        FromB: Default + TryExtendOne<B> + IntoIterator<Item = B>,
        Self: Iterator<Item = (A, B)> + Sized;
}

#[sealed::sealed]
impl<I: Iterator> TryUnzip for I {
    fn try_unzip<A, B, FromA, FromB>(self) -> TryUnzipResult<A, B, FromA, FromB, Self>
    where
        FromA: Default + TryExtendOne<A> + IntoIterator<Item = A>,
        FromB: Default + TryExtendOne<B> + IntoIterator<Item = B>,
        Self: Iterator<Item = (A, B)> + Sized,
    {
        let mut from = (FromA::default().no_drop(), FromB::default().no_drop());
        let mut this = self.no_drop();

        for (a, b) in this.by_ref().map(|(a, b)| (a.no_drop(), b.no_drop())) {
            if let Err(error_a) = from.0.try_extend_one(a.unwrap()) {
                return UnzipError::new_a(error_a, from.0.unwrap(), from.1.unwrap(), Some(b.unwrap()), this.unwrap()).into_err();
            }

            if let Err(error_b) = from.1.try_extend_one(b.unwrap()) {
                return UnzipError::new_b(error_b, from.1.unwrap(), from.0.unwrap(), None, this.unwrap()).into_err();
            }
        }

        this.forget();
        (from.0.unwrap(), from.1.unwrap()).into_ok()
    }
}
