use collect_failable::utils::FixedSizeHint;
use collect_failable::{CapacityMismatch, TryFromIterator};

/// Unified macro that generates complete test functions for array creation
///
/// Supports both success and error cases:
/// - `array_test!(name, [Type; N], iter, Ok(expected_array))`
/// - `array_test!(name, [Type; N], iter, Err(expected_error))`
macro_rules! array_test {
    ($name:ident, $array_type:ty, $iter:expr, Ok($expected:expr)) => {
        #[test]
        fn $name() {
            let found = <$array_type>::try_from_iter($iter).expect("should be ok");
            assert_eq!(found, $expected, "should match expected array");
        }
    };

    ($name:ident, $array_type:ty, $iter:expr, Err($expected_error:expr)) => {
        #[test]
        fn $name() {
            let err = <$array_type>::try_from_iter($iter).expect_err("should be err");
            assert_eq!(err.error, $expected_error, "should match expected error");
        }
    };
}

const TOO_SHORT_HINT_ERR: CapacityMismatch = CapacityMismatch::bounds(2..=2, (1, Some(1)));
const TOO_LONG_HINT_ERR: CapacityMismatch = CapacityMismatch::bounds(2..=2, (3, Some(3)));
const TOO_SHORT_HIDDEN_ERR: CapacityMismatch = CapacityMismatch::underflow(2..=2, 1);
const TOO_LONG_HIDDEN_ERR: CapacityMismatch = CapacityMismatch::overflow(2..=2);

array_test!(valid_array, [u32; 2], 1..=2, Ok([1, 2]));
array_test!(too_long_data, [u32; 2], 1..=3, Err(TOO_LONG_HINT_ERR));
array_test!(too_short_data, [u32; 2], 1..=1, Err(TOO_SHORT_HINT_ERR));
array_test!(too_long_data_hidden, [u32; 2], FixedSizeHint::hide_size(1..=3), Err(TOO_LONG_HIDDEN_ERR));
array_test!(too_short_data_hidden, [u32; 2], FixedSizeHint::hide_size(1..=1), Err(TOO_SHORT_HIDDEN_ERR));

struct BadIter(usize);

// A non-fused iterator that can return Some after returning None
impl Iterator for BadIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        match self.0 {
            1 => Some(1),
            2 => None,
            3 => Some(2),
            _ => None,
        }
    }
}

array_test!(non_fused_ok, [i32; 1], BadIter(0), Ok([1]));

array_test!(non_fused_err, [i32; 2], BadIter(0), Err(TOO_LONG_HIDDEN_ERR));
