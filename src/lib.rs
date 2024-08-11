//! Entity Component System based on sparse sets.
//!
//! ```rust,no_test
//! use sparsey::prelude::*;
//!
//! struct Position(f32);
//! struct Velocity(f32);
//!
//! fn main() {
//!     let mut entities = EntityStorage::default();
//!     entities.register::<Position>();
//!     entities.register::<Velocity>();
//!
//!     entities.create((Position(0.0),));
//!     entities.create((Position(0.0), Velocity(1.0)));
//!     entities.create((Position(0.0), Velocity(2.0)));
//!
//!     entities.run(|mut positions: CompMut<Position>, velocities: Comp<Velocity>| {
//!         (&mut positions, &velocities).for_each(|(position, velocity)| {
//!             position.0 += velocity.0;
//!         });
//!    });
//! }
//! ```

pub mod entity;
pub mod query;
pub mod util;
