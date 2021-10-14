/// Marker trait for types that can be stored in `ComponentStorage`s.
/// Automatically implemented for all `Send + Sync + 'static` types.
pub trait Component
where
    Self: Send + Sync + 'static,
{
    // Empty
}

impl<T> Component for T
where
    T: Send + Sync + 'static,
{
    // Empty
}
