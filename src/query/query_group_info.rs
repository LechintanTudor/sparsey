use crate::entity::GroupInfo;
use crate::query::QueryPart;
use std::ops::Range;

/// Holds grouping information about a query.
#[derive(Clone, Copy, Debug)]
pub enum QueryGroupInfo<'a> {
    /// The query is empty.
    Empty,
    /// The query has a single component view.
    Single {
        /// The number of components in the component view.
        len: usize,
        /// The group info of the component view, if any.
        group_info: Option<GroupInfo<'a>>,
    },
    /// The query has multiple component views.
    Multiple(GroupInfo<'a>),
}

impl<'a> QueryGroupInfo<'a> {
    /// Returns the inner group info, if any.
    #[inline]
    #[must_use]
    pub const fn as_group_info(self) -> Option<GroupInfo<'a>> {
        match self {
            Self::Empty => None,
            Self::Single { group_info, .. } => group_info,
            Self::Multiple(group_info) => Some(group_info),
        }
    }
}

/// Returns the group range of the query described by the given parts.
#[must_use]
pub fn group_range<G, I, E>(get: &G, include: &I, exclude: &E) -> Option<Range<usize>>
where
    G: QueryPart,
    I: QueryPart,
    E: QueryPart,
{
    let get = get.group_info()?;
    let include = include.group_info()?;
    let exclude = exclude.group_info()?;
    group_range_impl(get, include, exclude)
}

#[must_use]
fn group_range_impl(
    get: QueryGroupInfo<'_>,
    include: QueryGroupInfo<'_>,
    exclude: QueryGroupInfo<'_>,
) -> Option<Range<usize>> {
    use QueryGroupInfo as Info;

    let include = match (get, include) {
        (Info::Empty, Info::Empty) => panic!("Cannot process empty Query"),
        (get, Info::Empty) => get,
        (Info::Empty, include) => include,
        (get, include) => {
            let get = get.as_group_info()?;
            let include = include.as_group_info()?;
            Info::Multiple(get.combine(&include)?)
        }
    };

    match (include, exclude) {
        (Info::Single { len, .. }, Info::Empty) => Some(0..len),
        (Info::Multiple(include), Info::Empty) => include.include_group_range(),
        (include, exclude) => {
            let include = include.as_group_info()?;
            let exclude = exclude.as_group_info()?;
            include.exclude_group_range(&exclude)
        }
    }
}
