#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]

//! Sparsey is a sparse set-based Entity Component System with lots of features
//! and nice syntax \~( ˘▾˘\~)
//! ```
//! use sparsey::prelude::*;
//!
//! struct Position(f32, f32);
//! struct Velocity(f32, f32);
//!
//! fn main() {
//!     let mut world = World::default();
//!     world.register::<Position>();
//!     world.register::<Velocity>();
//!
//!     world.create_entity((Position(0.0, 0.0), Velocity(1.0, 2.0)));
//!     world.create_entity((Position(0.0, 0.0), Velocity(3.0, 4.0)));
//!
//!     let (mut positions, velocities)
//!         = world.borrow::<(CompMut<Position>, Comp<Velocity>)>();
//!
//!     for (mut position, velocity) in (&mut positions, &velocities).iter() {
//!         position.0 += velocity.0;
//!         position.1 += velocity.1;
//!     }
//! }
//! ```

/// Exports functions for filtering component and resource views.
pub mod filters {
    pub use crate::query::{added, changed, contains, mutated};
    pub use crate::resources::{res_added, res_changed, res_mutated};
}

/// Exports most commonly used items.
pub mod prelude {
    pub use crate::layout::{Layout, LayoutGroupDescriptor};
    pub use crate::query::{IntoEntityIterator, Query, QueryGetModifier, SliceQuery};
    pub use crate::storage::{ChangeTicks, Entity};
    pub use crate::systems::{
        Commands, Dispatcher, IntoLocalFn, IntoLocalSystem, IntoSystem, SystemResult,
    };
    pub use crate::world::{Comp, CompMut, Res, ResMut, World};
}

/// Manages `ComponentStorage`s and `Component` grouping within a `World`.
pub mod components;
/// Describes the layout of component groups within a `World`.
pub mod layout;
/// Enables fetching and iterating components from component views.
pub mod query;
/// Enables creating and managing data which is unique within a `World`.
pub mod resources;
/// Exports functionality for managing `ComponentStorage`s.
pub mod storage;
/// Exports functionality for creating `System`s and dispatching them sequentially or in parallel.
pub mod systems;
/// Exports functionality for creating and managing `World`s.
pub mod world;

pub(crate) mod utils;
