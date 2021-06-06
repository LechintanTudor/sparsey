pub use self::added::*;
pub use self::combinators::*;
pub use self::contains::*;
pub use self::mutated::*;
pub use self::passthrough::*;
pub use self::updated::*;
pub use self::view::*;

mod added;
mod combinators;
mod contains;
mod mutated;
mod passthrough;
mod updated;
mod view;

use crate::components::{ComponentTicks, Entity, Ticks};

pub trait ComponentInfoFilter {
	fn matches(info: Option<&ComponentTicks>, world_tick: Ticks, last_system_tick: Ticks) -> bool;
}

pub trait QueryFilter {
	fn matches(&self, entity: Entity) -> bool;
}
