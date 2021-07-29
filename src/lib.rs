#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]

/// Exports functions used for filtering component views.
pub mod filters {
	pub use crate::query::{added, contains, maybe, mutated, updated};
}

/// Exports most commonly used items.
pub mod prelude {
	pub use crate::components::{Comp, CompMut, ComponentTicks, Entity};
	pub use crate::layout::{Layout, LayoutGroupDescriptor};
	pub use crate::query::{Query, QueryBaseModifiers, SliceQuery};
	pub use crate::resources::{Res, ResMut, Resources};
	pub use crate::systems::{
		Commands, Dispatcher, IntoLocalFn, IntoLocalSystem, IntoSystem, SystemResult,
	};
	pub use crate::utils::EntityIterator;
	pub use crate::world::World;
}

pub use self::components::*;
pub use self::group::*;
pub use self::layout::*;
pub use self::query::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::utils::*;
pub use self::world::*;

mod components;
mod group;
mod layout;
mod query;
mod resources;
mod systems;
mod utils;
mod world;
