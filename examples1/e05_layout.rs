// Example 05 - Layout
// Create a movement and collision system and
// use `Layout` to optimize iteration speed.

use sparsey::{
	Comp, CompMut, Dispatcher, Entity, IntoSystem, Layout, LayoutGroupDescriptor, Query, Resources,
	SystemResult, UnfilteredQuery, World,
};

const ROOM_HALF_WIDTH: f32 = 50.0;
const ROOM_HALF_HEIGHT: f32 = 50.0;

// X and Y centered position.
#[derive(Copy, Clone, Debug)]
struct Position(f32, f32);

// Half-width and half-height.
#[derive(Copy, Clone, Debug)]
struct Collider(f32, f32);

// X and Y velocity.
#[derive(Copy, Clone, Debug)]
struct Velocity(f32, f32);

// Update the `Position` of the entity and set its
// `Velocity` to zero if the entity is out of bounds.
fn handle_collisions(
	mut positions: CompMut<Position>,
	mut velocities: CompMut<Velocity>,
	colliders: Comp<Collider>,
) -> SystemResult {
	// Check if `Position` and `Velocity` form a group. Should be TRUE.
	println!(
		"(Position, Velocity): {}",
		(&positions, &velocities).is_grouped()
	);

	// Check if `Position`, `Velocity` and `Collider` form a group. Should be TRUE.
	println!(
		"(Position, Velocity, Collider): {}",
		(&positions, &velocities, &colliders).is_grouped()
	);

	// Check if `Position`, and `Collider` form a group. Should be FALSE.
	println!(
		"(Position, Collider): {}",
		(&positions, &colliders).is_grouped()
	);

	// Very fast iterations because the components are tightly packed and ordered.
	for (mut pos, vel) in (&mut positions, &velocities).iter() {
		pos.0 += vel.0;
		pos.1 += vel.1;
	}

	// Very fast iterations because the components are tightly packed and ordered.
	for (pos, col, mut vel) in (&positions, &colliders, &mut velocities).iter() {
		// Check if the entity is outside the room.
		if pos.0 - col.0 <= -ROOM_HALF_WIDTH
			|| pos.0 + col.0 >= ROOM_HALF_WIDTH
			|| pos.1 - col.1 <= -ROOM_HALF_HEIGHT
			|| pos.1 + col.1 >= ROOM_HALF_HEIGHT
		{
			*vel = Velocity(0.0, 0.0);
		}
	}

	// Grouped storages store the components and entities in tightly packed ordered arrays,
	// meaning we can get slices to those components and entities, which is very useful.
	// To access those slices we use methods available through the `UnfilteredQuery` trait.

	// Get `Position` and `Velocity` slices.
	let _: (&[Position], &[Velocity]) = (&positions, &velocities).slice()?;

	// Get all entities with `Position` and `Velocity` components.
	let _: &[Entity] = (&positions, &velocities).entities()?;

	// Get `Entity` and component slices at the same time.
	let _: (&[Entity], (&[Position], &[Velocity])) = (&positions, &velocities).slice_entities()?;

	Ok(())
}

fn main() {
	// Layouts provide a way to optimize iterations for queries
	// which ask specifically for all components from a given group.
	// Grouping a set of components provides the fastest possible way
	// to iterate over them, the trade-off being an increased cost
	// when inserting or removing components from grouped storages.
	//
	// A component may belong to multiple groups as long as one group
	// is completely contained within the other. In our case `Position`
	// and `Velocity` belong to two different groups.
	//
	// Grouping is handled automatically by the `World`, so the user
	// doesn't have to modify their existing systems to use groups.
	// Setting a `Layout` should be done right after creating the
	// `World`.
	let layout = Layout::builder()
		.add_group(<(Position, Velocity)>::group())
		.add_group(<(Position, Velocity, Collider)>::group())
		.build();

	// Create a `World` and set its `Layout`.
	// Alternatively, use `World::with_layout`.
	let mut world = World::default();
	world.set_layout(&layout);

	let mut dispatcher = Dispatcher::builder()
		.add_system(handle_collisions.system())
		.build();

	dispatcher.set_up(&mut world);

	let mut resources = Resources::default();

	dispatcher.run_seq(&mut world, &mut resources).unwrap();
}
