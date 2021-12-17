# Component Views

## Manipulating Game Data
Consider the following code:

```rust, ignore
use sparsey::world::World;

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Velocity(f32, f32);

fn main() {
    let mut world = World::default();
    world.register::<Position>();
    world.register::<Velocity>();

    let entity1 = world.create_entity((Position(0.0, 0.0), Velocity(1.0, 1.0)));
    let entity2 = world.create_entity((Position(1.0, 1.0), Velocity(2.0, 2.0)));
    let entity3 = world.create_entity((Position(2.0, 2.0),));
}
```

We would like to update the `Position` of all entities that have a `Velocity`, but the `World` 
doesn't have functions to mutate the components directly. Luckily, component views come to the 
rescue!

## Borrowing Component Views
For each component type, Sparsey stores an array with every instance of that component added to the
`World`. To get a view over a component storage, we use the `borrow` function on `World`, with a
`Comp<T>` parameter for immutable views or a `CompMut<T>` for mutable views. In our example, we
want to get a mutable view over `Position` and an immutable view over `Velocity`.

```rust, ignore
use sparsey::world::{Comp, CompMut};

let mut positions = world.borrow::<CompMut<Position>>();
let velocities = world.borrow::<Comp<Velocity>>();
```

## Querying Component Views
Because Sparsey stores components in packed arrays, we can get slices of all entities and components
from a component view using the `entities` and `components` functions.

```rust, ignore
println!("All entities with positions:");
for entity in positions.entities() {
    println!("{:?}", entity);
}

println!("All positions:");
for position in positions.components() {
    println!("{:?}", position);
}
```

To check if a component view contains an `Entity`, use the `contains` function.

```rust, ignore
println!("{:?} has Position is {}", entity1, positions.contains(entity1));
```

## Iterating Over Multiple Component Views
Coming back to our original problem, we need to get the `Position` and `Velocity` of all entities
that have both, and add the later to the former. To do that we call the `iter` function on a tuple
that contains a mutable reference to the `Position` view to get mutable components and an immutable
reference to the `Velocity` to get immutable components. The `iter` function is provided by the
`Query` trait.

```rust, ignore
use sparsey::query::Query;

for (mut pos, vel) in (&mut positions, &velocities).iter() {
    pos.0 += vel.0;
    pos.1 += vel.1;
}