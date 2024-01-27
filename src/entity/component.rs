/// Marker trait for components that can be added to entities.
pub trait Component: Send + Sync + 'static {
    // Empty
}

impl<T> Component for T
where
    T: Send + Sync + 'static,
{
    // Empty
}
