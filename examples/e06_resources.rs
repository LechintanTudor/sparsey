// Example 06 - Resources
// Insert and use a `Time` resource.

use sparsey::{Dispatcher, IntoSystem, Res, ResMut, Resources, World};

#[derive(Copy, Clone, Debug)]
struct Time(u32);

/// `ResMut<T>` gives us an exclusive view over a resource of type `T`.
fn update_time(mut time: ResMut<Time>) {
	time.0 += 1;
}

/// `Res<T>` gives us a shared view over a resource of type `T`.
fn print_time(time: Res<Time>) {
	println!("Time is: {}", time.0);
}

fn main() {
	let mut world = World::default();

	// `Resources` is a container used for storing objects which
	// aren't associated to any `Entity`. They are uniquely identified
	// by their `TypeId` so at most we can have one `Resource` of
	// a given type.
	let mut resources = Resources::default();

	// Add resources using the `insert` method.
	resources.insert(Time(0));

	let mut dispatcher = Dispatcher::builder()
		.add_system(update_time.system())
		.add_system(print_time.system())
		.build();

	for _ in 0..5 {
		dispatcher.run_locally(&mut world, &mut resources).unwrap();
	}
}
