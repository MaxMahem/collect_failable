use collect_failable::errors::partial_array::{ArrayIndex, PostInc};

#[test]
fn array_index() {
    let mut index = ArrayIndex::<3>::new();
    assert_eq!(*index, 0);

    assert_eq!(index.try_post_inc(), Some(0));
    assert_eq!(*index, 1);

    assert_eq!(index.try_post_inc(), Some(1));
    assert_eq!(*index, 2);

    assert_eq!(index.try_post_inc(), Some(2));
    assert_eq!(*index, 3);

    assert_eq!(index.try_post_inc(), None);
    assert_eq!(*index, 3);
}

#[test]
fn post_inc() {
    let mut index = 0;
    assert_eq!(index.post_inc(), 0);
    assert_eq!(index, 1);
}
