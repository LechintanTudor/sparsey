use crate::components::Entity;
use crate::query::{
	DenseIter, EntityIterator, IterData, QueryBase, QueryFilter, QueryGroupInfo, QueryModifier,
	SparseIter,
};

pub enum Iter<'a, Q, I, E, F>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	Sparse(SparseIter<'a, Q, I, E, F>),
	Dense(DenseIter<'a, Q, F>),
}

impl<'a, Q, I, E, F> Iter<'a, Q, I, E, F>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	pub(crate) fn new(query: Q, include: I, exclude: E, filter: F) -> Self {
		let group_info = QueryGroupInfo::new(
			query.group_info(),
			include.group_info(),
			exclude.group_info(),
		);

		match group_info.group_range() {
			Some(range) => {
				let (query_data, query) = query.split_dense();
				let include_data = include.into_iter_data();

				let iter_data = query_data
					.unwrap_or_else(|| include_data.unwrap())
					.with_range(range);

				unsafe { Iter::Dense(DenseIter::new_unchecked(iter_data, query, filter)) }
			}
			None => {
				let (query_data, query) = query.split_sparse();
				let (include_data, include) = include.split();
				let (_, exclude) = exclude.split();

				let iter_data = [query_data, include_data]
					.iter()
					.flat_map(|d| d)
					.min_by_key(|d| d.entities().len())
					.copied()
					.unwrap_or(IterData::empty());

				Iter::Sparse(SparseIter::new(iter_data, query, include, exclude, filter))
			}
		}
	}

	pub fn is_dense(&self) -> bool {
		match self {
			Self::Sparse(_) => false,
			Self::Dense(_) => true,
		}
	}
}

impl<'a, Q, I, E, F> Iterator for Iter<'a, Q, I, E, F>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	type Item = Q::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Self::Sparse(sparse) => sparse.next(),
			Self::Dense(dense) => dense.next(),
		}
	}

	fn for_each<Func>(self, f: Func)
	where
		Self: Sized,
		Func: FnMut(Self::Item),
	{
		match self {
			Self::Sparse(sparse) => sparse.for_each(f),
			Self::Dense(dense) => dense.for_each(f),
		}
	}
}

impl<'a, Q, I, E, F> EntityIterator for Iter<'a, Q, I, E, F>
where
	Q: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	fn current_entity(&self) -> Option<Entity> {
		match self {
			Self::Sparse(sparse) => sparse.current_entity(),
			Self::Dense(dense) => dense.current_entity(),
		}
	}
}
