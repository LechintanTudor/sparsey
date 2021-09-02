use crate::query::{IterData, QueryBase, QueryFilter, QueryModifier};
use crate::storage::Entity;
use crate::utils::EntityIterator;

pub struct SparseIter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	data: IterData<'a>,
	base: B::SparseSplit,
	include: I::Split,
	exclude: E::Split,
	filter: F,
	index: usize,
}

impl<'a, B, I, E, F> SparseIter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	pub fn new(
		data: IterData<'a>,
		base: B::SparseSplit,
		include: I::Split,
		exclude: E::Split,
		filter: F,
	) -> Self {
		Self {
			data,
			base,
			include,
			exclude,
			filter,
			index: 0,
		}
	}
}

impl<'a, B, I, E, F> Iterator for SparseIter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	type Item = B::Item;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let entity = *self.data.entities.get(self.index)?;
			self.index += 1;

			if self.filter.matches(entity)
				&& I::includes_split(&self.include, entity)
				&& E::excludes_split(&self.exclude, entity)
			{
				let item = unsafe {
					B::get_from_sparse_split(
						&mut self.base,
						entity,
						self.data.world_tick,
						self.data.change_tick,
					)
				};

				if item.is_some() {
					return item;
				}
			}
		}
	}
}

impl<'a, B, I, E, F> EntityIterator for SparseIter<'a, B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	fn current_entity(&self) -> Option<Entity> {
		self.data.entities.get(self.index).copied()
	}
}
