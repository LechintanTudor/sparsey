use crate::component::NonZeroStorageMask;
use core::cmp;

/// Grouping information for any number of views in a query.
#[derive(Clone, Copy, Default, Debug)]
pub enum QueryGroupInfo {
    /// The query is empty.
    #[default]
    Empty,
    /// The query has one view.
    One(ViewGroupInfo),
    /// The query has multiple compatible views.
    Many(GroupInfo),
}

impl QueryGroupInfo {
    /// Tries to add two group infos.
    ///
    /// Returns the new group info if the operands were compatible.
    #[inline]
    #[must_use]
    pub fn add_query(&self, other: &Self) -> Option<Self> {
        let query = match (self, other) {
            (Self::Empty, Self::Empty) => Self::Empty,
            (query, Self::Empty) | (Self::Empty, query) => *query,
            (query_a, query_b) => {
                let query_a = query_a.group_info()?;
                let query_b = query_b.group_info()?;
                Self::Many(query_a.add_group(query_b)?)
            }
        };

        Some(query)
    }

    /// Tries to add a view to the current group info.
    ///
    /// Returns the new group info if the operands were compatible.
    #[inline]
    #[must_use]
    pub fn add_view(&self, view: &ViewGroupInfo) -> Option<Self> {
        match self {
            Self::Empty => Some(Self::One(*view)),
            Self::One(old_view) => {
                if let (Some(info_a), Some(info_b)) = (old_view.info, &view.info) {
                    Some(Self::Many(info_a.add_group(*info_b)?))
                } else {
                    None
                }
            }
            Self::Many(old_info) => {
                if let Some(info) = &view.info {
                    Some(Self::Many(old_info.add_group(*info)?))
                } else {
                    None
                }
            }
        }
    }

    /// Returns the group info of this query, if any.
    #[inline]
    #[must_use]
    pub fn group_info(&self) -> Option<GroupInfo> {
        match self {
            Self::Empty => None,
            Self::One(view) => view.info,
            Self::Many(info) => Some(*info),
        }
    }
}

/// Stores the length and grouping information for a component view.
#[derive(Clone, Copy, Debug)]
pub struct ViewGroupInfo {
    /// The group info of the view, if any.
    pub info: Option<GroupInfo>,
    /// The number of components in the view.
    pub len: usize,
}

/// Grouping information for one or more views.
#[derive(Clone, Copy, Debug)]
pub struct GroupInfo {
    pub(crate) group_start: u8,
    pub(crate) group_end: u8,
    pub(crate) storage_mask: NonZeroStorageMask,
}

impl GroupInfo {
    /// Tries to add two group infos.
    ///
    /// Returns the new group info if the operands were compatible.
    #[inline]
    #[must_use]
    pub fn add_group(self, other: GroupInfo) -> Option<GroupInfo> {
        if self.group_start != other.group_start {
            return None;
        }

        Some(GroupInfo {
            group_start: self.group_start,
            group_end: cmp::max(self.group_end, other.group_end),
            storage_mask: self.storage_mask | other.storage_mask,
        })
    }
}
