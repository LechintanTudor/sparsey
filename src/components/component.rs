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
