use crate::components::{Entity, Ticks};

#[derive(Debug)]
pub struct SplitQueryData<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

pub trait SparseSplitQuery {
	type Item;

	unsafe fn get(
		&mut self,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

pub trait IntoSparseSplitQuery<'a> {
	type SparseSplitQuery: SparseSplitQuery;

	fn split_sparse(self) -> (Option<SplitQueryData<'a>>, Self::SparseSplitQuery);
}

pub trait DenseSplitQuery {
	type Item;

	unsafe fn get(
		&mut self,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

pub trait IntoDenseSplitQuery<'a> {
	type DenseSplitQuery: DenseSplitQuery;

	fn split_dense(self) -> (Option<SplitQueryData<'a>>, Self::DenseSplitQuery);
}

pub trait ComponentSplitQuery {
	unsafe fn includes_all(&self, entity: Entity) -> bool;

	unsafe fn excludes_all(&self, entity: Entity) -> bool;
}

pub trait IntoComponentSplitQuery<'a> {
	type ComponentSplitQuery: ComponentSplitQuery;

	fn split_component(self) -> (Option<SplitQueryData<'a>>, Self::ComponentSplitQuery);
}
