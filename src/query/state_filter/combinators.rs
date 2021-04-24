use crate::components::Entity;
use crate::query::StateFilter;

pub struct AndStateFilter<F1, F2>(F1, F2);

impl<F1, F2> AndStateFilter<F1, F2> {
	pub fn new(filter1: F1, filter2: F2) -> Self {
		Self(filter1, filter2)
	}
}

impl<F1, F2> StateFilter for AndStateFilter<F1, F2>
where
	F1: StateFilter,
	F2: StateFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.matches(entity) && self.1.matches(entity)
	}
}

pub struct OrStateFilter<F1, F2>(F1, F2);

impl<F1, F2> OrStateFilter<F1, F2> {
	pub fn new(filter1: F1, filter2: F2) -> Self {
		Self(filter1, filter2)
	}
}

impl<F1, F2> StateFilter for OrStateFilter<F1, F2>
where
	F1: StateFilter,
	F2: StateFilter,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.matches(entity) || self.1.matches(entity)
	}
}
