use crate::components::Entity;
use crate::query::{EntityIterator, IterData, QueryBase, QueryFilter};

pub struct DenseIter<'a, Q, F>
where
	Q: QueryBase<'a>,
	F: QueryFilter,
{
	data: IterData<'a>,
	index: usize,
	query: Q::DenseSplit,
	filter: F,
}

impl<'a, Q, F> DenseIter<'a, Q, F>
where
	Q: QueryBase<'a>,
	F: QueryFilter,
{
	pub unsafe fn new_unchecked(data: IterData<'a>, query: Q::DenseSplit, filter: F) -> Self {
		Self {
			data,
			index: 0,
			query,
			filter,
		}
	}
}

impl<'a, Q, F> Iterator for DenseIter<'a, Q, F>
where
	Q: QueryBase<'a>,
	F: QueryFilter,
{
	type Item = Q::Item;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let entity = *self.data.entities().get(self.index)?;
			let index = self.index;
			self.index += 1;

			if self.filter.matches(entity) {
				let item = unsafe {
					Q::get_from_dense_split(
						&mut self.query,
						index,
						self.data.world_tick(),
						self.data.last_system_tick(),
					)
				};

				if item.is_some() {
					return item;
				}
			}
		}
	}
}

impl<'a, Q, F> EntityIterator for DenseIter<'a, Q, F>
where
	Q: QueryBase<'a>,
	F: QueryFilter,
{
	fn current_entity(&self) -> Option<Entity> {
		self.data.entities().get(self.index).copied()
	}
}
