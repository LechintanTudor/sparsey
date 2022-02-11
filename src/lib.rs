#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::unused_unit)]

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
//!     world.create((Position(0.0, 0.0), Velocity(1.0, 2.0)));
//!     world.create((Position(0.0, 0.0), Velocity(3.0, 4.0)));
//!     
//!     let mut positions = world.borrow_mut::<Position>();
//!     let velocities = world.borrow::<Velocity>();
//!
//!     (&mut positions, &velocities).for_each(|(position, velocity)| {
//!         position.0 += velocity.0;
//!         position.1 += velocity.1;
//!     });
//! }
//! ```

/// Re-exports the most commonly used items.
pub mod prelude {
    pub use crate::layout::{Layout, LayoutGroupDescriptor};
    pub use crate::query::{CompoundQuery, IntoEntityIter, QueryFilters};
    pub use crate::resources::{Res, ResMut, Resources};
    pub use crate::storage::Entity;
    pub use crate::systems::Schedule;
    pub use crate::world::{Comp, CompMut, Entities, World};
}

pub mod components;
pub mod layout;
pub mod query;
pub mod resources;
pub mod storage;
pub mod systems;
pub mod world;

pub(crate) mod utils;
