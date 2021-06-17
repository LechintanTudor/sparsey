use crate::components::{ComponentTicks, Entity, SparseArrayView, Ticks};
use crate::query::ComponentView;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct SparseSplitComponentView<'a, T> {
	sparse: SparseArrayView<'a>,
	data: *mut T,
	ticks: *mut ComponentTicks,
}

impl<'a, T> SparseSplitComponentView<'a, T> {
	pub unsafe fn get<C>(
		&mut self,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<C::Item>
	where
		C: ComponentView<'a, Component = T>,
	{
		let index = self.sparse.get_index(entity)? as usize;
		C::get_from_parts(self.data, self.ticks, index, world_tick, last_system_tick)
	}
}

#[derive(Copy, Clone)]
pub struct DenseSplitComponentView<'a, T> {
	lifetime: PhantomData<&'a ()>,
	data: *mut T,
	ticks: *mut ComponentTicks,
}

impl<'a, T> DenseSplitComponentView<'a, T> {
	pub unsafe fn get<V>(
		&mut self,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<V::Item>
	where
		V: ComponentView<'a, Component = T>,
	{
		V::get_from_parts(self.data, self.ticks, index, world_tick, last_system_tick)
	}
}

macro_rules! split_sparse {
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let [<split_ $first_type:lower>] = crate::query::split::split_sparse($first);
			$(let [<split_ $other_type:lower>] = crate::query::split::split_sparse($other);)*

			let entities = crate::query::split::shortest_entity_slice(&[
				[<split_ $first_type:lower>].0
				$(, [<split_ $other_type:lower>].0)*
			]).unwrap();

			(
				Some(crate::query::IterData::new(entities, world_tick, last_system_tick)),
				(
					[<split_ $first_type:lower>].1,
					$([<split_ $other_type:lower>].1,)*
				)
			)
		}
	}};
}

macro_rules! split_dense {
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let (entities, [<split_ $first_type:lower>]) = crate::query::split::split_dense($first);
			$(let [<split_ $other_type:lower>] = crate::query::split::split_dense($other).1;)*

			(
				Some(crate::query::IterData::new(entities, world_tick, last_system_tick)),
				(
					[<split_ $first_type:lower>],
					$([<split_ $other_type:lower>],)*
				)
			)
		}
	}};
}

macro_rules! split_modifier {
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let [<split_ $first_type:lower>] = crate::query::split::split_modifier($first);
			$(let [<split_ $other_type:lower>] = crate::query::split::split_modifier($other);)*

			let entities = crate::query::split::shortest_entity_slice(&[
				[<split_ $first_type:lower>].0
				$(, [<split_ $other_type:lower>].0)*
			]).unwrap();

			(
				Some(crate::query::IterData::new(entities, world_tick, last_system_tick)),
				(
					[<split_ $first_type:lower>].1,
					$([<split_ $other_type:lower>].1,)*
				)
			)
		}
	}};
}

pub(crate) fn shortest_entity_slice<'a>(slices: &[&'a [Entity]]) -> Option<&'a [Entity]> {
	slices.iter().min_by_key(|e| e.len()).copied()
}

pub(crate) fn split_sparse<'a, C>(
	view: C,
) -> (&'a [Entity], SparseSplitComponentView<'a, C::Component>)
where
	C: ComponentView<'a>,
{
	let (sparse, entities, data, ticks) = view.into_parts();
	(
		entities,
		SparseSplitComponentView {
			sparse,
			data,
			ticks,
		},
	)
}

pub(crate) fn split_dense<'a, C>(
	view: C,
) -> (&'a [Entity], DenseSplitComponentView<'a, C::Component>)
where
	C: ComponentView<'a>,
{
	let (_, entities, data, ticks) = view.into_parts();
	(
		entities,
		DenseSplitComponentView {
			lifetime: PhantomData,
			data,
			ticks,
		},
	)
}

pub(crate) fn split_modifier<'a, C>(view: C) -> (&'a [Entity], SparseArrayView<'a>)
where
	C: ComponentView<'a>,
{
	let (sparse, entities, _, _) = view.into_parts();
	(entities, sparse)
}
