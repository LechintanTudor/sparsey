pub trait Component
where
    Self: Send + Sync + 'static,
{
    // Marker
}

impl<T> Component for T where T: Send + Sync + 'static {}
