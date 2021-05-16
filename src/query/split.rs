use crate::components::Entity;

macro_rules! split_sparse {
	($split_fn:ident, ($first_type:ident, $first:expr) $(, ($other_type:ident, $other:expr))*) => {{
		paste::paste! {
			let world_tick = $first.world_tick();
			let last_system_tick = $first.last_system_tick();
			let [<type_ $first_type:lower>] = $first.$split_fn();
			$(let [<type_ $other_type:lower>] = $other.$split_fn();)*

			let entities = shortest_entity_slice!(
				[<type_ $first_type:lower>].0
				$(, [<type_ $other_type:lower>].0)*
			);

			(
				Some(crate::query::IterData::new(entities, world_tick, last_system_tick)),
				(
					[<type_ $first_type:lower>].1,
					$([<type_ $other_type:lower>].1,)*
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
			let [<type_ $first_type:lower>] = $first.$split_fn();
			$(let [<type_ $other_type:lower>] = $other.$split_fn();)*

			let entities = [<type_ $first_type:lower>].0;

			(
				Some(crate::query::IterData::new(entities, world_tick, last_system_tick)),
				(
					[<type_ $first_type:lower>].1,
					$([<type_ $other_type:lower>].1,)*
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
		crate::query::split::shortest_entity_slice($first, shortest_entity_slice!($($other),+))
	}
}

pub(crate) fn shortest_entity_slice<'a>(a: &'a [Entity], b: &'a [Entity]) -> &'a [Entity] {
	if a.len() <= b.len() {
		a
	} else {
		b
	}
}
