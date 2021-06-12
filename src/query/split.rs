use crate::components::Entity;

macro_rules! split_sparse {
	($split_fn:ident, ($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let [<split_ $first_type:lower>] = $first.$split_fn();
			$(let [<split_ $other_type:lower>] = $other.$split_fn();)*

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
	($split_fn:ident, ($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let (entities, [<split_ $first_type:lower>]) = $first.$split_fn();
			$(let [<split_ $other_type:lower>] = $other.$split_fn().1;)*

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

pub(crate) fn shortest_entity_slice<'a>(slices: &[&'a [Entity]]) -> Option<&'a [Entity]> {
	slices.iter().min_by_key(|e| e.len()).copied()
}
