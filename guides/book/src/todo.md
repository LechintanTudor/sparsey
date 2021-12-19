# TODO

```rust, ignore
pub struct FearlessConcurrency<T>(pub T);
unsafe impl<T> Send for FearlessConcurrency<T> {}
unsafe impl<T> Sync for FearlessConcurrency<T> {}
```