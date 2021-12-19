# Sparsey
[![Crates.io](https://img.shields.io/crates/v/sparsey)](https://crates.io/crates/sparsey)
[![Documentation](https://docs.rs/sparsey/badge.svg)](https://docs.rs/sparsey)

Sparsey is a sparse set-based Entity Component System with lots of features and nice syntax
\~( ˘▾˘\~)
<br />
Check out the [Sparsey Cheat Sheet](/guides/cheat_sheet.md) and [examples](/examples/) to get
started!

# Example 
```rust
use sparsey::prelude::*;

// Components are Send + Sync + 'static types.
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
    // Create a World to store out game data.
    let mut world = World::default();

    // Create a Dispatcher to run our systems.
    let mut dispatcher = Dispatcher::builder()
        .add_system(update_velocity.system())
        .add_system(update_position.system())
        .build();

    // Register all component types we want to use.
    dispatcher.register_storages(&mut world);

    // Create some entities.
    world.create_entity((Position(0.0), Velocity(1.0)));
    world.create_entity((Position(0.0), Velocity(2.0)));
    world.create_entity((Position(0.0), Velocity(3.0), Immovable));

    // Run the systems 3 times.
    for _ in 0..3 {
        dispatcher.run_seq(&mut world).unwrap();
        world.increment_tick();
    }
}
```

# Features
## Easy to Write Systems
Systems are plain functions that may return a `SystemResult` to signal errors.

```rust
fn print_alive_entities(hp: Comp<Hp>) {
    for (entity, hp) in (&hp).iter().entities() {
        if hp.0 > 0 {
            println!("{:?} - {:?}", entity, hp);
        }
    }
}

fn save_entities(save_file: ResMut<SaveFile>, position: Comp<Position>) -> SystemResult {
    for (entity, position) in (&position).iter().entities() {
        save_file.save_entity_with_position(entity, position)?
    }

    Ok(())
}
```

## Expressive Queries
Get, include, exclude and filter components using Sparsey's query API.

```rust
fn queries(a: Comp<A>, b: Comp<B>, c: Comp<C>, d: Comp<D>, e: Comp<E>) {
    // Iter components A and B from entities with A and B.
    for (a, b) in (&a, &b).iter() {}

    // Iter components A from entities with A and B.
    for a in (&a).include(&b).iter() {}

    // Iter components A from entities with A and without B.
    for a in (&a).exclude(&b).iter() {}

    // Iter components A from entities with A and B, without C.
    for a in (&a).include(&b).exclude(&c).iter() {}

    // Iter components A from entities with A and B, without C and with D xor E.
    for a in (&a).include(&b).exclude(&c).filter(contains(&d) ^ contains(&e)).iter() {}
}
```

## Granular Change Detection
Filter queries based on whether or not a component or component set was changed.

```rust
fn change_detection(a: Comp<A>, b: Comp<B>, c: Comp<C>, d: Comp<D>, e: Comp<E>) {
    // Iter changed A components.
    for a in changed(&a).iter() {}

    // Iter A, B component sets where A or B was changed. 
    for (a, b) in changed((&a, &b)).iter() {}

    // Iter A, B, C component sets where A or B was changed and C was changed. 
    for ((a, b), c) in (changed((&a, &b)), changed(&c)).iter() {}
}
```

## Great Performance with Grouped Storages
Sparsey allows the user to "group" component storages to greatly optimize iteration performance.
<br />

Groups are created by setting a `Layout` on the `World`.
```rust
let layout = Layout::builder()
    .add_group(<(A, B)>::group())
    .add_group(<(A, B, C, D>)>::group())
    .build();

let world = World::with_layout(&layout);
```

After the layout is set, iterators over the grouped storages become "dense", greatly improving their
performance. Additionally, grouped storages allow access to their components and entities as slices.

```rust
fn dense_iterators(a: Comp<A>, b: Comp<B>, c: Comp<C>, d: Comp<D>) {
    assert!((&a, &b).iter().is_dense());
    assert!((&a, &b, &c, &d).iter().is_dense());
    assert!((&a, &b).exclude((&c, &d)).iter().is_dense());

    let _: &[Entity] = (&a, &b).entities();
    let _: (&[A], &[B]) = (&a, &b).components();
    let _: (&[Entity], (&[A], &[B])) = (&a, &b).entities_components();
}
```

# Thanks
Sparsey takes inspiration and borrows features from other free and open source ECS projects, namely 
[Bevy](https://github.com/bevyengine/bevy), [EnTT](https://github.com/skypjack/entt),
[Legion](https://github.com/amethyst/legion), [Shipyard](https://github.com/leudz/shipyard) and 
[Specs](https://github.com/amethyst/specs). Make sure you check them out!

# License
Sparsey is dual-licensed under either
* MIT License (docs/LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (docs/LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
<br />
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above without any 
additional terms or conditions.
