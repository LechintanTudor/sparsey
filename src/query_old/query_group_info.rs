use crate::components::GroupInfo;
use std::ops::Range;

/// Tracks the family and group to which the component views that form a query belong.
#[doc(hidden)]
#[derive(Clone)]
pub enum QueryGroupInfo<'a> {
    /// The query is empty.
    Empty,
    /// The query contains a single component view which may belong to a family.
    Single { len: usize, info: Option<GroupInfo<'a>> },
    /// The query contains multiple component views that belong to the same family.
    Multiple(GroupInfo<'a>),
}

impl<'a> QueryGroupInfo<'a> {
    /// Returns the family to which all component views that form the query belong.
    pub fn as_group_info(self) -> Option<GroupInfo<'a>> {
        match self {
            Self::Empty => None,
            Self::Single { info, .. } => info,
            Self::Multiple(info) => Some(info),
        }
    }
}

/// Returns the range of the group described by the given group infos.
pub(crate) fn group_range(
    get: Option<QueryGroupInfo>,
    include: Option<QueryGroupInfo>,
    exclude: Option<QueryGroupInfo>,
) -> Option<Range<usize>> {
    use QueryGroupInfo::*;

    let get = get?;
    let include = include?;
    let exclude = exclude?;

    let include = match (get, include) {
        (Empty, Empty) => return None,
        (get, Empty) => get,
        (Empty, include) => include,
        (get, include) => {
            let get = get.as_group_info()?;
            let include = include.as_group_info()?;
            Multiple(get.combine(include)?)
        }
    };

    match (include, exclude) {
        (Single { len, .. }, Empty) => Some(0..len),
        (Multiple(include), Empty) => include.group_len().map(|l| 0..l),
        (include, exclude) => {
            let include = include.as_group_info()?;
            let exclude = exclude.as_group_info()?;
            include.exclude_group_range(&exclude)
        }
    }
}
