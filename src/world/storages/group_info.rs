use crate::world::{Group, GroupMask};
use std::ptr;

/// Holds information about the layout in the `World`
/// of the component storage from which it was created.
#[derive(Copy, Clone)]
pub struct GroupInfo<'a> {
	groups: &'a [Group],
	group_index: usize,
	storage_index: usize,
}

impl<'a> GroupInfo<'a> {
	pub(crate) fn new(groups: &'a [Group], group_index: usize, storage_index: usize) -> Self {
		Self {
			groups,
			group_index,
			storage_index,
		}
	}

	pub(crate) fn has_same_group_set(&self, other: &GroupInfo) -> bool {
		ptr::eq(self.groups, other.groups)
	}

	pub(crate) fn group_index(&self) -> usize {
		self.group_index
	}

	pub(crate) fn mask(&self) -> GroupMask {
		GroupMask::include_index(self.storage_index)
	}

	pub(crate) fn groups(&self) -> &[Group] {
		self.groups
	}
}

pub(crate) fn get_group_len(group_infos: &[GroupInfo]) -> Option<usize> {
	let (first, others) = group_infos.split_first()?;
	let mut group_index = first.group_index();
	let mut group_mask = first.mask();

	for other in others {
		if !first.has_same_group_set(other) {
			return None;
		}

		group_index = group_index.max(other.group_index());
		group_mask |= other.mask();
	}

	let groups = first.groups();
	let group = unsafe { groups.get_unchecked(group_index) };

	if group.include_mask() == group_mask {
		Some(group.len())
	} else if group.exclude_mask() == group_mask {
		todo!()
	} else {
		None
	}
}
