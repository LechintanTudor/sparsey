# World, Entities and Components

## The `World`
The `World` is Sparsey's core data structure, responsible for creating and managing game data. An
empty `World` can be created using `default`. Each `World` has a unique id that can be queried with
the `id` function.

```rust, ignore
use sparsey::world::World;

let world = World::default();
println!("{:?}", world.id());
```
  
## Entities and Components
Entities represent objects within a `World` such as player characters, enemies and projectiles. An
`Entity` can have data attached to it in the form of components. For example, a player character
`Entity` can have a `Posititon` component and an `Hp` component.

## Defining Components
Any `Send + Sync + 'static` type can be a component.

```rust, ignore
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Hp(u32);
```

Before we use a Component type, we must `register` it on the `World`.

```rust, ignore
let mut world = World::default();
world.register::<Position>();
world.register::<Hp>();
```

## Creating Entities
To create a new `Entity`, use `create_entity` and provide a tuple containing the components of the
entity. To create an `Entity` with no components use an empty tuple.

```rust, ignore
let player = world.create_entity((Position { x: 0.0, y: 0.0 }, Hp(100)));
let empty = world.create_entity(());
```

## Querying Entities
Use `contains_entity` to check if a `World` contains an `Entity`.

```rust, ignore
assert!(world.contains_entity(player));
assert!(world.contains_entity(empty));
```

Use `entities` to get all the entities in the `World` as a slice.

```rust, ignore
for entity in world.entities() {
    println!("{:?}", entity);
}
```

## Removing Entities
To remove an `Entity` and all of its associated components, use `destroy_entity`.

```rust, ignore
world.destroy_entity(player);
assert!(!world.contains_entity(player));
```

To remove all entities from the `World`, use `clear_entities`.

```rust, ignore
world.clear_entities();
assert_eq!(world.entities(), &[]);
```

## Adding Components to an Existing Entity
Use `insert_components` to add a set of components to an existing `Entity`. The function returns an
error if the provided `Entity` was not found in the `World`.

```rust, ignore
let enemy = world.create_entity(());
world.insert_components(enemy, ((Position { x: 0.0, y: 0.0 }, Hp(200)))).unwrap();
```

## Removing Components from an Existing Entity
Use `delete_components` to remove a set of components from an existing `Entity`. The components to
remove are provided as a type argument.

```rust, ignore
enemy.delete_components::<(Position, Hp)>();
```

If you want to retrieve the removed components use `remove_components`. This function tries to
remove all components provided as a type argument and returns them if they were all successfully
removed.

```rust, ignore
let e1 = world.create_entity((Position { x: 0.0, y: 0.0 },));
world.remove_components::<(Position, Hp)>(e1).unwrap_none();

let e2 = world.create_entity((Position { x: 0.0, y: 0.0 }, Hp(100)));
let (position, hp) = world.remove_components::<(Position, Hp)>(e2).unwrap();
println!("{:?}, {:?}", position, hp);
```
