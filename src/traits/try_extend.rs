#[cfg(doc)]
use crate::TryFromIterator;
#[cfg(doc)]
use std::collections::HashMap;

/// Trait for extending an existing collection from an iterator with fallible operations.
///
/// This trait is similar to [`Extend`], but allows implementor to uphold a container's invariant
/// during construction with a **basic error guarantee**. On an error, the collection may be
/// modified, but will be in a valid state. The specific extension that triggers the error must
/// not be inserted.
///
/// For a stronger error guarantee where the collection is unchanged on error, see
/// [`TryExtendSafe`].
///
/// Implementations may rely on [`Iterator::size_hint`] providing reliable bounds for the number of
/// elements in the iterator in order to optimize their implementations. An iterator that violates
/// the bounds returned by [`Iterator::size_hint`] may cause panics, produce incorrect results, or
/// produce a result that violates container constraints, but must not result in undefined behavior.
pub trait TryExtend<T> {
    /// Error type returned by the fallible extension methods.
    type Error;

    /// Tries to extends the collection providing a **basic error guarantee**.
    ///
    /// On failure, the collection may be partially modified, but it must remain valid.
    /// The specific extension that triggers the error must not be inserted.
    ///
    /// For strong guarantee needs, see [`TryExtendSafe::try_extend_safe`].
    ///
    /// # Errors
    ///
    /// Returns [`TryExtend::Error`] if a failure occurs while extending the collection.
    ///
    /// # Examples
    ///
    /// The provided [`HashMap`] implementation errors if a key collision occurs during extension.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/try-extend.rs", try_extend_basic_guarantee_example, [])]
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>;
}

/// Trait for extending a collection with a **strong error guarantee**.
///
/// This trait extends [`TryExtend`] by providing a method that guarantees the collection
/// remains unchanged if an error occurs during extension.
///
/// Not all types can implement this trait. For example, tuples of collections cannot
/// provide this guarantee because if the second collection fails to extend, the first
/// may have already been modified.
///
/// Like with [`TryExtend`], implementors may rely on [`Iterator::size_hint`] providing reliable
/// bounds for the number of elements in the iterator in order to optimize their implementations.
/// An iterator that violates the bounds returned by [`Iterator::size_hint`] may cause panics,
/// produce incorrect results, or produce a result that violates container constraints, but must
/// not result in undefined behavior.
pub trait TryExtendSafe<T>: TryExtend<T> {
    /// Tries to extends the collection providing a **strong error guarantee**.
    ///
    /// On failure, the collection must remain unchanged. Implementors may need to buffer
    /// elements or use a more defensive algorithm to satisfy this guarantee.
    ///
    /// For a faster basic-guarantee alternative, see [`TryExtend::try_extend`].
    ///
    /// # Errors
    ///
    /// Returns [`TryExtend::Error`] if a failure occurs while extending the collection.
    ///
    /// # Examples
    ///
    /// The provided [`HashMap`] implementation errors if a key collision occurs during extension.
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/try-extend.rs", try_extend_safe_map_collision_example, [])]
    /// ```
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>;
}
