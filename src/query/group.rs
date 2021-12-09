use crate::components::QueryGroupInfo;
use crate::query::{QueryGet, QueryModifier};
use std::error::Error;
use std::fmt;
use std::ops::Range;

/// Error returned when trying to slice ungrouped components.
#[derive(Clone, Debug)]
pub struct InvalidGroup;

impl Error for InvalidGroup {}

impl fmt::Display for InvalidGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tried to slice ungrouped component views")
    }
}

/// Returns `true` if the query only fetches one element and has no modifiers.
pub(crate) fn is_trivial_group<'a, G, I, E>() -> bool
where
    G: QueryGet<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
{
    G::GETS_ONE && I::IS_PASSTHROUGH && E::IS_PASSTHROUGH
}

/// For non-trivial groups, returns the range of grouped components.
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
