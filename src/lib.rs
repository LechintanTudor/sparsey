#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]

pub mod filters {
	pub use crate::query::{added, contains, maybe, mutated, updated};
}

pub mod prelude {
	pub use crate::components::Entity;
	pub use crate::layout::{Layout, LayoutGroupDescriptor};
	pub use crate::query::{EntityIterator, Query, QueryBaseModifiers, SliceQuery};
	pub use crate::resources::{Res, ResMut, Resources};
	pub use crate::systems::{
		Commands, Dispatcher, IntoLocalFn, IntoLocalSystem, IntoSystem, SystemResult,
	};
	pub use crate::world::{Comp, CompMut, World};
}

pub use self::components::*;
pub use self::layout::*;
pub use self::query::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::world::*;

mod components;
mod layout;
mod misc;
mod query;
mod resources;
mod systems;
mod world;
