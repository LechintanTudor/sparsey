/// Trait automatically implemented for all types that can be used as components. (Any `Send + Sync
/// + 'static` type)
pub trait Component: Send + Sync + 'static {
    // Empty
}

impl<T> Component for T
where
    T: Send + Sync + 'static,
{
    // Empty
}
