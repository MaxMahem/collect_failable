use crate::{OneOf2, TryExtend};

/// Extends [`Iterator`] with a failable unzip method.
///
/// This is similar to [`Iterator::unzip`], but allows for failable construction. The created
/// containers may be of different types, but both must implement [`Default`] and [`TryExtend`].
pub trait TryUnzip {
    /// Tries to unzip the iterator into two collections.
    ///
    /// Both containers are extended, element by element, in parallel.
    ///
    /// # Errors
    ///
    /// Returns a [`TryExtend::Error`] wrapped in a [`OneOf2`] if either of the underlying
    /// collections fail to extend.
    ///
    /// # Examples
    ///
    /// Different types of containers can be unzipped into.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/try-unzip.rs", try_unzip_different_containers_example, [])]
    /// ```
    ///
    /// ## Multiple Errors
    ///
    /// In the case of multiple possible failures, the error from extending `FromA` is checked and
    /// returned first.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/try-unzip.rs", try_unzip_collision_example, [])]
    /// ```
    fn try_unzip<A, B, FromA, FromB>(self) -> Result<(FromA, FromB), OneOf2<FromA::Error, FromB::Error>>
    where
        FromA: Default + TryExtend<A>,
        FromB: Default + TryExtend<B>,
        Self: Iterator<Item = (A, B)>;
}

/// Implemententation for any [`Iterator`].
impl<I> TryUnzip for I
where
    I: Iterator,
{
    fn try_unzip<A, B, FromA, FromB>(mut self) -> Result<(FromA, FromB), OneOf2<FromA::Error, FromB::Error>>
    where
        FromA: Default + TryExtend<A>,
        FromB: Default + TryExtend<B>,
        Self: Iterator<Item = (A, B)>,
    {
        self.try_fold((FromA::default(), FromB::default()), |mut tuple, (a, b)| {
            tuple.0.try_extend(std::iter::once(a)).map_err(OneOf2::A)?;
            tuple.1.try_extend(std::iter::once(b)).map_err(OneOf2::B)?;
            Ok(tuple)
        })
    }
}
