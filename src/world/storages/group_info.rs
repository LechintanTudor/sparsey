use crate::world::{Group, GroupMask};
use std::ops::Range;
use std::ptr;

#[derive(Copy, Clone)]
pub struct GroupInfo<'a> {
	family: &'a [Group],
	group_index: u32,
	storage_index: u32,
}

impl<'a> GroupInfo<'a> {
	pub(crate) fn new(family: &'a [Group], group_index: u32, storage_index: u32) -> Self {
		Self {
			family,
			group_index,
			storage_index,
		}
	}

	pub fn mask(&self) -> u16 {
		1 << self.storage_index
	}
}

#[derive(Copy, Clone)]
pub enum QueryGroupInfo<'a> {
	Empty,
	Grouped(QueryGroupInfoData<'a>),
}

impl<'a> QueryGroupInfo<'a> {
	pub const fn new() -> Self {
		Self::Empty
	}

	pub fn with_group(self, info: GroupInfo<'a>) -> Option<Self> {
		match self {
			Self::Empty => Some(Self::Grouped(QueryGroupInfoData::new(info))),
			Self::Grouped(data) => Some(Self::Grouped(data.with_group(info)?)),
		}
	}
}

#[derive(Copy, Clone)]
pub struct QueryGroupInfoData<'a> {
	family: &'a [Group],
	mask: u16,
	index: u32,
}

impl<'a> QueryGroupInfoData<'a> {
	fn new(info: GroupInfo<'a>) -> Self {
		Self {
			family: info.family,
			mask: info.mask(),
			index: info.group_index,
		}
	}

	fn with_group(self, info: GroupInfo) -> Option<Self> {
		if !ptr::eq(self.family, info.family) {
			return None;
		}

		Some(Self {
			family: self.family,
			mask: self.mask | info.mask(),
			index: self.index.max(info.group_index),
		})
	}
}

#[derive(Copy, Clone)]
pub enum CombinedQueryGroupInfo<'a> {
	Empty,
	Grouped(CombinedQueryGroupInfoData<'a>),
}

impl<'a> CombinedQueryGroupInfo<'a> {
	pub const fn new() -> Self {
		Self::Empty
	}

	pub fn include(self, info: QueryGroupInfo<'a>) -> Option<Self> {
		let info = match info {
			QueryGroupInfo::Grouped(data) => data,
			QueryGroupInfo::Empty => return Some(self),
		};

		match self {
			Self::Empty => Some(Self::Grouped(CombinedQueryGroupInfoData::new_include(info))),
			Self::Grouped(data) => Some(Self::Grouped(data.include(info)?)),
		}
	}

	pub fn exclude(self, info: QueryGroupInfo<'a>) -> Option<Self> {
		let info = match info {
			QueryGroupInfo::Grouped(data) => data,
			QueryGroupInfo::Empty => return Some(self),
		};

		match self {
			Self::Empty => Some(Self::Grouped(CombinedQueryGroupInfoData::new_exclude(info))),
			Self::Grouped(data) => Some(Self::Grouped(data.exclude(info)?)),
		}
	}
}

#[derive(Copy, Clone)]
pub struct CombinedQueryGroupInfoData<'a> {
	family: &'a [Group],
	mask: GroupMask,
	index: u32,
}

impl<'a> CombinedQueryGroupInfoData<'a> {
	fn new_include(info: QueryGroupInfoData<'a>) -> Self {
		Self {
			family: info.family,
			mask: GroupMask::new(info.mask, 0),
			index: info.index,
		}
	}

	fn new_exclude(info: QueryGroupInfoData<'a>) -> Self {
		Self {
			family: info.family,
			mask: GroupMask::new(0, info.mask),
			index: info.index,
		}
	}

	fn include(self, info: QueryGroupInfoData) -> Option<Self> {
		if !ptr::eq(self.family, info.family) {
			return None;
		}

		Some(Self {
			family: self.family,
			mask: self.mask.include(info.mask),
			index: self.index.max(info.index),
		})
	}

	fn exclude(self, info: QueryGroupInfoData) -> Option<Self> {
		if !ptr::eq(self.family, info.family) {
			return None;
		}

		Some(Self {
			family: self.family,
			mask: self.mask.exclude(info.mask),
			index: self.index.max(info.index),
		})
	}
}
