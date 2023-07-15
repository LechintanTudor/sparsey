#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::unused_unit)]
//#![forbid(missing_docs)]

//! Sparsey is a sparse set-based Entity Component System.
//!
//! ```
//! use sparsey::prelude::*;
//!
//! struct Position(f32);
//! struct Velocity(f32);
//!
//! fn main() {
//!     let mut world = World::default();
//!     world.register::<Position>();
//!     world.register::<Velocity>();
//!
//!     world.create((Position(0.0),));
//!     world.create((Position(0.0), Velocity(1.0)));
//!     world.create((Position(0.0), Velocity(2.0)));
//!
//!     let resources = Resources::default();
//!
//!     sparsey::run(
//!         &world,
//!         &resources,
//!         |mut positions: CompMut<Position>, velocities: Comp<Velocity>| {
//!             (&mut positions, &velocities).for_each(|(position, velocity)| {
//!                 position.0 += velocity.0;
//!             });
//!         },
//!     );
//! }
//! ```

pub mod components;
pub mod layout;
pub mod query;
pub mod resources;
pub mod storage;
pub mod systems;
pub mod utils;
pub mod world;

/// Re-exports the most commonly used items.
pub mod prelude {
    pub use crate::layout::{Layout, LayoutGroupDescriptor};
    pub use crate::query::{BuildCompoundQuery, IntoEntityIter, Query};
    pub use crate::resources::{Res, ResMut, Resources};
    pub use crate::storage::Entity;
    pub use crate::systems::{IntoExclusiveSystem, IntoLocalSystem, IntoSystem, Schedule};
    pub use crate::world::{Comp, CompMut, Entities, World};
}

pub use self::systems::{run, run_exclusive, run_local};
