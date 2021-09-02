use crate::group::group_range;
use crate::query2::{DenseIter, IterData, QueryBase, QueryFilter, QueryModifier, SparseIter};
use crate::storage::Entity;
use crate::utils::EntityIterator;

pub enum Iter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	Sparse(SparseIter<'a, B, I, E, F>),
	Dense(DenseIter<'a, B, F>),
}

impl<'a, B, I, E, F> Iter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	pub(crate) fn new(base: B, include: I, exclude: E, filter: F) -> Self {
		let group_range = match (
			base.group_info(),
			include.group_info(),
			exclude.group_info(),
		) {
			(Some(base_info), Some(include_info), Some(exclude_info)) => {
				group_range(base_info, include_info, exclude_info)
			}
			_ => None,
		};

		match group_range {
			Some(range) => {
				let (base_data, base) = base.split_dense();

				let iter_data = base_data
					.or_else(|| include.split().0)
					.map(|iter_data| iter_data.with_range(range))
					.unwrap_or(IterData::EMPTY);

				unsafe { Iter::Dense(DenseIter::new_unchecked(iter_data, base, filter)) }
			}
			None => {
				let (base_data, base) = base.split_sparse();
				let (include_data, include) = include.split();
				let (_, exclude) = exclude.split();

				let iter_data = [base_data, include_data]
					.iter()
					.flatten()
					.min_by_key(|d| d.entities.len())
					.copied()
					.unwrap_or(IterData::EMPTY);

				Iter::Sparse(SparseIter::new(iter_data, base, include, exclude, filter))
			}
		}
	}

	/// Returns `true` if the iterator is dense.
	pub fn is_dense(&self) -> bool {
		match self {
			Self::Sparse(_) => false,
			Self::Dense(_) => true,
		}
	}
}

impl<'a, B, I, E, F> Iterator for Iter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	type Item = B::Item;

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

impl<'a, B, I, E, F> EntityIterator for Iter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
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
