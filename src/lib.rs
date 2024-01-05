pub mod entity;
pub mod resource;

use crate::entity::EntityStorage;
use crate::resource::ResourceStorage;

#[derive(Default)]
pub struct World {
    pub entities: EntityStorage,
    pub resources: ResourceStorage,
}
