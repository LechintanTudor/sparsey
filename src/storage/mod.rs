//! Sparse set-based storages for entities and components.

pub use self::component::*;
pub use self::entity::*;
pub use self::entity_storage::*;
pub use self::sparse_array::*;

pub(crate) use self::component_storage::*;

mod component;
mod component_storage;
mod entity;
mod entity_storage;
mod sparse_array;
