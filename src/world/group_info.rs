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
	group_index: usize,
	storage_index: usize,
}

impl<'a> GroupInfoData<'a> {
	pub(crate) fn new(family: &'a [Group], group_index: usize, storage_index: usize) -> Self {
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
	Unrelated,
	Related(CombinedGroupInfoData<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct CombinedGroupInfoData<'a> {
	family: &'a [Group],
	mask: u16,
	index: usize,
}

impl<'a> CombinedGroupInfo<'a> {
	pub(crate) fn from_group_info(info: GroupInfo<'a>) -> Self {
		Self::Empty.combine(info)
	}

	pub(crate) fn combine(self, info: GroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, GroupInfo::Grouped(data)) => Self::Related(CombinedGroupInfoData {
				family: data.family,
				mask: 1 << data.storage_index,
				index: data.group_index,
			}),
			(Self::Related(combined_data), GroupInfo::Grouped(data)) => {
				if !ptr::eq(combined_data.family, data.family) {
					Self::Unrelated
				} else {
					Self::Related(CombinedGroupInfoData {
						family: combined_data.family,
						mask: combined_data.mask | (1 << data.storage_index),
						index: combined_data.index.max(data.group_index),
					})
				}
			}
			(_, _) => Self::Unrelated,
		}
	}
}

#[derive(Copy, Clone, Debug)]
enum QueryGroupInfo<'a> {
	Empty,
	Unrelated,
	Related(QueryGroupInfoData<'a>),
}

#[derive(Copy, Clone, Debug)]
struct QueryGroupInfoData<'a> {
	family: &'a [Group],
	mask: GroupMask,
	index: usize,
}

impl<'a> QueryGroupInfoData<'a> {
	fn group_range(&self) -> Option<Range<usize>> {
		let group = self.family[self.index as usize];

		if self.mask == group.include_mask() {
			Some(0..group.len())
		} else if self.mask == group.exclude_mask() {
			let prev_group = self.family[(self.index - 1) as usize];
			Some(group.len()..prev_group.len())
		} else {
			None
		}
	}
}

impl<'a> QueryGroupInfo<'a> {
	fn new(
		query: CombinedGroupInfo<'a>,
		include: CombinedGroupInfo<'a>,
		exclude: CombinedGroupInfo<'a>,
	) -> Self {
		Self::Empty.include(query).include(include).exclude(exclude)
	}

	fn include(self, info: CombinedGroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, CombinedGroupInfo::Empty) => Self::Empty,
			(Self::Empty, CombinedGroupInfo::Related(info)) => Self::Related(QueryGroupInfoData {
				family: info.family,
				mask: GroupMask::new(info.mask, 0),
				index: info.index,
			}),
			(Self::Related(_), CombinedGroupInfo::Empty) => self,
			(Self::Related(query_info), CombinedGroupInfo::Related(info)) => {
				if !ptr::eq(query_info.family, info.family) {
					Self::Unrelated
				} else {
					Self::Related(QueryGroupInfoData {
						family: query_info.family,
						mask: query_info.mask.include(info.mask),
						index: query_info.index.max(info.index),
					})
				}
			}
			(_, _) => Self::Unrelated,
		}
	}

	fn exclude(self, info: CombinedGroupInfo<'a>) -> Self {
		match (self, info) {
			(Self::Empty, CombinedGroupInfo::Empty) => Self::Empty,
			(Self::Empty, CombinedGroupInfo::Related(info)) => Self::Related(QueryGroupInfoData {
				family: info.family,
				mask: GroupMask::new(0, info.mask),
				index: info.index,
			}),
			(Self::Related(_), CombinedGroupInfo::Empty) => self,
			(Self::Related(query_info), CombinedGroupInfo::Related(info)) => {
				if !ptr::eq(query_info.family, info.family) {
					Self::Unrelated
				} else {
					Self::Related(QueryGroupInfoData {
						family: query_info.family,
						mask: query_info.mask.exclude(info.mask),
						index: query_info.index.max(info.index),
					})
				}
			}
			(_, _) => Self::Unrelated,
		}
	}

	fn group_range(&self) -> Option<Range<usize>> {
		match self {
			Self::Related(data) => data.group_range(),
			_ => None,
		}
	}
}

pub fn group_range<'a>(
	base: CombinedGroupInfo<'a>,
	include: CombinedGroupInfo<'a>,
	exclude: CombinedGroupInfo<'a>,
) -> Option<Range<usize>> {
	QueryGroupInfo::new(base, include, exclude).group_range()
}
