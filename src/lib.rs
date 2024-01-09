pub mod entity;
pub mod query;
pub mod resource;

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
