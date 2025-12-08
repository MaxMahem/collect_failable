/// A [`Option<T>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
/// This avoids requiring `T: Debug` while still providing useful debug output.
pub(crate) struct OptionTypeDebug<'a, T>(pub &'a Option<T>);

impl<T> std::fmt::Debug for OptionTypeDebug<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(_) => write!(f, "Some({})", std::any::type_name::<T>()),
            None => f.write_str("None"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_option_none() {
        let opt: Option<i32> = None;
        let debug_output = format!("{:?}", OptionTypeDebug(&opt));
        assert_eq!(debug_output, "None");
    }

    #[test]
    fn debug_option_some() {
        let opt = Some(42);
        let debug_output = format!("{:?}", OptionTypeDebug(&opt));
        assert_eq!(debug_output, "Some(i32)");
    }
}
