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
    use sparsey::World;

    struct Position(i32, i32);
    struct Velocity(i32, i32);

    fn main() {
        let mut world = World::builder()
            .register::<Position>()
            .register::<Velocity>()
            .build();

        world.create((Position(0, 0), Velocity(1, 2)));
        world.create((Position(0, 0), Velocity(2, 3)));
    
        world.for_each::<(&mut Position, &Velocity)>(|(position, velocity)| {
            position.0 += velocity.0;
            position.1 += velocity.1;
        });
    }
```

## Features

### Powerful Queries

Get, include and exclude components using Sparsey's query API.

```rust
// Iter components A and B from entities with A and B.
world
    .query_all::<(&A, &B)>()
    .for_each(|_| ());

// Iter components A from entities with A and B.
world
    .query_all::<&A>()
    .include::<&B>()
    .for_each(|_| ());

// Iter components A from entities with A and without B.
world
    .query_all::<&A>()
    .exclude::<&B>()
    .for_each(|_| ());
    
// Iter components A from entities with A and B, without C.
world
    .query_all::<&A>()
    .include::<&B>()
    .exclude::<&C>()
    .for_each(|_| ());
```

### Great Performance with Grouped Storages

Sparsey allows the user to "group" components to greatly optimize iteration
performance. When a component group is formed, the `World` ensures that all
components that belong to that group are stored in order at the beginning of
their storages, making the iteration process a traversal of densely-packed
arrays.

```rust
let mut world = World::builder()
    .add_group::<(A, B)>()
    .add_group::<(A, B, C, D>)>()
    .build();
```

Additionally, grouped components can be accessed as slices.

```rust
/// Get all entities with A and B as a slice.
world
    .query_all::<Entity>()
    .include::<(&A, &B)>
    .slice()
    .map(|_| ());

/// Get all A and B components of entities with A and B as a tuple of slices.
world
    .query_all::<(&A, &B)>()
    .slice()
    .map(|_| ());

/// Get all entities and A and B components of entities with A and B as a tuple
/// of slices.
world
    .query_all::<(Entity, &A, &B)>()
    .slice()
    .map(|_| ());

/// Get all A and B components of entities with A and B, and without C and D as
/// a tuple.
world
    .query_all::<(&A, &B)>()
    .exclude::<(&C, &D)>()
    .slice()
    .map(|_| ());
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
