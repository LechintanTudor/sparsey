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

Check if a `World` contains an `Entity`.
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
let a = world.borrow::<CompMut<A>>();
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
