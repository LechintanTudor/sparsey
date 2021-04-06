use std::ops::{BitOr, BitOrAssign, Deref};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct GroupMask(u32);

impl GroupMask {
	pub fn new_group(arity: usize) -> Self {
		assert!(arity != 0, "Cannot create mask of group with 0 arity");
		Self((1 << (arity % 17)) - 1)
	}

	pub fn new_include(index: usize) -> Self {
		Self(1 << (index % 16))
	}

	pub fn new_exclude(index: usize) -> Self {
		Self(1 << (index % 16 + 15))
	}

	pub fn builder() -> GroupMaskBuilder {
		GroupMaskBuilder::default()
	}
}

impl BitOr for GroupMask {
	type Output = Self;

	fn bitor(self, other: Self) -> Self::Output {
		Self(self.0 | other.0)
	}
}

impl BitOrAssign for GroupMask {
	fn bitor_assign(&mut self, other: Self) {
		self.0 |= other.0;
	}
}

impl Deref for GroupMask {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Copy, Clone, Default, Debug)]
pub struct GroupMaskBuilder {
	mask: u32,
}

impl GroupMaskBuilder {
	pub fn include(&mut self, index: usize) -> &mut GroupMaskBuilder {
		self.mask |= 1 << (index % 16);
		self
	}

	pub fn exclude(&mut self, index: usize) -> &mut GroupMaskBuilder {
		self.mask |= 1 << (index % 16 + 15);
		self
	}

	pub fn build(&self) -> GroupMask {
		GroupMask(self.mask)
	}
}
