use crate::storage::Entity;

macro_rules! split_sparse {
	() => {
		(None, ())
	};
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let change_tick = $first.change_tick();
			let [<split_ $first_type:lower>] = $first.split().into_sparse_split();
			$(let [<split_ $other_type:lower>] = $other.split().into_sparse_split();)*

			let entities = crate::query::shortest_entity_slice(&[
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
	() => {
		(None, ())
	};
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let change_tick = $first.change_tick();
			let (entities, [<split_ $first_type:lower>]) = $first.split().into_dense_split();
			$(let [<split_ $other_type:lower>] = $other.split().into_dense_split().1;)*

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
	() => {
		(None, ())
	};
	(($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let change_tick = $first.change_tick();
			let [<split_ $first_type:lower>] = $first.split().into_modifier_split();
			$(let [<split_ $other_type:lower>] = $other.split().into_modifier_split();)*

			let entities = crate::query::shortest_entity_slice(&[
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
