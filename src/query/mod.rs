pub use self::component_filter::*;
pub use self::element::*;
//pub use self::state_filter::*;

mod component_filter;
mod element;
//mod state_filter;

use crate::components::{Entity, Ticks};
use crate::world::CombinedQueryGroupInfo;

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
