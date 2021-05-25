use crate::components::Entity;
use crate::query::QueryFilter;
use std::ops::{BitAnd, BitOr};

pub struct AndFilter<Q1, Q2>(Q1, Q2);

impl<Q1, Q2> AndFilter<Q1, Q2> {
	pub(crate) fn new(q1: Q1, q2: Q2) -> Self {
		Self(q1, q2)
	}
}

impl<Q1, Q2> QueryFilter for AndFilter<Q1, Q2>
where
	Q1: QueryFilter,
	Q2: QueryFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.matches(entity) && self.1.matches(entity)
	}
}

impl<Q1, Q2, Q3> BitAnd<Q3> for AndFilter<Q1, Q2>
where
	Q1: QueryFilter,
	Q2: QueryFilter,
	Q3: QueryFilter,
{
	type Output = AndFilter<Self, Q3>;

	fn bitand(self, other: Q3) -> Self::Output {
		AndFilter::new(self, other)
	}
}

impl<Q1, Q2, Q3> BitOr<Q3> for AndFilter<Q1, Q2>
where
	Q1: QueryFilter,
	Q2: QueryFilter,
	Q3: QueryFilter,
{
	type Output = OrFilter<Self, Q3>;

	fn bitor(self, other: Q3) -> Self::Output {
		OrFilter::new(self, other)
	}
}

pub struct OrFilter<Q1, Q2>(Q1, Q2);

impl<Q1, Q2> OrFilter<Q1, Q2> {
	pub(crate) fn new(q1: Q1, q2: Q2) -> Self {
		Self(q1, q2)
	}
}

impl<Q1, Q2> QueryFilter for OrFilter<Q1, Q2>
where
	Q1: QueryFilter,
	Q2: QueryFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.matches(entity) || self.1.matches(entity)
	}
}

impl<Q1, Q2, Q3> BitAnd<Q3> for OrFilter<Q1, Q2>
where
	Q1: QueryFilter,
	Q2: QueryFilter,
	Q3: QueryFilter,
{
	type Output = AndFilter<Self, Q3>;

	fn bitand(self, other: Q3) -> Self::Output {
		AndFilter::new(self, other)
	}
}

impl<Q1, Q2, Q3> BitOr<Q3> for OrFilter<Q1, Q2>
where
	Q1: QueryFilter,
	Q2: QueryFilter,
	Q3: QueryFilter,
{
	type Output = OrFilter<Self, Q3>;

	fn bitor(self, other: Q3) -> Self::Output {
		OrFilter::new(self, other)
	}
}
