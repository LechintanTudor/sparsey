use sparsey::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
struct Lava {
    height: i32,
}

#[derive(Default, Debug)]
struct FallenInLava {
    entities: Vec<Entity>,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    y: i32,
}

fn raise_lava(mut lava: ResMut<Lava>) {
    lava.height += 2;
    println!("[Lava raised to y={}]", lava.height);
}

fn update_fallen_in_lava(
    positions: Comp<Position>,
    lava: Res<Lava>,
    mut fallen_in_lava: ResMut<FallenInLava>,
) {
    (&positions).for_each_with_entity(|(entity, positions)| {
        if positions.y < lava.height {
            println!("{:?} with y={} fell in lava", entity, positions.y);
            fallen_in_lava.entities.push(entity);
        }
    });

    println!();
}

fn destroy_fallen_in_lava(world: &mut World) {
    world
        .resources
        .get_mut::<FallenInLava>()
        .entities
        .drain(..)
        .for_each(|entity| {
            world.entities.destroy(entity);
        });
}

fn main() {
    let mut world = World::default();
    world.entities.register::<Position>();
    world.resources.insert(Lava::default());
    world.resources.insert(FallenInLava::default());

    world.entities.create((Position { y: 0 },));
    world.entities.create((Position { y: 1 },));
    world.entities.create((Position { y: 2 },));
    world.entities.create((Position { y: 3 },));
    world.entities.create((Position { y: 4 },));
    world.entities.create((Position { y: 5 },));

    for _ in 0..3 {
        world.run(raise_lava);
        world.run(update_fallen_in_lava);
        destroy_fallen_in_lava(&mut world);
    }
}
