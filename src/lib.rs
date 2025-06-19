//! Sparsey is an [Entity Component System (ECS)](https://www.geeksforgeeks.org/sparse-set/)
//! based on [sparse sets](https://www.geeksforgeeks.org/sparse-set/).
//!
//! # Example
//!
//! ```rust
//! use sparsey::World;
//!
//! struct Position(i32, i32);
//! struct Velocity(i32, i32);
//!
//! fn main() {
//!     let mut world = World::builder()
//!         .register::<Position>()
//!         .register::<Velocity>()
//!         .build();
//!
//!     world.create((Position(0, 0), Velocity(1, 2)));
//!     world.create((Position(0, 0), Velocity(2, 3)));
//!     
//!     world.for_each::<(&mut Position, &Velocity)>(|(position, velocity)| {
//!         position.0 += velocity.0;
//!         position.1 += velocity.1;
//!     });
//! }
//! ```
//!
//! # Features
//!
//! - `std` (on by default): link to the `std` crate.
//! - `parallel`: enable parallel iterators.
//!
//! # Usage
//!
//! The most important items from Sparsey crate are [`World`] and [`Entity`],
//! which are re-exported at the root of the crate for easy access. A [`World`]
//! is a collection of entities and components thas supports
//! create/read/update/delete (CRUD) operations, while an [`Entity`] is a
//! versioned index used to reference components within a [`World`].

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod component;
pub mod entity;
pub mod query;
pub mod world;

pub use self::entity::Entity;
pub use self::world::World;
