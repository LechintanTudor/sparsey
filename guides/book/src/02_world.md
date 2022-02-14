# The World

The `World` is a container for entities and components. Entities represent objects within a game
world and can have data associated with them in the form of components. Components must be
registered before using them in a `World`. The user can `create` and `destory` entities and
`insert`, `remove` and `delete` components from existing entities.

```rust, ignore
use sparsey::prelude::*;

struct Position(i32, i32);
struct Velocity(i32, i32);

fn main() {
    let mut world = World::default();
    // Componenents must be registered before being used.
    world.register::<Position>();
    world.register::<Velocity>();

    // An entity can be created with an initial set of components.
    let entity = world.create((Position(0, 0), Velocity(0, 0));
    assert!(world.contains(entity));

    // Removes components from an existing entity and returns them.
    let (velocity,): (Option<Velocity>,) = world.remove::<(Velocity,)>(entity);
    assert!(velocity.is_some());

    let (velocity,): (Option<Velocity>,) = world.remove::<(Velocity,)>(entity);
    assert!(velocity.is_none());

    // Adds components to an existing entity.
    world.insert(entity, (Velocity(1, 1),));

    // Deletes components from an existing entity. This is faster than removing them.
    world.delete::<(Velocity,)>(entity);

    // Destroying an entity removes all of its remaining components from the World.
    world.destroy(entity);
    assert!(!world.contains(entity));
}
```
