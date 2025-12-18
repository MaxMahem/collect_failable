use collect_failable::CapacityMismatch;

use crate::error_tests::identity;

const BOUNDS_ERROR: CapacityMismatch = CapacityMismatch::bounds(5..=10, (11, Some(15)));
const OVERFLOW_ERROR: CapacityMismatch = CapacityMismatch::overflow(5..=10);
const UNDERFLOW_ERROR: CapacityMismatch = CapacityMismatch::underflow(5..=10, 3);

identity!(bounds_equality, CapacityMismatch::bounds(5..=10, (11, Some(15))), BOUNDS_ERROR);
identity!(overflow_equality, CapacityMismatch::overflow(5..=10), OVERFLOW_ERROR);
identity!(underflow_equality, CapacityMismatch::underflow(5..=10, 3), UNDERFLOW_ERROR);

// bounds() should panic if hint is within capacity range
identity!(bounds_panic_within_range, CapacityMismatch::bounds(5..=10, (7, Some(9))), panics: "hint must be outside capacity range");

// bounds() should panic if hint min is within range and max is None
identity!(bounds_panic_unbounded, CapacityMismatch::bounds(5..=10, (7, None)), panics: "hint must be outside capacity range");

// underflow() should panic if count is >= capacity minimum
identity!(underflow_panic, CapacityMismatch::underflow(5..=10, 5), panics: "count must be less than capacity minimum");
