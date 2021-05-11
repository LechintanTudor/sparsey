use crate::components::{Component, Entity, SparseArrayView, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::query::IterData;
use crate::world::{CombinedGroupInfo, GroupInfo};
use paste::paste;

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

	fn group_info(&self) -> GroupInfo;

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

	fn group_info(&self) -> GroupInfo {
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

	fn group_info(&self) -> GroupInfo {
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

	fn group_info(&self) -> CombinedGroupInfo;

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::Split);
}

impl ComponentFilter for () {
	fn includes_all(&self, _entity: Entity) -> bool {
		true
	}

	fn excludes_all(&self, _entity: Entity) -> bool {
		true
	}
}

impl<'a> BaseComponentFilter<'a> for () {
	type Split = ();

	fn group_info(&self) -> CombinedGroupInfo {
		CombinedGroupInfo::Empty
	}

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::Split) {
		(None, ())
	}
}

macro_rules! impl_filter {
	($(($elem:ident, $idx:tt)),+) => {
		impl<$($elem),+> ComponentFilter for ($($elem,)+)
		where
			$($elem: ComponentFilterElement,)+
		{
			fn includes_all(&self, entity: Entity) -> bool {
				true && $(self.$idx.includes(entity))&&+
			}

			fn excludes_all(&self, entity: Entity) -> bool {
				true && $(!self.$idx.includes(entity))&&+
			}
		}

		impl<'a, $($elem),+> BaseComponentFilter<'a> for ($($elem,)+)
		where
			$($elem: BaseComponentFilterElement<'a>,)+
		{
			type Split = ($($elem::Split,)+);

			fn group_info(&self) -> CombinedGroupInfo {
				CombinedGroupInfo::new() $(.combine(self.$idx.group_info()))+
			}

			fn split_sparse(self) -> (Option<IterData<'a>>, Self::Split) {
				sparse_split_filter!($(($elem, self.$idx)),+)
			}
		}
	};
}

macro_rules! sparse_split_filter {
	(($first_elem:ident, $first:expr)) => {{
		paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let [<elem_ $first_elem:lower>] = $first.split();

			(
				Some(IterData::new([<elem_ $first_elem:lower>].0, world_tick, last_system_tick)),
				([<elem_ $first_elem:lower>].1,),
			)
		}
	}};
	(($first_elem:ident, $first:expr), $(($other_elem:ident, $other:expr)),+) => {{
		paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let [<elem_ $first_elem:lower>] = $first.split();
			$(let [<elem_ $other_elem:lower>] = $other.split();)+

			let entities = shortest_entity_slice!(
				[<elem_ $first_elem:lower>].0,
				$([<elem_ $other_elem:lower>].0),+
			);

			(
				Some(IterData::new(entities, world_tick, last_system_tick)),
				(
					[<elem_ $first_elem:lower>].1,
					$([<elem_ $other_elem:lower>].1,)+
				)
			)
		}
	}};
}

macro_rules! shortest_entity_slice {
	($first:expr) => {
		$first
	};
	($first:expr, $($other:expr),+) => {
		shortest_entity_slice($first, shortest_entity_slice!($($other),+))
	}
}

fn shortest_entity_slice<'a>(a: &'a [Entity], b: &'a [Entity]) -> &'a [Entity] {
	if a.len() <= b.len() {
		a
	} else {
		b
	}
}

impl_filter!((A, 0));
impl_filter!((A, 0), (B, 1));
impl_filter!((A, 0), (B, 1), (C, 2));
impl_filter!((A, 0), (B, 1), (C, 2), (D, 3));
