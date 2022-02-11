//! Resource creation and management.

pub use self::res::*;
pub use self::resource::*;
pub use self::resources::*;
pub use self::sync_resources::*;

pub(crate) use self::unsafe_resources::*;

mod res;
mod resource;
mod resources;
mod sync_resources;
mod unsafe_resources;
