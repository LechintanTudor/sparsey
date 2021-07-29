use crate::group::GroupMask;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Group {
	arity: usize,
	include_mask: GroupMask,
	exclude_mask: GroupMask,
	pub(crate) len: usize,
}

impl Group {
	pub fn new(prev_arity: usize, arity: usize) -> Self {
		Self {
			arity,
			include_mask: GroupMask::new_include_group(arity),
			exclude_mask: GroupMask::new_exclude_group(prev_arity, arity),
			len: 0,
		}
	}

	pub fn arity(&self) -> usize {
		self.arity
	}

	pub fn include_mask(&self) -> GroupMask {
		self.include_mask
	}

	pub fn exclude_mask(&self) -> GroupMask {
		self.exclude_mask
	}

	pub fn len(&self) -> usize {
		self.len
	}
}
