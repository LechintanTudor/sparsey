# Component Views

Components from the `World` can be queried and changed using Component Views. Component Views can be
immutable (`Comp<T>`) or mutable (`CompMut<T>`) and allow accessing all components of a given type
and the entities to which the components belong. The `World` uses interior mutability to ensure that
the user doesn't violate Rust's aliasing rules when borrowing views.

```rust, ignore
use sparsey::prelude::*;

#[derive(Debug)]
struct Position(i32, i32);

#[derive(Debug)]
struct Velocity(i32, i32);

fn main() {
    let mut world = World::default();
    world.register::<Position>();
    world.register::<Velocity>();

    let e0 = world.create((Position(0, 0), Velocity(1, 0)));
    let e1 = world.create((Position(0, 0),));

    // Views can be borrowed mutably or immutably.
    let positions: CompMut<Position> = world.borrow_mut::<Position>();
    let velocities: Comp<Velocity> = world.borrow::<Velocity>();

    // Prints all components and the entities to which they belong. 
    println!("All Position components: {:?}", positions.components());
    println!("All entities with Position: {:?}", positions.entities());

    println!("All Velocity components: {:?}", velocities.components());
    println!("All entities with Velocities: {:?}", velocities.entities());
}
```
