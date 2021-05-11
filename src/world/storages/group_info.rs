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

impl<'a> CombinedGroupInfo<'a> {
	pub const fn new() -> Self {
		Self::Empty
	}

	pub fn combine(self, info: GroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, GroupInfo::Grouped(data)) => {
				Self::Grouped(CombinedGroupInfoData::new(data))
			}
			(Self::Grouped(combined_data), GroupInfo::Grouped(data)) => {
				match combined_data.combine(data) {
					Some(combined_data) => Self::Grouped(combined_data),
					None => Self::Ungrouped,
				}
			}
			(_, _) => Self::Ungrouped,
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub struct CombinedGroupInfoData<'a> {
	family: &'a [Group],
	mask: u16,
	index: u8,
}

impl<'a> CombinedGroupInfoData<'a> {
	pub(crate) fn new(data: GroupInfoData<'a>) -> Self {
		Self {
			family: data.family,
			mask: 1 << data.storage_index,
			index: data.group_index,
		}
	}

	pub(crate) fn combine(self, data: GroupInfoData<'a>) -> Option<Self> {
		if !ptr::eq(self.family, data.family) {
			return None;
		}

		Some(Self {
			family: self.family,
			mask: self.mask | (1 << data.storage_index),
			index: self.index.max(data.group_index),
		})
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
