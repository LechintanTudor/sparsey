pub use self::component_filter::*;
pub use self::component_info_filter::*;
pub use self::component_view::*;
//pub use self::iter::*;

mod component_filter;
mod component_info_filter;
mod component_view;
//mod iter;

use crate::components::{Entity, Ticks};
use crate::world::{CombinedGroupInfo, QueryGroupInfo};

#[derive(Debug)]
pub struct IterData<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

impl<'a> IterData<'a> {
	pub fn new(entities: &'a [Entity], world_tick: Ticks, last_system_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			last_system_tick,
		}
	}
}

pub unsafe trait Query<'a> {
	type Item;
	type SparseSplit;
	type DenseSplit;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> CombinedGroupInfo;

	fn split_sparse(self) -> (IterData<'a>, Self::SparseSplit);

	fn split_dense(self) -> (IterData<'a>, Self::DenseSplit);

	unsafe fn get_from_sparse_split(
		sparse: &mut Self::SparseSplit,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;

	unsafe fn get_from_dense_split(
		dense: &mut Self::DenseSplit,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

pub struct Include {}

pub struct Exclude {}

pub struct Filter {}

// macro_rules! impl_query {
// 	($(($elem:ident, $idx:tt)),*) => {
// 		unsafe impl<'a, $($elem),*> Query<'a> for ($($elem,)*)
// 		where
// 			$($elem: QueryElement<'a>,)*
// 		{
// 			type Item = ($($elem::Item,)*);

// 			#[allow(unused_variables)]
// 			fn get(self, entity: Entity) -> Option<Self::Item> {
// 				Some(($(self.$idx.get(entity)?,)*))
// 			}

// 			#[allow(unused_variables)]
// 			fn contains(&self, entity: Entity) -> bool {
// 				true $(&& self.$idx.contains(entity))*
// 			}

// 			fn group_info(&self) -> Option<CombinedQueryGroupInfo> {
// 				let info = QueryGroupInfo::new() $(.with_group(self.$idx.group_info()?)?)*;
// 				CombinedQueryGroupInfo::new().include(info)
// 			}
// 		}
// 	};
// }

// impl_query!();
// impl_query!((A, 0));
// impl_query!((A, 0), (B, 1));
// impl_query!((A, 0), (B, 1), (C, 2));
// impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
