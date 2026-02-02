# TODO

- [X] Explore `no_alloc` support for `CollectError` to allow `ArrayVec` extensions to be used in environments without `alloc`.
  - Now when not using `alloc` errors are not boxed.
- [ ] Explore adding `TryUnzip` for more than two collections.
- [ ] Explore adding `TryExtend` for more than two collections.
- [ ] Explore a safe method of collecting into an array.
- [X] Explore not panicking on invalid size hints.
  - Decided to panic on invalid size hints, and other logic errors.
- [ ] Consider spinning of Capacity trait into its own crate, or as a part of `size_hinter` crate.
- [ ] Add more implementations.
  - `TinyVec`
  