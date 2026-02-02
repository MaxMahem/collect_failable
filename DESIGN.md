# Various Design Notes

## Error Handling

In so much as possible, this crate uses strongly typed and narrowly focused errors that can be used to retrieve the provided data in the case of a failure. Ideally, the errors should only be able to represent the failure modes that are possible for the given operation, hence why `CollectError` and `ExtendError` are generic over a wrapped inner error type.

An exception was made for `CapacityError` because while some sorts of capacity errors are not possible for some operations (IE an 'Extend' operation can never 'underflow') the type would become to complex if we modeled this.

## Error Boxing

When the `alloc` feature is enabled, errors data is boxed. This is done because errors should generally be on the cold path, and the error data can be large in some cases (IE arrays).

## Readonly Errors

Ideally, errors should be immutable. But in rust, this currently locks you out of pattern matching and other features. As a compromise, errors are 'readonly' by implementing `Deref` to a hidden inner error types (the same type that is boxed). This type can be pattern matched (though you do need to name it) or destructured via the `into_data` methods.

## Error Validation

Some errors imply various logical constraints:

* A `CapacityError` of a `Bounds` variety should contain disjoint bounds data.
* A `CapacityError` of a `Overflow` variety should have a maximum capacity that can be overflowed.
* A `CapacityError` of a `Underflow` variety should have a underflow count that is less than the minimum capacity.
* Any method that takes in an `Iterator` and reads its `size_hint` validates that the `size_hint` is valid.

These logical constraints are enforced via `panic!`, since they represent program logic error, and the cost of the check should be unimportant on the error path. If necessary, fail-free methods are still possible via `new`.
