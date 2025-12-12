use std::fmt::Debug;

/// A [`Option<T>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
/// This avoids requiring `T: Debug` while still providing useful debug output.
pub(crate) struct ResultTypeDebug<'a, T, E>(pub &'a Result<T, E>);

impl<T, E: Debug> Debug for ResultTypeDebug<'_, T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Ok(_) => write!(f, "Ok({})", std::any::type_name::<T>()),
            Err(e) => write!(f, "Err({e:?})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_result_ok() {
        let result: Result<i32, i32> = Ok(42);
        let debug_output = format!("{:?}", ResultTypeDebug(&result));
        assert_eq!(debug_output, "Ok(i32)");
    }

    #[test]
    fn debug_result_err() {
        let result: Result<i32, i32> = Err(42);
        let debug_output = format!("{:?}", ResultTypeDebug(&result));
        assert_eq!(debug_output, "Err(42)");
    }
}
