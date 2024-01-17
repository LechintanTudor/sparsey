pub mod entity;
pub mod query;
pub mod resource;
pub mod system;

pub mod prelude {
    pub use crate::entity::{Comp, CompMut, Entities, Entity, EntityStorage, GroupLayout};
    pub use crate::query::{BuildCompoundQuery, IntoEntityIter, Query};
    pub use crate::resource::{Res, ResMut, ResourceStorage};
    pub use crate::system::{IntoSystem, Run, System};
    pub use crate::World;
}

use crate::entity::{EntityStorage, GroupLayout};
use crate::resource::ResourceStorage;

#[derive(Default, Debug)]
pub struct World {
    pub entities: EntityStorage,
    pub resources: ResourceStorage,
}

impl World {
    #[inline]
    #[must_use]
    pub fn new(layout: &GroupLayout) -> Self {
        Self {
            entities: EntityStorage::new(layout),
            resources: ResourceStorage::default(),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty() && self.resources.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.entities.clear();
        self.resources.clear();
    }

    #[inline]
    pub fn reset(&mut self) {
        self.entities.reset();
        self.resources.clear();
    }
}
