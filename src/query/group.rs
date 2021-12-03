use std::ops::Range;

use crate::components::QueryGroupInfo;
use crate::query::{QueryGet, QueryModifier};

pub(crate) fn is_trivial_group<'a, G, I, E>() -> bool
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
{
    G::GETS_ONE && I::IS_PASSTHROUGH && E::IS_PASSTHROUGH
}

pub(crate) fn group_range<'a, G, I, E>(get: &G, include: &I, exclude: &E) -> Option<Range<usize>>
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
{
    debug_assert!(!is_trivial_group::<G, I, E>());

    get.group_info()
        .and_then(|info| include.include_group_info(info))
        .and_then(|info| exclude.exclude_group_info(info))
        .and_then(QueryGroupInfo::group_range)
}
