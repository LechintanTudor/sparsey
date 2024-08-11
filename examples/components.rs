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

    let e0 = world.create((Position { x: 0, y: 0 }, Speed { x: 1, y: 2 }));
    let e1 = world.create((Position { x: 0, y: 0 }, Speed { x: 2, y: 1 }));

    for item in &mut world.query_all::<(Entity, &Position, &Speed)>() {
        println!("{item:#?}\n");
    }
}
