//! Helper types for describing the layout of component storages within a
//! [`World`](crate::world::World).

mod component_info;
mod group;
mod group_family;
mod layout;

pub use self::component_info::*;
pub use self::group::*;
pub(crate) use self::group_family::*;
pub use self::layout::*;
