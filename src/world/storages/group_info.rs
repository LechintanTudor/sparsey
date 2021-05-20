use crate::world::{Group, GroupMask};
use std::ops::Range;
use std::ptr;

#[derive(Copy, Clone, Debug)]
pub enum GroupInfo<'a> {
	Ungrouped,
	Grouped(GroupInfoData<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct GroupInfoData<'a> {
	family: &'a [Group],
	group_index: u8,
	storage_index: u8,
}

impl<'a> GroupInfoData<'a> {
	pub(crate) fn new(family: &'a [Group], group_index: u8, storage_index: u8) -> Self {
		Self {
			family,
			group_index,
			storage_index,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum CombinedGroupInfo<'a> {
	Empty,
	Ungrouped,
	Grouped(CombinedGroupInfoData<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct CombinedGroupInfoData<'a> {
	family: &'a [Group],
	mask: u16,
	index: u8,
}

impl<'a> CombinedGroupInfo<'a> {
	pub(crate) fn combine(self, info: GroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, GroupInfo::Grouped(data)) => Self::Grouped(CombinedGroupInfoData {
				family: data.family,
				mask: 1 << data.storage_index,
				index: data.group_index,
			}),
			(Self::Grouped(combined_data), GroupInfo::Grouped(data)) => {
				if !ptr::eq(combined_data.family, data.family) {
					Self::Ungrouped
				} else {
					Self::Grouped(CombinedGroupInfoData {
						family: combined_data.family,
						mask: combined_data.mask | (1 << data.storage_index),
						index: combined_data.index.max(data.group_index),
					})
				}
			}
			(_, _) => Self::Ungrouped,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum QueryGroupInfo<'a> {
	Empty,
	Ungrouped,
	Grouped(QueryGroupInfoData<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct QueryGroupInfoData<'a> {
	family: &'a [Group],
	mask: GroupMask,
	index: u8,
}

impl<'a> QueryGroupInfo<'a> {
	pub(crate) fn include(self, info: CombinedGroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, CombinedGroupInfo::Empty) => Self::Empty,
			(Self::Grouped(_), CombinedGroupInfo::Empty) => self,
			(Self::Grouped(query_info), CombinedGroupInfo::Grouped(info)) => {
				if !ptr::eq(query_info.family, info.family) {
					Self::Ungrouped
				} else {
					Self::Grouped(QueryGroupInfoData {
						family: query_info.family,
						mask: query_info.mask.include(info.mask),
						index: query_info.index.max(info.index),
					})
				}
			}
			(_, _) => Self::Ungrouped,
		}
	}

	pub(crate) fn exclude(self, info: CombinedGroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, CombinedGroupInfo::Empty) => Self::Empty,
			(Self::Grouped(_), CombinedGroupInfo::Empty) => self,
			(Self::Grouped(query_info), CombinedGroupInfo::Grouped(info)) => {
				if !ptr::eq(query_info.family, info.family) {
					Self::Ungrouped
				} else {
					Self::Grouped(QueryGroupInfoData {
						family: query_info.family,
						mask: query_info.mask.exclude(info.mask),
						index: query_info.index.max(info.index),
					})
				}
			}
			(_, _) => Self::Ungrouped,
		}
	}

	pub(crate) fn group_range(&self) -> Option<Range<usize>> {
		match self {
			Self::Grouped(info) => {
				let group = info.family[info.index as usize];

				if info.mask == group.include_mask() {
					Some(0..group.len())
				} else if info.mask == group.exclude_mask() {
					let prev_group = info.family[(info.index - 1) as usize];
					Some(group.len()..prev_group.len())
				} else {
					None
				}
			}
			_ => None,
		}
	}
}
