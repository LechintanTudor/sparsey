use sparsey::entity::{Entity, GroupLayout, World};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Speed {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let layout = GroupLayout::builder()
        .add_group::<(Position, Speed)>()
        .build();

    let mut world = World::new(&layout);
    world.create((Position { x: 0, y: 0 }, Speed { x: 1, y: 2 }));
    world.create((Position { x: 0, y: 0 }, Speed { x: 2, y: 1 }));

    world
        .query_all::<(Entity, &Position, &Speed)>()
        .iter()
        .for_each(|item| println!("{item:#?}\n"));

    println!("=====");

    world
        .query_all::<(Entity, &Position, &Speed)>()
        .for_each(|item| println!("{item:#?}\n"));

    println!("=====");

    world.for_each::<(Entity, &Position, &Speed)>(|(entity, position, speed)| {
        println!("{:#?}\n", (entity, position, speed))
    });
}
