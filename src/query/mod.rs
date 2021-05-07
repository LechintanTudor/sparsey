pub use self::component_filter::*;
pub use self::element::*;
pub use self::info_filter::*;

mod component_filter;
mod element;
mod info_filter;

use crate::components::{Entity, Ticks};
use crate::world::{CombinedQueryGroupInfo, QueryGroupInfo};

#[derive(Debug)]
pub struct IterData<'a> {
	entities: &'a [Entity],
	world_tick: Ticks,
	last_system_tick: Ticks,
}

pub unsafe trait Query<'a> {
	type Item;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> Option<CombinedQueryGroupInfo>;
}

macro_rules! impl_query {
	($(($elem:ident, $idx:tt)),*) => {
		unsafe impl<'a, $($elem),*> Query<'a> for ($($elem,)*)
		where
			$($elem: QueryElement<'a>,)*
		{
			type Item = ($($elem::Item,)*);

			#[allow(unused_variables)]
			fn get(self, entity: Entity) -> Option<Self::Item> {
				Some(($(self.$idx.get(entity)?,)*))
			}

			#[allow(unused_variables)]
			fn contains(&self, entity: Entity) -> bool {
				true $(&& self.$idx.contains(entity))*
			}

			fn group_info(&self) -> Option<CombinedQueryGroupInfo> {
				let info = QueryGroupInfo::new() $(.with_group(self.$idx.group_info()?)?)*;
				CombinedQueryGroupInfo::new().include(info)
			}
		}
	};
}

impl_query!();
impl_query!((A, 0));
impl_query!((A, 0), (B, 1));
impl_query!((A, 0), (B, 1), (C, 2));
impl_query!((A, 0), (B, 1), (C, 2), (D, 3));
