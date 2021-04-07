use std::ops::{BitOr, BitOrAssign, Deref};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct GroupMask(u32);

impl GroupMask {
	pub fn include_group(arity: usize) -> Self {
		Self((1 << arity) - 1)
	}

	pub fn exclude_group(arity: usize, prev_arity: usize) -> Self {
		if prev_arity != 0 {
			let exclude_count = arity - prev_arity;
			let exclude_mask = ((1 << exclude_count) - 1) << (16 + prev_arity);
			let include_mask = (1 << prev_arity) - 1;

			Self(include_mask | exclude_mask)
		} else {
			Self(0)
		}
	}

	pub fn include_index(index: usize) -> Self {
		Self(1 << (index % 16))
	}

	pub fn exclude_index(index: usize) -> Self {
		Self(1 << (index % 16 + 16))
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
