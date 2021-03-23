// Example 04 - Filters
// Create entities with `Hp` and `HpRegen` components
// and iterate filtered ComponentViews.

use sparsey::{Comp, CompMut, Dispatcher, EntityIterator, IntoSystem, Query, Resources, World};

#[derive(Copy, Clone, Debug)]
struct Hp(i32);

#[derive(Copy, Clone, Debug)]
struct HpRegen(i32);

fn print_health(mut hps: CompMut<Hp>, _hp_regens: Comp<HpRegen>) {
	use sparsey::filters::{added, changed};

	// Only get entities with `Hp` and `HpRegen` components which
	// were added after the last call to `world.clear_flags`.
	println!("Newly added:");
	for (entity, (mut hp,)) in (added(&mut hps),).iter().entities() {
		println!("{:?} => {:?}", entity, *hp);

		// Give a small `Hp` boost to newly added entities.
		hp.0 += 5;
	}
	println!();

	// Only get entities with `Hp` components which
	// were changed after the last call to `world.clear_flags`.
	println!("Hp changed:");
	for (entity, (hp,)) in (changed(&hps),).iter().entities() {
		println!("{:?} => {:?}", entity, hp);
	}
	println!();

	// Only get entities with `Hp` components which were
	// NOT changed after the last call to `world.clear_flags`.
	// Notice the `!` before `changed`.
	println!("Hp NOT changed:");
	for (entity, (hp,)) in (!changed(&hps),).iter().entities() {
		println!("{:?} => {:?}", entity, hp);
	}
	println!();
}

fn main() {
	let mut world = World::default();

	let mut dispatcher = Dispatcher::builder()
		.add_system(print_health.system())
		.build();

	dispatcher.set_up(&mut world);

	world.create((Hp(100),));
	world.create((Hp(50), HpRegen(3)));
	world.create((Hp(200), HpRegen(5)));
	world.create((Hp(300), HpRegen(7)));
	world.create(());

	let mut resources = Resources::default();

	// Run the systems multiple times.
	for i in 1..=2 {
		println!("ITERATION: {}", i);
		println!("------------------------------");
		dispatcher.run_seq(&mut world, &mut resources).unwrap();

		// Clear component flags after each iteration.
		world.clear_flags();
	}
}
