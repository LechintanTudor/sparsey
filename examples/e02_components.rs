// Example 02 - Components
// Create entities with `Hp` and `HpRegen` components
// and iterate over the individual component sets.

use sparsey::{Comp, CompMut, Dispatcher, Entity, EntityIterator, IntoSystem, Resources, World};

// Health points of the entity.
#[derive(Copy, Clone, Debug)]
struct Hp(i32);

// Health regeneration of the entity.
#[derive(Copy, Clone, Debug)]
struct HpRegen(i32);

fn print_health(hps: CompMut<Hp>, hp_regens: Comp<HpRegen>) {
    // Iterate over all `Hp` components.
    // Very fast, as the components are tightly packed in an array.
    for hp in hps.iter() {
        println!("{:?}", hp);
    }

    println!();

    // Iterate over all `HpRegen` components and their associated `Entity`.
    // Still very fast as the components and entities are tightly packed.
    // The `entities` method is only available when the `EntityIterator`
    // trait is in scope.
    for (entity, hp_regen) in hp_regens.iter().entities() {
        println!("{:?} => {:?}", entity, hp_regen);
    }
}

fn main() {
    // `World` is a container which maps keys known as `Entities`
    // to a set of components.
    let mut world = World::default();

    // We have to register the components we want to use.
    // There is a better alternative which we'll showcase
    // in the next example.
    world.register::<Hp>();
    world.register::<HpRegen>();

    // Create a new `Entity` from a component tuple.
    let e1: Entity = world.create((Hp(100),));

    // Create a new `Entity` with multiple components.
    let e2 = world.create((Hp(100), HpRegen(2)));

    println!("e1: {:?}", e1);
    println!("e2: {:?}", e2);
    println!();

    // Create some other entities.
    world.create((Hp(200), HpRegen(5)));
    world.create((Hp(420), HpRegen(69)));
    world.create(()); // Entity with no components.

    let mut resources = Resources::default();

    let mut dispatcher = Dispatcher::builder()
        .add_system(print_health.system())
        .build();

    dispatcher.run_locally(&mut world, &mut resources);
}
