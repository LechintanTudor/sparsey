# Sparsey

[![Crates.io](https://img.shields.io/crates/v/sparsey)](https://crates.io/crates/sparsey)
[![Documentation](https://docs.rs/sparsey/badge.svg)](https://docs.rs/sparsey)

Sparsey is a sparse set-based
[Entity Component System (ECS)](https://github.com/SanderMertens/ecs-faq#what-is-ecs).

## Design Goals

- Be flexible: Any `Send + Sync + 'static` type can be used as a component.
- Be concise: The most commonly used functionalities should require the least
  amount of typing.
- Make use of sparse sets: Provide features exclusive to sparse set-based ECS.

## Example

```rust
use sparsey::prelude::*;

struct Position(f32);
struct Velocity(f32);

fn main() {
    let mut entities = EntityStorage::default();
    entities.register::<Position>();
    entities.register::<Velocity>();

    entities.create((Position(0), Velocity(1)));
    entities.create((Position(0), Velocity(2)));
    entities.create((Position(0), Velocity(3)));

    entities.run(|mut positions: CompMut<Position>, velocities: Comp<Velocity>| {
        (&mut positions, &velocities).for_each(|(position, velocity)| {
            position.0 += velocity.0;
        }); 
    });
}
```

## Features

### Easy to Use Systems

Systems are plain functions.

```rust
struct HealMultiplier(f32);

fn update_positions(mut positions: CompMut<Position>, velocities: Comp<Velocity>) {
    (&mut positions, &velocities).for_each(|(position, velocity)| {
        position.0 += velocity.0;
    });
}

fn update_hps(mut hps: CompMut<Hp>, heals: Comp<Heal>, heal_multipler: Res<HealMultiplier>) {
    (&mut hps, &heals).for_each(|(hp, heal)| {
        hp.0 += heal.0 * heal_multiplier.0;
    });
}

let mut world = World::default();
world.entities.register::<Position>();
world.entities.register::<Velocity>();
world.resources.insert(HealMultiplier(1.2));

world.run(update_positions);
world.run(update_hps);
```

### Expressive Queries

Get, include and exclude components using Sparsey's query API.

```rust
fn queries(a: Comp<A>, b: Comp<B>, c: Comp<C>, d: Comp<D>, e: Comp<E>) {
    // Iter components A and B from entities with A and B.
    (&a, &b).for_each(|(a, b)| {
        // ... 
    });

    // Iter components A from entities with A and B.
    (&a).include(&b).for_each(|a| {
        // ...
    });

    // Iter components A from entities with A and without B.
    (&a).exclude(&b).for_each(|a| {
        // ...
    });

    // Iter components A from entities with A and B, without C.
    (&b).include(&b).exclude(&c).for_each(|a| {
        // ...
    });
}
```

### Great Performance with Grouped Storages

Sparsey allows the user to "group" component storages to greatly optimize
iteration performance. Groups are created by setting a `GroupLayout`.

```rust
let layout = GroupLayout::builder()
    .add_group::<(A, B)>()
    .add_group::<(A, B, C, D>)>()
    .build();

let entities = EntityStorage::new(&layout);
```

After the layout is set, iterators over the grouped storages become "dense",
greatly improving their performance. Additionally, grouped storages allow access
to their components and entities as slices.

```rust
fn group_slices(a: Comp<A>, b: Comp<B>) {
    if let Some(entities) = (&a, &b).group_entities() {
        // ...
    }

    if let Some((a_slice, b_slice)) = (&a, &b).group_components() {
        // ...
    }

    if let Some((entities, (a_slice, b_slice))) = (&a, &b).group_data() {
        // ...
    }
}
```

## Thanks

Sparsey takes inspiration and borrows features from other free and open source
ECS projects, namely [Bevy](https://github.com/bevyengine/bevy),
[EnTT](https://github.com/skypjack/entt),
[Legion](https://github.com/amethyst/legion),
[Shipyard](https://github.com/leudz/shipyard) and
[Specs](https://github.com/amethyst/specs). Make sure you check them out!

## License

Sparsey is dual-licensed under either

- MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/license/mit/](https://opensource.org/license/mit/))

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))

at your option.

<br />

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above without any additional terms or conditions.
