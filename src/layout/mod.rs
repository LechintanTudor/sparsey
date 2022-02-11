//! Helper types for describing the layout of component storages within a `World`.

pub use self::component_info::*;
pub use self::group::*;
pub use self::layout::*;

pub(crate) use self::group_family::*;

mod component_info;
mod group;
mod group_family;
mod layout;
