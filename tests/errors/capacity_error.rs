use collect_failable::SizeHint;
use collect_failable::errors::{CapacityError, CapacityErrorKind};

use crate::error_tests::test_ctor;

test_ctor!(
    bounds,
    CapacityError::<()>::bounds(SizeHint::bounded(5, 10), SizeHint::bounded(11, 15)),
    capacity => SizeHint::bounded(5, 10),
    kind => CapacityErrorKind::Bounds { hint: SizeHint::bounded(11, 15) }
);

test_ctor!(
    overflow,
    CapacityError::overflow(SizeHint::bounded(5, 10), 42),
    capacity => SizeHint::bounded(5, 10),
    kind => CapacityErrorKind::Overflow { rejected: 42 }
);

test_ctor!(
    underflow,
    CapacityError::<()>::underflow(SizeHint::bounded(5, 10), 4),
    capacity => SizeHint::bounded(5, 10),
    kind => CapacityErrorKind::Underflow { count: 4 }
);

mod error_item_provider {
    use super::*;
    use collect_failable::errors::ErrorItemProvider;

    #[test]
    fn item_present() {
        let error = CapacityError::overflow(SizeHint::bounded(5, 10), 42);

        assert_eq!(error.item(), Some(&42));
        assert_eq!(error.into_item(), Some(42));
    }

    #[test]
    fn item_absent() {
        let error = CapacityError::<()>::bounds(SizeHint::bounded(5, 10), SizeHint::bounded(11, 15));

        assert_eq!(error.item(), None);
        assert_eq!(error.into_item(), None);
    }
}
