# TODO

- [ ] Explore `no_alloc` support for `CollectError` to allow `ArrayVec` extensions to be used in strictly `no_std` environments (without `alloc`).
  - Currently `CollectError` uses `Box` to minimize stack size.
  - We might need a feature flag or a generic strategy to allow unboxed errors or alternative storage.
- [ ] Explore adding `TryUnzip` for more than two collections.
