use crate::components;
use crate::components::GroupInfo;
use std::ops::Range;

#[derive(Clone, Copy)]
pub enum QueryGroupInfo<'a> {
    Empty,
    Single { len: usize, info: Option<GroupInfo<'a>> },
    Multiple(GroupInfo<'a>),
}

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
        (Empty, include @ Single { .. }) => include,
        (get @ Single { .. }, Empty) => get,
        (Single { info: Some(info1), .. }, Single { info: Some(info2), .. })
        | (Single { info: Some(info1), .. }, Multiple(info2))
        | (Multiple(info1), Single { info: Some(info2), .. })
        | (Multiple(info1), Multiple(info2)) => Multiple(info1.combine(info2)?),
        _ => return None,
    };

    match (include, exclude) {
        (Single { len, .. }, Empty) => Some(0..len),
        (Single { info: Some(include), .. }, Single { info: Some(exclude), .. })
        | (Single { info: Some(include), .. }, Multiple(exclude))
        | (Multiple(include), Single { info: Some(exclude), .. })
        | (Multiple(include), Multiple(exclude)) => components::group_range(include, exclude),
        _ => todo!(),
    }
}
