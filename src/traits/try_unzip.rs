use fluent_result::into::IntoResult;
use no_drop::dbg::IntoNoDrop;

use crate::errors::UnzipError;
use crate::TryExtendOne;

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
    fn try_unzip<FromA, FromB>(self) -> Result<(FromA, FromB), UnzipError<FromA, FromB, Self>>
    where
        FromA: Default + TryExtendOne,
        FromB: Default + TryExtendOne,
        Self: Iterator<Item = (FromA::Item, FromB::Item)> + Sized;
}

#[sealed::sealed]
impl<I: Iterator> TryUnzip for I {
    fn try_unzip<FromA, FromB>(self) -> Result<(FromA, FromB), UnzipError<FromA, FromB, Self>>
    where
        FromA: Default + TryExtendOne,
        FromB: Default + TryExtendOne,
        Self: Iterator<Item = (FromA::Item, FromB::Item)> + Sized,
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
