// Example 01 - Simple System
// Run 2 systems sequentially.

use sparsey::{Dispatcher, IntoSystem, Resources, World};

fn say_hello() {
	println!("Hello");
}

fn say_world() {
	println!("World");
}

fn main() {
	let mut world = World::default();
	let mut resources = Resources::default();

	// `Dispatchers` are used to schedule the execution of systems.
	let mut dispatcher = Dispatcher::builder()
		.add_system(say_hello.system()) // Systems are created from simple functions using
		.add_system(say_world.system()) // the `system` method from the `IntoSystem` trait.
		.build();

	// `run_locally` runs all the systems sequentially
	// in the order in which they were added to the `Dispatcher`.
	dispatcher.run_locally(&mut world, &mut resources).unwrap();
}
