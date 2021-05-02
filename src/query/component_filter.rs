use crate::components::{Component, Entity, SparseArrayView, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::world::{GroupInfo, GroupMask, QueryGroupInfo};

pub struct QueryInfo<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

pub trait ComponentFilterElement {
	fn includes(&self) -> bool;
}

pub trait ComponentFilter {
	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}

pub trait ComponentFilterQuery<'a>
where
	Self: ComponentFilter,
{
	type Split: ComponentFilter;

	fn split(self) -> Option<(QueryInfo<'a>, Self::Split)>;

	fn group_info(&self) -> Option<QueryGroupInfo<'a>>;
}

pub trait ComponentFilterQueryElement<'a>
where
	Self: ComponentFilterElement,
{
	fn split(self) -> (SparseArrayView<'a>, &'a [Entity]);

	fn group_info(&self) -> GroupInfo;
}
