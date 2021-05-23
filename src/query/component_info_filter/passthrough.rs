use crate::components::Entity;
use crate::query::{AndFilter, OrFilter, QueryComponentInfoFilter};
use std::ops::{BitAnd, BitOr};

#[derive(Copy, Clone, Debug)]
pub struct PassthroughFilter;

impl QueryComponentInfoFilter for PassthroughFilter {
	fn matches(&self, _: Entity) -> bool {
		true
	}
}

impl<Q> BitAnd<Q> for PassthroughFilter
where
	Q: QueryComponentInfoFilter,
{
	type Output = AndFilter<Self, Q>;

	fn bitand(self, other: Q) -> Self::Output {
		AndFilter::new(self, other)
	}
}

impl<Q> BitOr<Q> for PassthroughFilter
where
	Q: QueryComponentInfoFilter,
{
	type Output = OrFilter<Self, Q>;

	fn bitor(self, other: Q) -> Self::Output {
		OrFilter::new(self, other)
	}
}
