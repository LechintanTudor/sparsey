# Sparsey Cheat Sheet
Useful code snippets for working with Sparsey \~( ˘▾˘\~)

## World
Create a `World` without storage grouping.
```rust
let world = World::default();
```

Register component types.
```rust
world.register::<A>();
world.register::<B>();
```

Check if a component was registered.
```rust
if world.is_registered(&TypeId::of::<A>()) {
    /// ...
}
```

Create a `Layout` for storage grouping.
```rust
let layout = Layout::builder()
    .add_group(<(A, B)>::group())
    .add_group(<(A, B, C)::group()>)
    .build();
```

Create a `World` with storage grouping.
```rust
let world = World::with_layout(&layout);
```

Set a `Layout` for an existing `World`.
```rust
world.set_layout(&layout);
```

## Entities and Components
Create an `Entity` with no components.
```rust
let entity = world.create_entity(());
```

Check if an `Entity` exists.
```rust
if world.contains_entity(entity) {
    // ...
}
```

Create an `Entity` with `A` and `B`,
```rust
let entity = world.create_entity((A, B));
```

Append a `C` component to an existing `Entity`.
```rust
world.append_components(entity, (C,)).unwrap();
```

Remove components `A` and `B` from an existing `Entity`.
```rust
if let Some((a, b)) = world.remove_components<(A, B)>(entity) {
    // ...
}
```

## Component Views
Get an immutable component view.
```rust
let a = world.borrow::<Comp<A>>();
```

Get a mutable component view.
```rust
let mut a = world.borrow::<CompMut<A>>();
```

Borrow multiple views simultaneously.
```rust
let (a, b) = world.borrow::<(Comp<A>, Comp<B>)>();
```

Check if a view contains an `Entity`.
```rust
if a.contains(entity) {
    // ...
}
```

Get the component of an `Entity`.
```rust
if let Some(a) = a.get(entity) {
    /// ...
}
```

Get all entities with `A` components.
```rust
let entities: &[Entity] = a.entities();
```

Get all `A` components.
```rust
let components: &[A] = a.components();
```

Iterate all entities and components.
```rust
for (entity, component) in a.iter().entities() {
    /// ...
}
```

## Queries
Get mutable views over `A`, `B` and `C`.
```rust
let (mut a, mut b, mut c) = world.borrow::<(CompMut<A>, CompMut<B>, CompMut<C>)>();
```

Iterate the components of entities with `A`, `B` and `C`.
```rust
for (a, b, c) in (&a, &b, &c).iter() {
    // ...
}
```

Also get the entities while iterating the components.
```rust
for (entity, (a, b, c)) in (&a, &b, &c).iter().entities() {
    // ...
}
```

Iterate `A` components mutably and `B` and `C` components immutably.
```rust
for (mut a, b, c) in (&mut a, &b, &c).iter() {
    // ...
}
```

Iterate `A` components of entities with `B` and `C`.
```rust
for (a,) in (&a,).include((&b, &c)).iter() {
    // ...
}
```

Iterate `A` components of entities without `B` and `C`.
```rust
for (a,) in (&a,).exclude((&b, &c)).iter() {
    // ...
}
```

Iterate `A` components of entities with `B` and without `C`.
```rust
for (a,) in (&a,).include(&b).exclude(&c).iter() {
    // ...
}
```

Check if a query matches an `Entity`.
```rust
if (&a, &b, &c).contains(entity) {
    /// ...
}
```

Get the components for an `Entity`.
```rust
if let Some((a, b, c)) = (&a, &b, &c).get(entity) {
    /// ...
}
```

## Query Filters
Import filter functions
```rust
use sparsey::filters::{added, mutated, changed};
```

Iterate `A` components which were added/mutated/changed.
```rust
for (a,) in (added(&a),).iter() {
    // ...
}

for (a,) in (mutated(&a),).iter() {
    // ...
}

for (a,) in (changed(&a),).iter() {
    // ...
}
```

Iterate `A` components which were not added/mutated/changed.
```rust
for (a,) in (!added(&a),).iter() {
    // ...
}

for (a,) in (!mutated(&a),).iter() {
    // ...
}

for (a,) in (!changed(&a),).iter() {
    // ...
}
```

Filter query to only match entities to which `B` and `C` was just added.
```rust
for (a,) in (&a,).filter(added(&b) & added(&c)).iter() {
    /// ...
}
```

Filter query to only match entities to which `B` or `C` was just added.
```rust
for (a,) in (&a,).filter(added(&b) | added(&c)).iter() {
    /// ...
}
```

## Query Slicing
Create a `World` with `(A, B)` and `(A, B, C)` groups and borrow the component storages.
```rust
let layout = Layout::builder()
    .add_group(<(A, B)>::group())
    .add_group(<(A, B, C)>::group())
    .build();

let world = World::with_layout(&layout);
let (a, b) = world.borrow::<(Comp<A>, Comp<B>, Comp<C>)>();
```

Get all entities with `A`, and `B`.
```rust
let entities: &[Entity] = (&a, &b).entities().unwrap();
```

Get all components of entities with `A` and `B`.
```rust
let (a, b, c): (&[A], &[B]) = (&a, &b).components().unwrap();
```

Get the entities with `A` and `B` and their components.
```rust
let (entities, (a, b)): (&[Entity], (&[A], &[B])) = (&a, &b).entities_components().unwrap();
```

Get all components of entities with `A` and `B`, without `C`.
```rust
let (a, b) = (&a, &b).exclude(&c).components().unwrap();
```

## Resources
Insert a resource.
```rust
let previous_res: Option<A> = world.insert_resource(A);
```

Check if a resource exists.
```rust
if world.contains_resource(&TypeId::of::<A>()) {
    /// ...
}
```

Remove a resource.
```rust
let res: Option<A> = world.remove_resource::<A>(); 
```

Borrow a resource immutably.
```rust
let res = world.borrow::<Res<A>>(); 
```

Borrow a resource mutably.
```rust
let mut res = world.borrow::<ResMut<A>>();
```

## Resource Filters
Import resource filters.
```rust
use sparsey::filters::{res_added, res_mutated, res_changed}; 
```

Check if a resource was added/mutated/changed.
```rust
if res_added(&a) {
    // ...
} 

if res_muated(&a) {
    // ...
} 

if res_changed(&a) {
    // ...
} 
```
