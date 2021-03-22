// Example 01 - Simple System
// Run 2 systems sequentially.

use sparsey::{Dispatcher, IntoSystem, Resources, SystemResult, World};

// Systems can return nothing...
fn say_hello() {
	println!("Hello");
}

/// ...or return a `SystemResult`.
fn say_world() -> SystemResult {
	println!("World");
	Ok(())
}

fn main() {
	let mut world = World::default();
	let mut resources = Resources::default();

	// `Dispatchers` are used to schedule the execution of systems.
	// Systems are created from simple functions using
	// the `system` method from the `IntoSystem` trait.
	let mut dispatcher = Dispatcher::builder()
		.add_system(say_hello.system())
		.add_system(say_world.system())
		.build();

	// `run_locally` runs all the systems sequentially
	// in the order in which they were added to the `Dispatcher`.
	dispatcher.run_locally(&mut world, &mut resources).unwrap();
}
