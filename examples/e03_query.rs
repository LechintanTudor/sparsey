// Example 03 - Query
// Create entities with `Hp` and `HpRegen` components
// and iterate over multiple component views.

use sparsey::{Comp, CompMut, Dispatcher, EntityIterator, IntoSystem, Query, Resources, World};

#[derive(Copy, Clone, Debug)]
struct Hp(i32);

#[derive(Copy, Clone, Debug)]
struct HpRegen(i32);

// `CompMut<T>` gives us an exclusive view over all components of type `T`.
fn print_health(mut hps: CompMut<Hp>, hp_regens: Comp<HpRegen>) {
	// Iterate over the components of entities which have both `Hp` and `HpRegen`.
	// The `iter` method for tuples is only available when the `Query` trait
	// is in scope.
	for (hp, hp_regen) in (&hps, &hp_regens).iter() {
		println!("{:?}, {:?}", hp, hp_regen);
	}

	println!();

	// Increase the `Hp` of all entities with `HpRegen`.
	for (mut hp, hp_regen) in (&mut hps, &hp_regens).iter() {
		hp.0 += hp_regen.0;
	}

	// Show the updated components.
	for (entity, (hp, hp_regen)) in (&hps, &hp_regens).iter().entities() {
		println!("{:?} => {:?}, {:?}", entity, hp, hp_regen);
	}

	println!();

	// Get the second `Entity` in the `Hp` storage.
	let entity = hps.entities()[1];

	// `get` can be used to fetch components from multiple storages
	// when you know the `Entity`.
	if let Some((hp, hp_regen)) = (&hps, &hp_regens).get(entity) {
		println!("{:?} => {:?}, {:?}", entity, hp, hp_regen);
	}
}

fn main() {
	let mut world = World::default();

	// Create the `Dispatcher` before adding any entities to the `World`.
	let mut dispatcher = Dispatcher::builder()
		.add_system(print_health.system())
		.build();

	// `set_up` registers all component types used by the added systems.
	dispatcher.set_up(&mut world);

	world.create((Hp(100),));
	world.create((Hp(50), HpRegen(3)));
	world.create((Hp(200), HpRegen(5)));
	world.create((Hp(300), HpRegen(7)));
	world.create(());

	let mut resources = Resources::default();

	dispatcher.run_seq(&mut world, &mut resources).unwrap();
}
