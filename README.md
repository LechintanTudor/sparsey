Sparsey is a sparse set based Entity Component System that aims to provide the benefits of its core data structure with an intuitive and easy to use interface.

## Overview
### World and Components
```rust
// Components are simple structs.
struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

// World is a container for entities and components.
let mut world = World::default();
world.register::<Position>();
world.register::<Velocity>();
```

```rust
// Create a single Entity with a Component tuple.
let _: Entity = world.create(
    (Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }),
);

// Create entities in bulk with a Component tuple iterator.
let _: &[Entity] = world.extend(std::iter::repeat(10).map(|i| {
    (Position { x: i as f32, y: i as f32 }, Velocity { x: 1.0, y: 1.0 })
}))
```

### Systems and Failable Systems
```rust
// Systems are functions which take Component views as parameters.
fn movement(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
    // Queries are created from a tuple of Component views.
    // You can use `iter` to iterate all matching Component sets.
    for (mut pos, vel) in (&mut pos, &vel).iter() {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}
```

```rust
// Failable systems can return a SystemResult to signal failure.
fn failable() -> SystemResult {
    failable_function()?;
    Ok(())
}
```

```rust
// Dispatchers can be used to run the systems. To create a System 
// from a function use the `system` method directly on the function.
let mut dispatcher = Dispatcher::builder()
    .add_system(movement.system())
    .add_system(failable.system())
    .build();

// Run the systems and unwrap any errors.
dispatcher.run_seq(&mut world, &mut resources).unwrap();
```

### Granular Change Detection
```rust
fn movement(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
    use sparsey::filters::{added, changed, updated};

    // Iterate Component sets where the Position was added this frame.
    for (_, _) in (added(&mut pos), &vel).iter() {}

    // Iterate Component sets where the Position was changed this frame.
    for (_, _) in (changed(&mut pos), &vel).iter() {}

    // Iterate Component sets where the Position was updated this frame.
    // In our case, updated means the Component was added or changed.
    for (_, _) in (updated(&mut pos), &vel).iter() {}

    // To get the opposite effect, use the not operator.
    // Iterate Component sets where the Position was not added this frame.
    for (_, _) in (!added(&mut pos), &vel).iter() {}
}
```

### Layouts and Groups
```rust
// Grouped component storages ensure the components
// are stored in ordered arrays to provide the fastest
// iteration possible and keep cache misses to minimum.
let layout = Layout::builder()
    .add_group(<(A, B)>::group())
    .add_group(<(A, B, C)>::group())
    .build();

let world = World::with_layout(&layout);
```

```rust
fn group_test(a: Comp<A>, b: Comp<B>, c: Comp<C>) -> SystemResult {
    // Get all entities with A and B.
    let _: &[Entity] = (&a, &b).entities()?;

    // Get all A and B Component sets as ordered slices.
    let _: (&[A], &[B]) = (&a, &b).slice()?;

    // Get all A and B component sets and their entities.
    let _: (&[Entity], (&[A], &[B])) = (&a, &b).slice_entities()?;

    Ok(())
}
```
