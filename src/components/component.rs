pub trait Component
where
	Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}
