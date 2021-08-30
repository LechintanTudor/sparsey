use crate::query::ComponentView;
use crate::storage::{Entity, SparseArrayView};

macro_rules! split_sparse {
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let change_tick = $first.change_tick();
			let [<split_ $first_type:lower>] = $first.split_sparse();
			$(let [<split_ $other_type:lower>] = $first.split_sparse();)*

			let entities = crate::query::split::shortest_entity_slice(&[
				[<split_ $first_type:lower>].0
				$(, [<split_ $other_type:lower>].0)*
			]).unwrap();

			(
				Some(crate::query::IterData::new(entities, world_tick, change_tick)),
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
			let change_tick = $first.change_tick();
			let (entities, [<split_ $first_type:lower>]) = $first.split_dense();
			$(let [<split_ $other_type:lower>] = $first.split_dense().1;)*

			(
				Some(crate::query::IterData::new(entities, world_tick, change_tick)),
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
			let change_tick = $first.change_tick();
			let [<split_ $first_type:lower>] = $first.split_modifier();
			$(let [<split_ $other_type:lower>] = $first.split_modifier();)*

			let entities = crate::query::split::shortest_entity_slice(&[
				[<split_ $first_type:lower>].0
				$(, [<split_ $other_type:lower>].0)*
			]).unwrap();

			(
				Some(crate::query::IterData::new(entities, world_tick, change_tick)),
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
