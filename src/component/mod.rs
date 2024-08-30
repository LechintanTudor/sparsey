//! Component storage and management.

mod component_data;
mod component_set;
mod component_sparse_set;
mod component_storage;
mod group;
mod group_info;
mod group_layout;
mod group_mask;
mod storage_mask;
mod view;

pub use self::component_data::*;
pub use self::component_set::*;
pub use self::group_info::*;
pub use self::group_layout::*;
pub use self::view::*;

pub(crate) use self::component_sparse_set::*;
pub(crate) use self::component_storage::*;
pub(crate) use self::group::*;
pub(crate) use self::group_mask::*;
pub(crate) use self::storage_mask::*;

/// Marker trait for components that can be added to entities.
pub trait Component: Send + Sync + 'static {
    // Empty
}

impl<T> Component for T
where
    T: Send + Sync + 'static,
{
    // Empty
}
