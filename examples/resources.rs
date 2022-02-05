use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Lava {
    height: i32,
}

#[derive(Debug)]
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

fn fall_in_lava(pos: Comp<Position>, lava: Res<Lava>, mut fallen_in_lava: ResMut<FallenInLava>) {
    (&pos).for_each_with_entity(|(e, pos)| {
        if pos.y < lava.height {
            println!("{:?} with y={} fell in lava", e, pos.y);
            fallen_in_lava.entities.push(e);
        }
    });

    println!();
}

fn destroy_fallen_in_lava(world: &mut World, resources: &mut Resources) {
    let mut fallen_in_lava = resources.borrow_mut::<FallenInLava>().unwrap();
    world.bulk_destroy(&fallen_in_lava.entities);
    fallen_in_lava.entities.clear();
}

fn main() {
    let mut schedule = Schedule::builder()
        .add_system(raise_lava)
        .add_system(fall_in_lava)
        .add_local_fn(destroy_fallen_in_lava)
        .build();

    let mut world = World::default();
    schedule.set_up(&mut world);

    world.create((Position { y: 0 },));
    world.create((Position { y: 1 },));
    world.create((Position { y: 2 },));
    world.create((Position { y: 3 },));
    world.create((Position { y: 4 },));
    world.create((Position { y: 5 },));

    let mut resources = Resources::default();
    resources.insert(Lava { height: 0 });
    resources.insert(FallenInLava { entities: Vec::new() });

    for _ in 0..3 {
        schedule.run_seq(&mut world, &mut resources);
    }
}
