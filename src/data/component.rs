/// Market trait automatically derived for all types
/// which can be used as components in the `World`.
pub trait Component
where
    Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}
