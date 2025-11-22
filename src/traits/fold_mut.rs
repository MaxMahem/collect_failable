use include_doc::function_body;

#[cfg(doc)]
use std::collections::HashMap;

/// Extension trait for iterators that provides fold operations with mutable accumulator references.
///
/// This trait is similar to [`Iterator::fold`], but crucially differs in that the closure receives
/// a `&mut A` reference to the accumulator instead of consuming and returning it. This pattern may
/// be more ergonomic for accumulators that do not need to transfer ownership in order to
/// accumulate.
///
/// # Examples
///
/// Aggregating values into a hashamp with [`fold_mut`](FoldMut::fold_mut):
///
/// ```rust
#[doc = function_body!("tests/fold_mut.rs", fold_mut_success_aggregate_hashmap_example, [])]
/// ```
///
/// Building a [`HashMap`] terminating on colliding keys using [`try_fold_mut`](FoldMut::try_fold_mut).
///
/// ```rust
#[doc = function_body!("tests/fold_mut.rs", try_fold_mut_failure_example, [])]
/// ```
pub trait FoldMut<T>: Iterator<Item = T> {
    /// Folds iterator items into an accumulator using a mutable reference.
    ///
    /// Unlike [`Iterator::fold`], this method passes a mutable reference to the
    /// accumulator instead of consuming and returning it each iteration.
    ///
    /// # Parameters
    ///
    /// - `init`: The initial value of the accumulator
    /// - `fold`: A closure that mutates the accumulator and may return an error
    ///
    /// # Returns
    ///
    /// The final accumulated value after processing all items.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = function_body!("tests/fold_mut.rs", fold_mut_success_aggregate_hashmap_example, [])]
    /// ```
    fn fold_mut<A, F>(&mut self, mut init: A, mut fold: F) -> A
    where
        F: FnMut(&mut A, T),
    {
        self.for_each(|item| fold(&mut init, item));
        init
    }

    /// Folds iterator items into an accumulator with fallible operations.
    ///
    /// Similar to [`fold_mut`](FoldMut::fold_mut), but the closure can return a [`Result`] to
    /// signal early termination. If any closure invocation returns [`Err`], iteration stops
    /// immediately and the error is propagated.
    ///
    /// # Parameters
    ///
    /// - `init`: The initial value of the accumulator
    /// - `fold`: A closure that mutates the accumulator and may return an error
    ///
    /// # Errors
    ///
    /// Returns the first error encountered when the closure returns [`Err`] for any item.
    ///
    /// # Returns
    ///
    /// - [`Ok(A)`](Ok): The final accumulated value if all items were processed successfully
    /// - [`Err(E)`](Err): The first error encountered during iteration
    ///
    /// # Examples    
    ///
    /// Building a [`HashMap`] terminating on colliding keys.
    ///
    /// ```rust
    #[doc = function_body!("tests/fold_mut.rs", try_fold_mut_failure_example, [])]
    /// ```
    fn try_fold_mut<A, E, F>(&mut self, mut init: A, mut fold: F) -> Result<A, E>
    where
        Self: Sized,
        F: FnMut(&mut A, T) -> Result<(), E>,
    {
        self.try_for_each(|item| fold(&mut init, item))?;
        Ok(init)
    }
}

impl<I, T> FoldMut<T> for I where I: Iterator<Item = T> {}
