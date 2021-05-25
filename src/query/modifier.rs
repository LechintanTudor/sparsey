use crate::components::{Component, Entity, SparseArrayView};
use crate::dispatcher::{Comp, CompMut};
use crate::query::{ComponentView, IterData, UnfilteredComponentView};
use crate::world::CombinedGroupInfo;

pub trait QueryModifierElement<'a>
where
	Self: UnfilteredComponentView<'a>,
{
	fn into_entities(self) -> &'a [Entity] {
		let (_, entities, _, _) = self.split();
		entities
	}

	fn split_modifier(self) -> (&'a [Entity], SparseArrayView<'a>) {
		let (sparse, entities, _, _) = self.split();
		(entities, sparse)
	}
}

impl<'a, T> QueryModifierElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	// Empty
}

impl<'a, T> QueryModifierElement<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	// Empty
}

pub trait QueryModifier<'a> {
	type Split;

	fn includes(&self, entity: Entity) -> bool;

	fn excludes(&self, entity: Entity) -> bool;

	fn group_info(&self) -> CombinedGroupInfo<'a>;

	fn into_iter_data(self) -> Option<IterData<'a>>;

	fn split(self) -> (Option<IterData<'a>>, Self::Split);

	fn includes_split(split: &Self::Split, entity: Entity) -> bool;

	fn excludes_split(split: &Self::Split, entity: Entity) -> bool;
}

impl<'a, C> QueryModifier<'a> for C
where
	C: QueryModifierElement<'a>,
{
	type Split = SparseArrayView<'a>;

	fn includes(&self, entity: Entity) -> bool {
		<C as ComponentView<'a>>::contains(self, entity)
	}

	fn excludes(&self, entity: Entity) -> bool {
		!<C as ComponentView<'a>>::contains(self, entity)
	}

	fn group_info(&self) -> CombinedGroupInfo<'a> {
		let group_info = <C as ComponentView<'a>>::group_info(self);
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
		let (entities, split) = self.split_modifier();
		let iter_data = IterData::new(entities, world_tick, last_system_tick);

		(Some(iter_data), split)
	}

	fn includes_split(split: &Self::Split, entity: Entity) -> bool {
		split.contains(entity)
	}

	fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
		!split.contains(entity)
	}
}

impl<'a> QueryModifier<'a> for () {
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

macro_rules! to_sparse_array_view {
	($ident:ident) => {
		SparseArrayView<'a>
	};
}

macro_rules! impl_query_component_filter {
	($(($view:ident, $idx:tt)),+) => {
		impl<'a, $($view),+> QueryModifier<'a> for ($($view,)+)
		where
			$($view: QueryModifierElement<'a>,)+
		{
			type Split = ($(to_sparse_array_view!($view),)+);

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
				let view = self.0;
				let world_tick = view.world_tick();
				let last_system_tick = view.last_system_tick();
				let entities = view.into_entities();

				Some(IterData::new(entities, world_tick, last_system_tick))
			}

			fn split(self) -> (Option<IterData<'a>>, Self::Split) {
				split_sparse!(split_modifier, $(($view, self.$idx)),+)
			}

			fn includes_split(split: &Self::Split, entity: Entity) -> bool {
				$($view::includes_split(&split.$idx, entity))&&+
			}

			fn excludes_split(split: &Self::Split, entity: Entity) -> bool {
				$(!$view::includes_split(&split.$idx, entity))&&+
			}
		}
	};
}

impl_query_component_filter!((A, 0));
impl_query_component_filter!((A, 0), (B, 1));
impl_query_component_filter!((A, 0), (B, 1), (C, 2));
impl_query_component_filter!((A, 0), (B, 1), (C, 2), (D, 3));
