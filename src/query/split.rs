use crate::storage::Entity;

macro_rules! split_sparse {
	($(($elem:expr, $idx:tt)),+) => {
		{
			let splits = ($($elem.split(),)+);
			let entities = crate::query::split::shortest_entity_slice(&[$(splits.$idx.0),+]).unwrap();
			let sparse = ($(splits.$idx.1,)+);
			let data = ($(splits.$idx.2,)+);

			(entities, sparse, data)
		}
	};
}

macro_rules! split_dense {
	(($first_elem:expr, $first_idx:tt) $(, ($elem:expr, $idx:tt))*) => {
		{
			let (entities, _, first_data) = $first_elem.split();
			let data = (first_data, $($elem.split().2,)*);

			(entities, data)
		}
	};
}

macro_rules! split_modifier {
	($(($elem:expr, $idx:tt)),+) => {
		{
			let splits = ($({ let (entities, sparse, _) = $elem.split(); (entities, sparse) },)+);
			let entities = crate::query::split::shortest_entity_slice(&[$(splits.$idx.0),+]).unwrap();
			let sparse = ($(splits.$idx.1,)+);

			(Some(entities), sparse)
		}
	};
}

macro_rules! query_split_sparse {
	($(($elem:expr, $idx:tt)),+) => {
		{
			let splits = ($($elem.split_sparse(),)+);
			let entities = crate::query::split::shortest_entity_slice(&[$(splits.$idx.0),+]).unwrap();
			let sparse = ($(splits.$idx.1,)+);
			let data = ($(splits.$idx.2,)+);

			(entities, sparse, data)
		}
	};
}

macro_rules! query_split_dense {
	(($first_elem:expr, $first_idx:tt) $(, ($elem:expr, $idx:tt))*) => {
		{
			let (entities, first_data) = $first_elem.split_dense();
			let data = (first_data, $($elem.split_dense().1,)*);

			(entities, data)
		}
	};
}

pub(crate) fn shortest_entity_slice<'a>(slices: &[&'a [Entity]]) -> Option<&'a [Entity]> {
    slices.iter().min_by_key(|e| e.len()).copied()
}
