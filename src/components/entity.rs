use std::num::NonZeroU32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Entity {
	version: Version,
	index: u32,
}

impl Entity {
	pub const fn new(index: u32, version: Version) -> Self {
		Self { index, version }
	}

	pub const fn with_index(index: u32) -> Self {
		unsafe {
			Self {
				index,
				version: Version::new_unchecked(1),
			}
		}
	}

	pub const fn index(&self) -> usize {
		self.index as _
	}

	pub const fn version(&self) -> Version {
		self.version
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Version(NonZeroU32);

impl Default for Version {
	fn default() -> Self {
		unsafe { Self(NonZeroU32::new_unchecked(1)) }
	}
}

impl Version {
	pub fn new(id: u32) -> Self {
		Self(NonZeroU32::new(id).expect("Version ID cannot be zero"))
	}

	pub const unsafe fn new_unchecked(id: u32) -> Self {
		Self(NonZeroU32::new_unchecked(id))
	}

	pub const fn get(&self) -> u32 {
		self.0.get()
	}
}
