use crate::components::Entity;
use crate::query::{QueryFilter, UnfilteredComponentView};
use std::ops::Not;

pub fn contains<'a, C>(view: C) -> Contains<C>
where
	C: UnfilteredComponentView<'a>,
{
	Contains(view)
}

pub struct Contains<C>(C);

impl<'a, C> QueryFilter for Contains<C>
where
	C: UnfilteredComponentView<'a>,
{
	fn matches(&self, entity: Entity) -> bool {
		self.0.contains(entity)
	}
}

impl<C> Not for Contains<C> {
	type Output = NotContains<C>;

	fn not(self) -> Self::Output {
		NotContains(self.0)
	}
}

pub struct NotContains<C>(C);

impl<'a, C> QueryFilter for NotContains<C>
where
	C: UnfilteredComponentView<'a>,
{
	fn matches(&self, entity: Entity) -> bool {
		!self.0.contains(entity)
	}
}
