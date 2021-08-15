#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]

/// Exports functions for filtering component and resource views.
pub mod filters {
	pub use crate::query::{added, contains, maybe, mutated, updated};
	pub use crate::world::{res_added, res_changed, res_mutated};
}

/// Exports most commonly used items.
pub mod prelude {
	pub use crate::components::Entity;
	pub use crate::layout::{Layout, LayoutGroupDescriptor};
	pub use crate::query::{Query, QueryBaseModifiers, SliceQuery};
	pub use crate::systems::{
		Commands, Dispatcher, IntoLocalFn, IntoLocalSystem, IntoSystem, SystemResult,
	};
	pub use crate::utils::{ChangeTicks, EntityIterator};
	pub use crate::world::{Comp, CompMut, Res, ResMut, World};
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

/*
Commands
Res/ResMut
Comp/CompMut

ComponentStorages
	GroupedComponentStorages
	UngroupedComponentStorages

Resources

World
	EntityStorage
	ComponentStorages
	Resources

	world.create_entity_with_ticks((Position, Velocity), ChangeTicks::just_added(0));
	world.append_components(entity, (Immovable,))
	world.destroy_entity(entity)
*/
