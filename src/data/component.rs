/// Market trait automatically implemented for all types
/// which can be used as components in the `World`.
pub trait Component
where
	Self: Send + Sync + 'static,
{
}

impl<T> Component for T where T: Send + Sync + 'static {}

#[derive(Copy, Clone, Default, Debug)]
pub struct ComponentInfo {
	pub(crate) tick_added: u32,
	pub(crate) tick_mutated: u32,
}

impl ComponentInfo {
	pub(crate) fn new(tick_added: u32, tick_mutated: u32) -> Self {
		Self {
			tick_added,
			tick_mutated,
		}
	}

	pub fn tick_added(&self) -> u32 {
		self.tick_added
	}

	pub fn tick_mutated(&self) -> u32 {
		self.tick_mutated
	}
}
