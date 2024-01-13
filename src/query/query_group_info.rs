use crate::entity::GroupInfo;
use crate::query::QueryPart;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub enum QueryGroupInfo<'a> {
    Empty,
    Single {
        len: usize,
        group_info: Option<GroupInfo<'a>>,
    },
    Multiple(GroupInfo<'a>),
}

impl<'a> QueryGroupInfo<'a> {
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
