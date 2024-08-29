#![no_std]

extern crate alloc;

pub mod component;
pub mod entity;
pub mod query;
pub mod world;

pub use self::entity::Entity;
pub use self::world::World;
