# Queries

Queries allow fetching and iterating over entities that have a given set of components. Queries can
can be created by borrowing Component Views. Immutable Component Views support immutable component
fetching and iteration, while mutable Component Views support both immutable and mutable operations.

```rust, ignore
use sparsey::perlude::*;

#[derive(Debug)]
struct Position(i32, i32);

#[derive(Debug)]
struct Velocity(i32, i32);

fn main() {
    let mut world = World::default();
    world.register::<Position>();
    world.register::<Velocity>();

    let e0 = world.create((Position(0, 0), Velocity(1, 0)));
    let e1 = world.create((Position(0, 0), Velocity(2, 0)));
    let e2 = world.create((Position(0, 0),));

    let mut positions = world.borrow_mut::<Position>();
    let velocities = world.borrow::<Velocity>();

    // Prints the Position and Velocity of 'e0'.
    if let Some((position, velocity)) = (&positions, &velocities).get(e0) {
        println!("e0: {:?}, {:?}", position, velocity);
    }

    // Checks if 'e1' contains both Position and Velocity.
    assert!((&positions, &velocities).contains(e1));

    // Iterates over all entities with both Position and Velocity.
    (&mut positions, &velocities).for_each(|(position, velocity)| {
        position.0 += velocity.0;
        position.1 += velocity.1;
    });

    // Use `for_each_with_entity` to get the entity to which the components belong.
    (&positions, &velocities).for_each_with_entity(|(entity, (position, vleocity))| {
        println!("{:?}: {:?}, {:?}", entity, position, velocity);
    })
}
```
