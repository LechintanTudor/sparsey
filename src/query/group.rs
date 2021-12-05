use crate::components::QueryGroupInfo;
use crate::query::{QueryGet, QueryModifier};
use std::error::Error;
use std::fmt;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct InvalidGroup;

impl Error for InvalidGroup {}

impl fmt::Display for InvalidGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tried to slice ungrouped component views")
    }
}

pub(crate) fn is_trivial_group<'a, G, I, E>() -> bool
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
{
    G::GETS_ONE && I::IS_PASSTHROUGH && E::IS_PASSTHROUGH
}

pub(crate) fn get_group_range<'a, G, I, E>(
    get: &G,
    include: &I,
    exclude: &E,
) -> Result<Range<usize>, InvalidGroup>
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
        .ok_or(InvalidGroup)
}
