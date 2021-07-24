# Sparsey
Sparsey is a sparse set based Entity Component System with lots of features and nice syntax! \~( ˘▾˘\~)
<br/>

# Example 
```rust
/// Most commonly used items are accessible from the prelude.
/// Otherwise, all items are accessible from the crate root.
use sparsey::prelude::*;

/// Components are Send + Sync + 'static types.
struct Position(f32);
struct Velocity(f32);
struct Immovable;

// Sets the Velocity of Immovable entities to zero.
fn update_velocity(mut vel: CompMut<Velocity>, imv: Comp<Immovable>) {
    for (mut vel,) in (&mut vel,).include(&imv).iter() {
        vel.0 = 0.0;
    }
}

// Adds the Velocity of an entity to its Position. 
fn update_position(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
    for (mut pos, vel) in (&mut pos, &vel).iter() {
        pos.0 += vel.0;
    }
} 

fn main() {
    // Create the World and register the components we want to use.
    let mut world = World::default();
    world.register::<Position>();
    world.register::<Velocity>();

    /// Create some entities.
    world.create((Position(0.0), Velocity(1.0)));
    world.create((Position(0.0), Velocity(2.0)));
    world.create((Position(0.0), Velocity(3.0), Immovable));

    /// Resources can be used to store data which doesn't belong to
    /// any single entity. In our case, there are none.
    let mut resources = Resources::default();

    /// Create a Dispatcher for running our systems.
    let mut dispatcher = Dispatcher::builder()
        .add_system(update_velocity.system())
        .add_system(update_position.system())
        .build();

    /// Run the systems 3 times.
    for _ in 0..3 {
        dispatcher.run_seq(&mut world, &mut resources).unwrap();
        world.advance_ticks().unwrap();
    }
}
```
<br/>

# Features
## Systems
Systems are functions that have Component views and Resource views as parameters.
```rust
fn movement(mut pos: CompMut<Position>, vel: Comp<Velocity>, delta: Res<Delta>) {
    for (mut pos, vel) in (&mut pos, &vel).iter() {
        pos.x += vel.x * delta.0;
        pos.y += vel.y * delta.1;
    }
}
```
<br/>

Fallible systems may return a SystemResult to signal success or failure.
```rust
fn save_components(a: Comp<A>, b: Comp<B>, c: Comp<C>) -> SystemResult {
    for (a, b, c) in (&a, &b, &c).iter() {
        try_save_components(a, b, c)?;
    }
    Ok(())
}
```
<br/>

Systems are executed using a Dispatcher. 
Errors can be retrieved after the systems finish executing.
```rust
let mut dispatcher = Dispatcher::builder()
    .add_system(movement.system())
    .add_system(failable.system())
    .build();

if let Err(run_error) = dispatcher.run_seq(&mut world, &mut resources) {
    for error in run_error.errors() {
        println!("{}", error);
    }
}
```
<br/>

## Expressive Queries
Queries can be used to iterate entities and components.
```rust
fn example(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    // Fetch A, B and C from all entities which have A, B and C.
    for (a, b, c) in (&a, &b, &c).iter() {}

    // To get the entity to which the components belong use entities().
    for (entity, (a, b, c)) in (&a, &b, &c).iter().entities() {}

    // Fetch A from all entities which have A, B and C.
    for (a,) in (&a,).include((&b, &c)).iter() {}

    // Fetch A from all entities which have A and B, but not C.
    for (a,) in (&a,).include(&b).exclude(&c).iter() {}
}
```
<br/>

## Granular Change Detection
Sparsey supports change detection at a component level.
```rust
fn example(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    use sparsey::filters::{added, mutated, updated};

    // Restrict query to match only entities to which A was just added.
    for (a, b, c) in (added(&a), &b, &c).iter() {}

    // Restrict query to match only entities to which A was mutated.
    for (a, b, c) in (mutated(&a), &b, &c).iter() {}

    // Restrict query to match only entities to which A was just added or mutated.
    for (a, b, c) in (updated(&a), &b, &c).iter() {}

    // The opposite effect can be achieved by using the Not operator.
    // Restrict query to match only entities to which A was not just added.
    for (a, b, c) in (!added(&a), &b, &c).iter() {}
}
```
<br/>

## Groups and Layouts.
Layouts can be used to group component storages withing a World.
Grouped storages are much faster to iterate over, the downside being
a small performance penalty when inserting or removing components.
```rust
let layout = Layout::builder()
    .add_group(<(A, B)>::group())
    .add_group(<(A, B, C)>::group())
    .build();

let mut world = World::with_layout(&layout);
```
<br/>

All iterations bellow get a significant performance boost without having to change
the code at all.
```rust
fn iterators(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    for (a, b) in (&a, &b).iter() {}

    for (a,) in (&a,).include(&b).iter() {}

    for (a, b, c) in (&a, &b, &c).iter() {}

    for (a,) in (&a,).include((&b, &c)).iter() {}

    for (a, b) in (&a, &b).exclude(&c).iter() {}

    for (a,) in (&a,).include(&b).exclude(&c).iter() {}
}
```
<br/>

Groups allow accessing their components as ordered slices.
```rust
fn slices(a: Comp<A>, b: Comp<B>, c: Comp<C>) {
    // Get all entities with A and B as a slice.
    let _: &[Entity] = (&a, &b).entities();

    // Get A, B and C from all entities with A, B and C as slices.
    let _: (&[A], &[B], &[C]) = (&a, &b, &c).components();

    // Get all entities with A and B, but not C, and their components, as slices.
    let _: (&[Entity], (&[A], &[B])) = (&a, &b).exclude(&c).entities_components();
}
```
