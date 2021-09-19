# Sparsey
[![Crates.io](https://img.shields.io/crates/v/sparsey)](https://crates.io/crates/sparsey)
[![Documentation](https://docs.rs/sparsey/badge.svg)](https://docs.rs/sparsey)

Sparsey is a sparse set-based Entity Component System with lots of features and nice syntax \~( ˘▾˘\~)
<br />
Check out the [Sparsey Cheat Sheet](/guides/cheat_sheet.md) and [examples](/examples/) to get started!

# Example 
```rust
use sparsey::prelude::*;

/// Components are Send + Sync + 'static types.
struct Position(f32);
struct Velocity(f32);
struct Immovable;

// Sets the Velocity of Immovable entities to zero.
fn update_velocity(mut vel: CompMut<Velocity>, imv: Comp<Immovable>) {
    for mut vel in (&mut vel).include(&imv).iter() {
        vel.0 = 0.0;
    }
}

// Adds the Velocity of each entity to its Position. 
fn update_position(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
    for (mut pos, vel) in (&mut pos, &vel).iter() {
        pos.0 += vel.0;
    }
} 

fn main() {
    // Create a World and register the components we want to use.
    let mut world = World::default();
    world.register::<Position>();
    world.register::<Velocity>();

    /// Create some entities.
    world.create_entity((Position(0.0), Velocity(1.0)));
    world.create_entity((Position(0.0), Velocity(2.0)));
    world.create_entity((Position(0.0), Velocity(3.0), Immovable));

    /// Create a Dispatcher to run our systems.
    let mut dispatcher = Dispatcher::builder()
        .add_system(update_velocity.system())
        .add_system(update_position.system())
        .build();

    /// Run the systems 3 times.
    for _ in 0..3 {
        dispatcher.run_seq(&mut world).unwrap();
        world.increment_tick().unwrap();
    }
}
```

# Features
## Systems
Systems are functions that have Component views and Resource views as parameters.
```rust
fn movement(mut pos: CompMut<Position>, vel: Comp<Velocity>, time: Res<Time>) {
    for (mut pos, vel) in (&mut pos, &vel).iter() {
        pos.x += vel.x * time.delta();
        pos.y += vel.y * time.delta();
    }
}
```

Fallible systems may return a SystemResult to signal failure.
```rust
fn save_entities(a: Comp<A>, b: Comp<B>, c: Comp<C>) -> SystemResult {
    for (entity, (a, b, c)) in (&a, &b, &c).iter().entities() {
        try_save_entity(entity, a, b, c)?;
    }
    Ok(())
}
```

Systems are executed using a Dispatcher. Errors can be retrieved after the systems finish executing.
```rust
let mut dispatcher = Dispatcher::builder()
    .add_system(movement.system())
    .add_system(save_entities.system())
    .build();

if let Err(run_error) = dispatcher.run_seq(&mut world) {
    for error in run_error.errors() {
        println!("{}", error);
    }
}
```

## Expressive Queries
Queries can be used to iterate entities and components.
```rust
fn example(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    // Fetch A, B and C from all entities which have A, B and C.
    for (a, b, c) in (&a, &b, &c).iter() {}

    // Use `entities` to get the entity to which the components belong.
    for (entity, (a, b, c)) in (&a, &b, &c).iter().entities() {}

    // Fetch A from all entities which have A, B and C.
    for a in (&a).include((&b, &c)).iter() {}

    // Fetch A from all entities which have A and B, but not C.
    for a in (&a).include(&b).exclude(&c).iter() {}
}
```

## Granular Change Detection
Sparsey supports change detection at a component level.
```rust
fn example(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    use sparsey::filters::{added, mutated, changed};

    // Restrict query to match only entities to which A was just added.
    for (a, b, c) in (added(&a), &b, &c).iter() {}

    // Restrict query to match only entities to which A was mutated.
    for (a, b, c) in (mutated(&a), &b, &c).iter() {}

    // Restrict query to match only entities to which A was just added or mutated.
    for (a, b, c) in (changed(&a), &b, &c).iter() {}

    // The opposite effect can be achieved by using the `Not` operator.
    // Restrict query to match only entities to which A was not just added.
    for (a, b, c) in (!added(&a), &b, &c).iter() {}
}
```

## Groups and Layouts
Layouts can be used to group component storages within a World. Grouped storages are much faster to iterate over and allow accessing their components and entities as ordered slices, with a small performance penalty when adding or removing components.
```rust
let layout = Layout::builder()
    .add_group(<(A, B)>::group())
    .add_group(<(A, B, C)>::group())
    .build();

let mut world = World::default();
world.set_layout(&layout);
```

All iterations bellow get a significant performance boost without having to change the code at all.
```rust
fn iterators(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    for (a, b) in (&a, &b).iter() {}

    for a in (&a).include(&b).iter() {}

    for (a, b, c) in (&a, &b, &c).iter() {}

    for a in (&a).include((&b, &c)).iter() {}

    for (a, b) in (&a, &b).exclude(&c).iter() {}

    for a in (&a).include(&b).exclude(&c).iter() {}
}
```

Groups allow accessing their components and entities as ordered slices.
```rust
fn slices(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    // Get all entities with A and B as a slice.
    let _: &[Entity] = (&a, &b).entities().unwrap();

    // Get A, B and C from all entities with A, B and C as slices.
    let _: (&[A], &[B], &[C]) = (&a, &b, &c).components().unwrap();

    // Get all entities with A and B, but not C, and their components, as slices.
    let _: (&[Entity], (&[A], &[B])) = (&a, &b)
        .exclude(&c)
        .entities_components()
        .unwrap();
}
```

# Thanks
Sparsey takes inspiration and borrows features from other free and open source ECS projects, namely [Bevy](https://github.com/bevyengine/bevy), [EnTT](https://github.com/skypjack/entt), [Legion](https://github.com/amethyst/legion), [Shipyard](https://github.com/leudz/shipyard) and [Specs](https://github.com/amethyst/specs). Make sure you check them out!

# License
Sparsey is dual-licensed under either
* MIT License (docs/LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (docs/LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
<br />
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above without any additional terms or conditions.
