use crate::collection_tests::panics;

use super::collection_tests::try_collect;

use collect_failable::errors::CapacityError;
use collect_failable::TryFromIterator;
use size_hinter::{InvalidIterator, SizeHint, SizeHinter};

// Note: CapacityError is now generic, so these need to be CapacityError<u32> or CapacityError<i32>
const TOO_SHORT_HINT_ERR: CapacityError<u32> = CapacityError::bounds(SizeHint::exact(2), SizeHint::exact(1));
const TOO_LONG_HINT_ERR: CapacityError<u32> = CapacityError::bounds(SizeHint::exact(2), SizeHint::exact(3));
const TOO_SHORT_HIDDEN_ERR: CapacityError<u32> = CapacityError::underflow(SizeHint::exact(2), 1);
// For overflow, we need to provide the rejected item, but in const context we can't.
// So we'll test  it differently

try_collect!(valid_array, [u32; 2], 1..=2, Ok([1, 2]));
try_collect!(too_long_data, [u32; 2], 1..=3, Err(TOO_LONG_HINT_ERR));
try_collect!(too_short_data, [u32; 2], 1..=1, Err(TOO_SHORT_HINT_ERR));
// Overflow requires a rejected item which can't be const, so just test manually
#[test]
fn too_long_data_hidden() {
    let result = <[u32; 2]>::try_from_iter((1..=3).hide_size());
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Verify it's an overflow error by checking the error message
    assert!(err.error.to_string().contains("exceeds capacity"));
}

try_collect!(too_short_data_hidden, [u32; 2], (1..=1).hide_size(), Err(TOO_SHORT_HIDDEN_ERR));

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

try_collect!(non_fused_ok, [i32; 1], BadIter(0), Ok([1]));
// Similar issue with overflow - test manually
#[test]
fn non_fused_err() {
    let result = <[i32; 2]>::try_from_iter(BadIter(0));
    assert!(result.is_err());
}

panics!(panic_on_invalid_iterator, <[(); 2]>::try_from_iter(InvalidIterator), "invalid size hint");
