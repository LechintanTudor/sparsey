# Sparsey

[![Crates.io](https://img.shields.io/crates/v/sparsey)](https://crates.io/crates/sparsey)
[![Documentation](https://docs.rs/sparsey/badge.svg)](https://docs.rs/sparsey)

Sparsey is a sparse set-based Entity Component System with lots of features and beautiful syntax \~(
˘▾˘\~)

# Example

```rust
use sparsey::prelude::*;

struct Position(f32);
struct Velocity(f32);
struct Frozen;

fn update_velocities(mut velocities: CompMut<Velocity>, frozen: Comp<Frozen>) {
    (&mut velocities).include(&frozen).for_each(|velocity| {
        velocity.0 = 0.0;
    });
}

fn update_positions(mut positions: CompMut<Position>, velocities: Comp<Velocity>) {
    (&mut positions, &velocities).for_each(|(position, velocity)| {
        position.0 += velocity.0;
    })
} 

fn main() {
    let mut schedule = Schedule::builder()
        .add_system(update_velocities)
        .add_system(update_positions)
        .build();

    let mut world = World::default();
    schedule.set_up(&mut world);

    world.create((Position(0.0), Velocity(1.0)));
    world.create((Position(0.0), Velocity(2.0)));
    world.create((Position(0.0), Velocity(3.0), Frozen));

    let mut resources = Resources::default();

    for _ in 0..5 {
        schedule.run(&mut world, &mut resources);
    }
}
```

# Features

## Easy to Use Systems

Systems are plain functions that borrow data from `World` and `Resources`.

```rust
fn update_positions(mut positions: CompMut<Position>, velocities: Comp<Velocity>) {
    (&mut positions, &velocities).for_each(|(position, velocity)| {
        position.0 += velocity.0;
    })
}

fn update_hps(mut hps: CompMut<Hp>, heals: Comp<Heal>, heal_multipler: Res<HealMultiplier>) {
    (&mut hps, &heals).for_each(|(hp, heal)| {
        hp.0 += heal.0 * heal_multiplier.0;
    })
}
```

Systems will be scheduled to run in parallel if their paramters don't conflict.

```rust
let schedule = Schedule::builder()
    .add_system(update_positions)
    .add_system(update_hps)
    .build()
```

## Expressive Queries

Get, include and exclude components using Sparsey's query API.

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

    // These return Some(_) if the storages are grouped and None otherwise.
    let _: Option<&[Entity]> = (&a, &b).as_entity_slice();
    let _: Option<(&[A], &[B])> = (&a, &b).as_component_slices();
    let _: Option<(&[Entity], (&[A], &[B]))> = (&a, &b).as_entity_and_component_slices();
}
```

# Thanks

Sparsey takes inspiration and borrows features from other free and open source ECS projects, namely
[Bevy](https://github.com/bevyengine/bevy), [EnTT](https://github.com/skypjack/entt),
[Legion](https://github.com/amethyst/legion), [Shipyard](https://github.com/leudz/shipyard) and
[Specs](https://github.com/amethyst/specs). Make sure you check them out!

# License

Sparsey is dual-licensed under either

- MIT License (docs/LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (docs/LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. <br /> Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual
licensed as above without any additional terms or conditions.
