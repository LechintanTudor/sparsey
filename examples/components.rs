use sparsey::component::GroupLayout;
use sparsey::World;

pub struct Handler<'a>(Vec<(&'a mut Position,)>);

impl Drop for Handler<'_> {
    fn drop(&mut self) {
        println!("{:#?}", self.0);
    }
}

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
        .build_layout();

    let mut world = World::new(&layout);
    world.create((Position { x: -1, y: -1 },));
    world.create((Position { x: 0, y: 0 }, Speed { x: 0, y: 0 }));
    world.create((Position { x: 1, y: 1 }, Speed { x: 1, y: 1 }));

    world.for_each::<&Position>(|position| {
        println!("{position:#?}");
    });
}
