#[derive(Copy, Clone, Default, Debug)]
pub struct ComponentInfo {
	pub(crate) tick_added: u32,
	pub(crate) tick_mutated: u32,
}

impl ComponentInfo {
	pub fn new(tick_added: u32) -> Self {
		Self {
			tick_added,
			tick_mutated: 0,
		}
	}

	pub fn tick_added(&self) -> u32 {
		self.tick_added
	}

	pub fn tick_mutated(&self) -> u32 {
		self.tick_mutated
	}
}
