pub use self::component::*;
pub use self::type_erased_sparse_set::*;
pub use self::type_erased_vec::*;

// TODO: Move to this module
pub use crate::storage::{ComponentFlags, Entity, IndexEntity, SparseArray};

mod component;
mod type_erased_sparse_set;
mod type_erased_vec;
