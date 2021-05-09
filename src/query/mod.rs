pub use self::component_filter::*;
pub use self::element::*;
pub use self::info_filter::*;
pub use self::iter::*;

mod component_filter;
mod element;
mod info_filter;
mod iter;

use crate::components::{Entity, Ticks};
use crate::world::{CombinedQueryGroupInfo, QueryGroupInfo};

#[derive(Debug)]
pub struct IterInfo<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

pub unsafe trait Query<'a> {
	type Item;
	type SparseSplit;
	type DenseSplit;
	type Include: BaseComponentFilter<'a>;
	type Exclude: BaseComponentFilter<'a>;
	type Filter: InfoFilter;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn includes(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<CombinedQueryGroupInfo>;

	fn split_sparse(
		self,
	) -> (
		IterInfo<'a>,
		Self::SparseSplit,
		<Self::Include as BaseComponentFilter<'a>>::Split,
		<Self::Exclude as BaseComponentFilter<'a>>::Split,
		Self::Filter,
	);

	fn split_dense(self) -> (IterInfo<'a>, Self::DenseSplit, Self::Filter);

	unsafe fn get_from_sparse_split(
		sparse: &mut Self::SparseSplit,
		include: &<Self::Include as BaseComponentFilter<'a>>::Split,
		exclude: &<Self::Exclude as BaseComponentFilter<'a>>::Split,
		filter: &Self::Filter,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	);

	unsafe fn get_from_dense_split(
		dense: &mut Self::DenseSplit,
		filter: &Self::Filter,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	);
}

pub trait SimpleQuery<'a>
where
	Self: Query<'a, Include = (), Exclude = ()>,
{
}

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
