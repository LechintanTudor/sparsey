pub use self::base::*;
pub use self::component_filter::*;
pub use self::component_info_filter::*;
pub use self::component_view::*;
pub use self::iter::*;

mod base;
mod component_filter;
mod component_info_filter;
mod component_view;
mod iter;

use crate::components::{Entity, Ticks};
use crate::world::{CombinedGroupInfo, QueryGroupInfo};

pub struct Include<Q, F> {
	query: Q,
	filter: F,
}

pub struct Exclude<Q, F> {
	query: Q,
	filter: F,
}

pub struct Filter<Q, F> {
	query: Q,
	filter: F,
}

pub unsafe trait Query {
	type Item;
	type Iterator: Iterator<Item = Self::Item>;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn iter(self) -> Self::Iterator;
}

fn shortest_entity_slice<'a>(slices: &[&'a [Entity]]) -> Option<&'a [Entity]> {
	let (mut shortest, others) = slices.split_first()?;

	for slice in others {
		if slice.len() < shortest.len() {
			shortest = slice;
		}
	}

	Some(shortest)
}
