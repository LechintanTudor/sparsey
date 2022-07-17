//! Resource creation and management.

mod res;
mod resource;
mod resources;
mod sync_resources;
mod unsafe_resources;

pub use self::res::*;
pub use self::resource::*;
pub use self::resources::*;
pub use self::sync_resources::*;

pub(crate) use self::unsafe_resources::*;
