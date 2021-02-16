pub use self::component::*;
pub use self::sparse_set_ref::*;
pub use self::type_erased_sparse_set::*;
pub use self::type_erased_vec::*;

// TODO: Move to this module
pub use crate::storage::{ComponentFlags, ComponentRefMut, Entity, IndexEntity, SparseArray};

mod component;
mod sparse_set_ref;
mod type_erased_sparse_set;
mod type_erased_vec;
