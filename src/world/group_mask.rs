use std::ops::{BitOr, BitOrAssign};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub(crate) struct GroupMask {
	include: u16,
	exclude: u16,
}

impl GroupMask {
	pub const fn empty() -> Self {
		Self {
			include: 0,
			exclude: 0,
		}
	}

	pub const fn new(include: u16, exclude: u16) -> Self {
		Self { include, exclude }
	}

	pub const fn new_include_group(arity: usize) -> Self {
		Self {
			include: (1 << arity) - 1,
			exclude: 0,
		}
	}

	pub const fn new_exclude_group(arity: usize, prev_arity: usize) -> Self {
		if prev_arity != 0 {
			let exclude_count = arity - prev_arity;

			Self {
				include: (1 << prev_arity) - 1,
				exclude: ((1 << exclude_count) - 1) << prev_arity,
			}
		} else {
			Self::empty()
		}
	}

	pub const fn include(&self, mask: u16) -> Self {
		Self {
			include: self.include | mask,
			exclude: self.exclude,
		}
	}

	pub const fn exclude(&self, mask: u16) -> Self {
		Self {
			include: self.include,
			exclude: self.exclude | mask,
		}
	}
}

impl BitOr for GroupMask {
	type Output = Self;

	fn bitor(self, other: Self) -> Self::Output {
		Self {
			include: self.include | other.include,
			exclude: self.exclude | other.exclude,
		}
	}
}

impl BitOrAssign for GroupMask {
	fn bitor_assign(&mut self, other: Self) {
		self.include |= other.include;
		self.exclude |= other.exclude;
	}
}
