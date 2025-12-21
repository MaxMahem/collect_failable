use super::collection_tests::try_collect;

use crate::utils::FixedSizeHint;
use collect_failable::errors::CapacityMismatch;
use collect_failable::TryFromIterator;

const TOO_SHORT_HINT_ERR: CapacityMismatch = CapacityMismatch::bounds(2..=2, (1, Some(1)));
const TOO_LONG_HINT_ERR: CapacityMismatch = CapacityMismatch::bounds(2..=2, (3, Some(3)));
const TOO_SHORT_HIDDEN_ERR: CapacityMismatch = CapacityMismatch::underflow(2..=2, 1);
const TOO_LONG_HIDDEN_ERR: CapacityMismatch = CapacityMismatch::overflow(2..=2);

try_collect!(valid_array, [u32; 2], 1..=2, Ok([1, 2]));
try_collect!(too_long_data, [u32; 2], 1..=3, Err(TOO_LONG_HINT_ERR));
try_collect!(too_short_data, [u32; 2], 1..=1, Err(TOO_SHORT_HINT_ERR));
try_collect!(too_long_data_hidden, [u32; 2], FixedSizeHint::hide_size(1..=3), Err(TOO_LONG_HIDDEN_ERR));
try_collect!(too_short_data_hidden, [u32; 2], FixedSizeHint::hide_size(1..=1), Err(TOO_SHORT_HIDDEN_ERR));

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
try_collect!(non_fused_err, [i32; 2], BadIter(0), Err(TOO_LONG_HIDDEN_ERR));
