use arrayvec::CapacityError;
use collect_failable::ExceedsCapacity;

#[test]
fn from_exceeds_capacity_to_capacity_error() {
    let exceeds = ExceedsCapacity::new(10, 15);
    let capacity_error: CapacityError<()> = exceeds.into();

    // CapacityError contains a unit value
    assert_eq!(capacity_error.element(), ());
}
