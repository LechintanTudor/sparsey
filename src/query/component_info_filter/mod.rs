pub use self::added::*;
pub use self::view::*;

mod added;
mod view;

use crate::components::{ComponentInfo, Entity, Ticks};

pub trait ComponentInfoFilter {
	fn matches(info: &ComponentInfo, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

pub trait QueryComponentInfoFilter {
	fn matches(&self, entity: Entity, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

impl QueryComponentInfoFilter for () {
	fn matches(&self, _: Entity, _: Ticks, _: Ticks) -> bool {
		true
	}
}
