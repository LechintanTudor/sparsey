#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::unused_unit)]

/// Exports functions for filtering component and resource views.
pub mod filters {
	// pub use crate::query::{added, changed, contains, maybe, mutated};
	pub use crate::world::{res_added, res_changed, res_mutated};
}

/// Exports most commonly used items.
pub mod prelude {
	pub use crate::layout::{Layout, LayoutGroupDescriptor};
	//pub use crate::query::{Query, QueryBaseModifiers, SliceQuery};
	pub use crate::storage::Entity;
	pub use crate::systems::{
		Commands, Dispatcher, IntoLocalFn, IntoLocalSystem, IntoSystem, SystemResult,
	};
	pub use crate::utils::ChangeTicks;
	pub use crate::world::{Comp, CompMut, Res, ResMut, World};
}

pub use self::components::*;
pub use self::group::*;
pub use self::layout::*;
pub use self::query2::*;
pub use self::resources::*;
pub use self::storage::*;
pub use self::systems::*;
pub use self::utils::*;
pub use self::world::*;

mod components;
mod group;
mod layout;
mod query2;
mod resources;
mod storage;
mod systems;
mod utils;
mod world;
