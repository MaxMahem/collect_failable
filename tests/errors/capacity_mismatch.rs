use collect_failable::errors::CapacityMismatch;
use size_hinter::SizeHint;

use crate::error_tests::identity;

const BOUNDS_ERROR: CapacityMismatch = CapacityMismatch::bounds(SizeHint::bounded(5, 10), SizeHint::bounded(11, 15));
const OVERFLOW_ERROR: CapacityMismatch = CapacityMismatch::overflow(SizeHint::bounded(5, 10));
const UNDERFLOW_ERROR: CapacityMismatch = CapacityMismatch::underflow(SizeHint::bounded(5, 10), 3);

identity!(bounds_equality, CapacityMismatch::bounds(SizeHint::bounded(5, 10), SizeHint::bounded(11, 15)), BOUNDS_ERROR);
identity!(overflow_equality, CapacityMismatch::overflow(SizeHint::bounded(5, 10)), OVERFLOW_ERROR);
identity!(underflow_equality, CapacityMismatch::underflow(SizeHint::bounded(5, 10), 3), UNDERFLOW_ERROR);
