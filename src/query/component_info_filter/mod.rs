pub use self::added::*;
pub use self::combinators::*;
pub use self::passthrough::*;
pub use self::view::*;

mod added;
mod combinators;
mod passthrough;
mod view;

use crate::components::{ComponentInfo, Entity, Ticks};

pub trait ComponentInfoFilter {
	fn matches(info: Option<&ComponentInfo>, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

pub trait QueryComponentInfoFilter {
	fn matches(&self, entity: Entity) -> bool;
}
