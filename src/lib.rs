#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]

//! Sparsey is a sparse set-based Entity Component System with lots of features
//! and nice syntax \~( ˘▾˘\~) ```
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
    pub use crate::query::{added, changed, contains, maybe, mutated};
    pub use crate::resources::{res_added, res_changed, res_mutated};
}

/// Exports most commonly used items.
pub mod prelude {
    pub use crate::layout::{Layout, LayoutGroupDescriptor};
    pub use crate::query::{Query, QueryBaseModifiers, SliceQuery};
    pub use crate::storage::Entity;
    pub use crate::systems::{
        Commands, Dispatcher, IntoLocalFn, IntoLocalSystem, IntoSystem, SystemResult,
    };
    pub use crate::utils::{ChangeTicks, IntoEntityIterator};
    pub use crate::world::{Comp, CompMut, Res, ResMut, World};
}

pub use self::components::*;
pub use self::group::*;
pub use self::layout::*;
pub use self::query::*;
pub use self::resources::*;
pub use self::storage::*;
pub use self::systems::*;
pub use self::utils::*;
pub use self::world::*;

mod components;
mod group;
mod layout;
mod query;
mod resources;
mod storage;
mod systems;
mod utils;
mod world;
