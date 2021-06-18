/// Marker trait automatically implemented for all types that are `Send + Sync +
/// 'static`.
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
