use crate::components::{Component, Entity, SparseArrayView, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::query::IterInfo;
use crate::world::{GroupInfo, GroupMask, QueryGroupInfo};

pub trait ComponentFilterElement {
	fn includes(&self, entity: Entity) -> bool;
}

impl<'a, T> ComponentFilterElement for &'a Comp<'a, T>
where
	T: Component,
{
	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}
}

impl<'a, T> ComponentFilterElement for &'a CompMut<'a, T>
where
	T: Component,
{
	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}
}

impl<'a> ComponentFilterElement for SparseArrayView<'a> {
	fn includes(&self, entity: Entity) -> bool {
		self.contains(entity)
	}
}

pub trait BaseComponentFilterElement<'a>
where
	Self: ComponentFilterElement,
{
	type Split: ComponentFilterElement;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn group_info(&self) -> Option<GroupInfo>;

	fn split(self) -> (&'a [Entity], Self::Split);
}

impl<'a, T> BaseComponentFilterElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Split = SparseArrayView<'a>;

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn group_info(&self) -> Option<GroupInfo> {
		self.group_info
	}

	fn split(self) -> (&'a [Entity], Self::Split) {
		let (sparse, entities, _, _) = self.storage.split();
		(entities, sparse)
	}
}

impl<'a, T> BaseComponentFilterElement<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	type Split = SparseArrayView<'a>;

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn group_info(&self) -> Option<GroupInfo> {
		self.group_info
	}

	fn split(self) -> (&'a [Entity], Self::Split) {
		let (sparse, entities, _, _) = self.storage.split();
		(entities, sparse)
	}
}

pub trait ComponentFilter {
	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}

pub trait BaseComponentFilter<'a>
where
	Self: ComponentFilter,
{
	type Split: ComponentFilter;

	fn group_info(&self) -> Option<QueryGroupInfo>;

	fn split(self) -> (Option<IterInfo<'a>>, Self::Split);
}
