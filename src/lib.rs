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

#![forbid(missing_docs)]

pub mod entity;
pub mod query;
pub mod resource;
pub mod system;

/// Re-exports the most commonly used items.
pub mod prelude {
    pub use crate::entity::{Comp, CompMut, Entities, Entity, EntityStorage, GroupLayout};
    pub use crate::query::{BuildCompoundQuery, IntoEntityIter, Query};
    pub use crate::resource::{Res, ResMut, ResourceStorage};
    pub use crate::system::{IntoSystem, Run, System};
    pub use crate::World;
}

use crate::entity::{EntityStorage, GroupLayout};
use crate::resource::ResourceStorage;

/// Storage for entities and resources.
#[derive(Default, Debug)]
pub struct World {
    /// Storage for entities.
    pub entities: EntityStorage,
    /// Storage for resources.
    pub resources: ResourceStorage,
}

impl World {
    /// Creates a new world with the given group layout.
    #[inline]
    #[must_use]
    pub fn new(layout: &GroupLayout) -> Self {
        Self {
            entities: EntityStorage::new(layout),
            resources: ResourceStorage::default(),
        }
    }

    /// Returns whether the world contains no entities and no resources.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty() && self.resources.is_empty()
    }

    /// Removes all entities and all resources from the storage.
    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
        self.resources.clear();
    }

    /// Removes all entities and all resources from the storage and resets the entity allocator.
    ///
    /// After this call, the storage is allowed to return previously allocated entities.
    #[inline]
    pub fn reset(&mut self) {
        self.entities.reset();
        self.resources.clear();
    }
}
