//! Manages entities and their associated components.

pub use self::comp::*;
pub use self::entities::*;
pub use self::world::*;

mod comp;
mod entities;
mod world;
