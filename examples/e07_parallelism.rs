// Example 07 - Parallelism
// Create a movement and a health system and run them in parallel.

use rayon::ThreadPoolBuilder;
use sparsey::{
	Comp, CompMut, Dispatcher, IntoSystem, Layout, LayoutGroupDescriptor, Query, Resources, World,
};

#[derive(Copy, Clone, Debug)]
struct Position(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Velocity(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Hp(f32);

#[derive(Copy, Clone, Debug)]
struct HpRegen(f32);

fn movement(mut positions: CompMut<Position>, velocities: Comp<Velocity>) {
	for (mut pos, vel) in (&mut positions, &velocities).iter() {
		pos.0 += vel.0;
		pos.1 += vel.1;
	}
}

fn health(mut hps: CompMut<Hp>, hp_regens: Comp<HpRegen>) {
	for (mut hp, hp_regen) in (&mut hps, &hp_regens).iter() {
		hp.0 += hp_regen.0;
	}
}

fn main() {
	// Group (`Position`, `Velocity`) and (`Hp`, `HpRegen`)
	// for a performance boost when iterating over them.
	let layout = Layout::builder()
		.add_group(<(Position, Velocity)>::group())
		.add_group(<(Hp, HpRegen)>::group())
		.build();

	let mut dispatcher = Dispatcher::builder()
		.add_system(movement.system())
		.add_system(health.system())
		.build();

	let mut world = World::with_layout(&layout);
	dispatcher.set_up(&mut world);

	world.create((
		Position(0.0, 0.0),
		Velocity(1.0, 1.0),
		Hp(5.0),
		HpRegen(2.0),
	));
	world.create((
		Position(0.0, 0.0),
		Velocity(0.0, 1.0),
		Hp(6.0),
		HpRegen(1.0),
	));
	world.create((
		Position(0.0, 0.0),
		Velocity(1.0, 0.0),
		Hp(7.0),
		HpRegen(0.0),
	));

	// `max_concurrency` returns the maximum number of systems in the `Dispatcher`
	// which can run in parallel. Two systems can run in parallel if they don't have
	// conflicting parameters.
	//
	// Two parameters conflict in those cases:
	// - Comp<T>/Res<T> and CompMut<T>/ResMut<T>
	// - CompMut<T>/ResMut<T> and CompMut<T>/ResMut<T>
	let max_parallel_systems = dispatcher.max_concurrecy();
	println!("Max parallel systems: {}", max_parallel_systems);

	// Create a `rayon::ThreadPool` with as many threads as our `Dispatcher` can
	// take advantage of.
	let thread_pool = ThreadPoolBuilder::new()
		.num_threads(max_parallel_systems)
		.build()
		.unwrap();

	let mut resources = Resources::default();

	// Use `run_par` to run the systems in parallel where possible.
	dispatcher
		.run_par(&mut world, &mut resources, &thread_pool)
		.unwrap();
}
