use collect_failable::{Capacity, SizeHint};

#[test]
fn capacity() {
    let vec = Vec::<i32>::new();
    assert_eq!(vec.capacity_hint(), SizeHint::UNIVERSAL);
}
