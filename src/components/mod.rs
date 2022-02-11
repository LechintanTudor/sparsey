//! Manages component storages and handles component grouping.

pub use self::component_set::*;
pub use self::component_storages::*;
pub use self::group_info::*;

pub(crate) use self::group::*;
pub(crate) use self::masks::*;

mod component_set;
mod component_storages;
mod group;
mod group_info;
mod masks;
