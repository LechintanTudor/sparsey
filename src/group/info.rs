use crate::group::{Group, GroupMask};
use std::ops::Range;
use std::ptr;

#[derive(Copy, Clone)]
pub struct GroupInfo<'a> {
	group_family: &'a [Group],
	group_index: usize,
	storage_index: usize,
}

impl<'a> GroupInfo<'a> {
	pub(crate) fn new(group_family: &'a [Group], group_index: usize, storage_index: usize) -> Self {
		Self {
			group_family,
			group_index,
			storage_index,
		}
	}
}

#[derive(Copy, Clone, Default)]
pub struct CombinedGroupInfo<'a> {
	group_family: Option<&'a [Group]>,
	max_group_index: usize,
	group_mask: u16,
}

impl<'a> CombinedGroupInfo<'a> {
	pub fn combine(self, group_info: GroupInfo<'a>) -> Option<Self> {
		match self.group_family {
			Some(group_family) => {
				ptr::eq(group_family, group_info.group_family).then(|| CombinedGroupInfo {
					group_family: Some(group_family),
					max_group_index: self.max_group_index.max(group_info.group_index),
					group_mask: self.group_mask | (1 << group_info.storage_index),
				})
			}
			None => Some(CombinedGroupInfo {
				group_family: Some(group_info.group_family),
				max_group_index: group_info.group_index,
				group_mask: 1 << group_info.storage_index,
			}),
		}
	}
}

fn common_group_family<'a>(group_families: &[Option<&'a [Group]>]) -> Option<&'a [Group]> {
	let mut group_family: Option<&[Group]> = None;

	for &gf in group_families {
		if let Some(gf) = gf {
			match group_family {
				Some(group_family) => {
					if !ptr::eq(group_family, gf) {
						return None;
					}
				}
				None => group_family = Some(gf),
			}
		}
	}

	group_family
}

pub fn group_range(
	base: CombinedGroupInfo,
	include: CombinedGroupInfo,
	exclude: CombinedGroupInfo,
) -> Option<Range<usize>> {
	let group_family = common_group_family(&[
		base.group_family,
		include.group_family,
		exclude.group_family,
	])?;

	let max_group_index = base
		.max_group_index
		.max(include.max_group_index)
		.max(exclude.max_group_index);

	let group_mask = GroupMask::new(base.group_mask | include.group_mask, exclude.group_mask);
	let group = group_family[max_group_index as usize];

	if group_mask == group.include_mask() {
		Some(0..group.len())
	} else if group_mask == group.exclude_mask() {
		let prev_group = group_family[(max_group_index - 1) as usize];
		Some(group.len()..prev_group.len())
	} else {
		None
	}
}
