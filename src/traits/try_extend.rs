#[cfg(doc)]
use crate::TryFromIterator;
#[cfg(doc)]
use std::collections::HashMap;

/// Trait for extending an existing collection from an iterator with fallible operations.
///
/// This trait is similar to [`Extend`], but allows implementor to uphold a containers invariant
/// during construction. This invaraint can be upheld in two ways:
///
/// - **Basic error guarantee**. On an error, the collection may be modified, but will be in a
///   valid state. [`TryExtend::try_extend`] provides this guarantee.
/// - **Strong error guarantee**. On an error, the collection is not modified.
///   [`TryExtend::try_extend_safe`] provides this guarantee.
///
/// Implementations may rely on [`Iterator::size_hint`] providing reliable bounds for the number of
/// elements in the iterator in order to optimize their implementations. An iterator that violates
/// the bounds returned by [`Iterator::size_hint`] may cause panics, produce incorrect results, or
/// produce a result that violates container constraints, but must not result in undefined behavior.
pub trait TryExtend<T> {
    /// Error type returned by the fallible extension methods.
    type Error;

    /// Tries to extends the collection providing a **strong error guarantee**.
    ///
    /// On failure, the collection must remain unchanged. Implementors may need to buffer
    /// elements or use a more defensive algorithm to satisfy this guarantee. If an implementation
    /// cannot provide this gurantee, this method should always return an error.
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

    /// Tries to extends the collection providing a **basic error guarantee**.
    ///
    /// On failure, the collection may be partially modified, but it must remain valid.
    /// The specific extension that triggers the error must not be inserted.
    ///
    /// For strong guarantee needs, use [`TryExtend::try_extend_safe`].
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
        I: IntoIterator<Item = T>,
    {
        self.try_extend_safe(iter)
    }
}
