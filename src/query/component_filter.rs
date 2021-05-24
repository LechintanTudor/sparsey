use crate::components::{Component, Entity, SparseArrayView, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::query::IterData;
use crate::world::{CombinedGroupInfo, GroupInfo};

pub trait ComponentFilter<'a> {
	type Split;

	fn includes(&self, entity: Entity) -> bool;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn group_info(&self) -> GroupInfo<'a>;

	fn into_entities(self) -> &'a [Entity];

	fn split(self) -> (&'a [Entity], Self::Split);

	fn includes_split(split: &Self::Split, entity: Entity) -> bool;
}

impl<'a, T> ComponentFilter<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Split = SparseArrayView<'a>;

	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn into_entities(self) -> &'a [Entity] {
		self.storage.entities()
	}

	fn split(self) -> (&'a [Entity], Self::Split) {
		let (sparse, entities, _, _) = self.storage.split();
		(entities, sparse)
	}

	fn includes_split(split: &Self::Split, entity: Entity) -> bool {
		split.contains(entity)
	}
}

impl<'a, T> ComponentFilter<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	type Split = SparseArrayView<'a>;

	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn group_info(&self) -> GroupInfo<'a> {
		self.group_info
	}

	fn into_entities(self) -> &'a [Entity] {
		self.storage.entities()
	}

	fn split(self) -> (&'a [Entity], Self::Split) {
		let (sparse, entities, _, _) = self.storage.split();
		(entities, sparse)
	}

	fn includes_split(split: &Self::Split, entity: Entity) -> bool {
		split.contains(entity)
	}
}

pub trait QueryComponentFilter<'a> {
	type Split;

	fn includes(&self, entity: Entity) -> bool;

	fn excludes(&self, entity: Entity) -> bool;

	fn group_info(&self) -> CombinedGroupInfo<'a>;

	fn into_iter_data(self) -> Option<IterData<'a>>;

	fn split(self) -> (Option<IterData<'a>>, Self::Split);

	fn includes_split(split: &Self::Split, entity: Entity) -> bool;

	fn excludes_split(split: &Self::Split, entity: Entity) -> bool;
}

impl<'a, F> QueryComponentFilter<'a> for F
where
	F: ComponentFilter<'a>,
{
	type Split = <F as ComponentFilter<'a>>::Split;

	fn includes(&self, entity: Entity) -> bool {
		<F as ComponentFilter<'a>>::includes(self, entity)
	}

	fn excludes(&self, entity: Entity) -> bool {
		!<F as ComponentFilter<'a>>::includes(self, entity)
	}

	fn group_info(&self) -> CombinedGroupInfo<'a> {
		let group_info = <F as ComponentFilter<'a>>::group_info(self);
		CombinedGroupInfo::from_group_info(group_info)
	}

	fn into_iter_data(self) -> Option<IterData<'a>> {
		let world_tick = self.world_tick();
		let last_system_tick = self.last_system_tick();
		let entities = self.into_entities();

		Some(IterData::new(entities, world_tick, last_system_tick))
	}

	fn split(self) -> (Option<IterData<'a>>, Self::Split) {
		let world_tick = self.world_tick();
		let last_system_tick = self.last_system_tick();
		let (entities, split) = <F as ComponentFilter<'a>>::split(self);
		let iter_data = IterData::new(entities, world_tick, last_system_tick);

		(Some(iter_data), split)
	}

	fn includes_split(split: &Self::Split, entity: Entity) -> bool {
		<F as ComponentFilter<'a>>::includes_split(split, entity)
	}

	fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
		!<F as ComponentFilter<'a>>::includes_split(split, entity)
	}
}

impl<'a> QueryComponentFilter<'a> for () {
	type Split = ();

	fn includes(&self, _: Entity) -> bool {
		true
	}

	fn excludes(&self, _: Entity) -> bool {
		true
	}

	fn group_info(&self) -> CombinedGroupInfo<'a> {
		CombinedGroupInfo::Empty
	}

	fn into_iter_data(self) -> Option<IterData<'a>> {
		None
	}

	fn split(self) -> (Option<IterData<'a>>, Self::Split) {
		(None, ())
	}

	fn includes_split(_: &Self::Split, _: Entity) -> bool {
		true
	}

	fn excludes_split(_: &Self::Split, _: Entity) -> bool {
		true
	}
}

macro_rules! impl_query_component_filter {
	($(($filter:ident, $idx:tt)),+) => {
		impl<'a, $($filter),+> QueryComponentFilter<'a> for ($($filter,)+)
		where
			$($filter: ComponentFilter<'a>,)+
		{
			type Split = ($($filter::Split,)+);

			fn includes(&self, entity: Entity) -> bool {
				$(self.$idx.includes(entity))&&+
			}

			fn excludes(&self, entity: Entity) -> bool {
				$(!self.$idx.includes(entity))&&+
			}

			fn group_info(&self) -> CombinedGroupInfo<'a> {
				CombinedGroupInfo::Empty $(.combine(self.$idx.group_info()))+
			}

			fn into_iter_data(self) -> Option<IterData<'a>> {
				let filter = self.0;
				let world_tick = filter.world_tick();
				let last_system_tick = filter.last_system_tick();
				let entities = filter.into_entities();

				Some(IterData::new(entities, world_tick, last_system_tick))
			}

			fn split(self) -> (Option<IterData<'a>>, Self::Split) {
				split_sparse!(split, $(($filter, self.$idx)),+)
			}

			fn includes_split(split: &Self::Split, entity: Entity) -> bool {
				$($filter::includes_split(&split.$idx, entity))&&+
			}

			fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
				$(!$filter::includes_split(&split.$idx, entity))&&+
			}
		}
	};
}

impl_query_component_filter!((A, 0));
impl_query_component_filter!((A, 0), (B, 1));
impl_query_component_filter!((A, 0), (B, 1), (C, 2));
impl_query_component_filter!((A, 0), (B, 1), (C, 2), (D, 3));
