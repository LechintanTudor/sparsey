use crate::storage::Entity;

macro_rules! ticks {
    ($elem:expr $(, $t:expr)*) => {
        ($elem.world_tick(), $elem.change_tick())
    };
}

macro_rules! split_sparse {
	($(($elem:expr, $idx:tt)),+) => {
		{
			let (world_tick, change_tick) = ticks!($($elem),+);
			let splits = ($($elem.split(),)+);

			let entities = crate::query::shortest_entity_slice(&[$(splits.$idx.0),+])
				.unwrap();
			let sparse = ($(splits.$idx.1,)+);
			let data = ($(splits.$idx.2,)+);

			let iter_data = crate::query::IterData::new(entities, world_tick, change_tick);
			(iter_data, sparse, data)
		}
	};
}

macro_rules! split_dense {
	(($first_elem:expr, $first_idx:tt) $(, ($other_elem:expr, $other_idx:tt))*) => {
		{
			let world_tick = $first_elem.world_tick();
			let change_tick = $first_elem.change_tick();
			let (entities, first_data) = {
				let (entities, _, data) = $first_elem.split();
				(entities, data)
			};

			let iter_data = crate::query::IterData::new(entities, world_tick, change_tick);
			let data = (first_data, $($other_elem.split().2),*);
			(iter_data, data)
		}
	};
}

macro_rules! split_modifier {
	($(($elem:expr, $idx:tt)),+) => {
		{
			let splits = (
				$({ let (entities, sparse, _, _) = $elem.split(); (entities, sparse) },)+
			);

			let entities = crate::query::shortest_entity_slice(&[$(splits.$idx.0),+]);
			let sparse = ($(splits.$idx.1,)+);

			(entities, sparse)
		}
	};
}

pub(crate) fn shortest_entity_slice<'a>(slices: &[&'a [Entity]]) -> Option<&'a [Entity]> {
    slices.iter().min_by_key(|e| e.len()).copied()
}
