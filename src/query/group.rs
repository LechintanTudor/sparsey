use crate::components;
use crate::query::{QueryBase, QueryModifier};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Range;

/// Error returned when trying to slice a `Query` with ungrouped component
/// storages.
#[derive(Debug)]
pub struct InvalidGroup;

impl Error for InvalidGroup {
    // Empty
}

impl Display for InvalidGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tried to slice query with ungrouped component storages")
    }
}

/// Returns `true` if the query has a single element and no modifiers.
pub(crate) fn is_trivial_group<'a, B, I, E>() -> bool
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
{
    B::ELEMENT_COUNT == 1 && I::IS_PASSTHROUGH && E::IS_PASSTHROUGH
}

/// For non trivial groups, returns the range of elements in the group.
pub(crate) fn group_range<'a, B, I, E>(
    base: &B,
    include: &I,
    exclude: &E,
) -> Result<Range<usize>, InvalidGroup>
where
    B: QueryBase<'a>,
    I: QueryModifier<'a>,
    E: QueryModifier<'a>,
{
    debug_assert!(!is_trivial_group::<B, I, E>());

    (|| -> Option<Range<usize>> {
        components::group_range(
            base.group_info()?,
            include.group_info()?,
            exclude.group_info()?,
        )
    })()
    .ok_or(InvalidGroup)
}
