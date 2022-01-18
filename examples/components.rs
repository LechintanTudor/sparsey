use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Velocity(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Immovable;

fn update_velocity(mut vel: CompMut<Velocity>, imv: Comp<Immovable>) {
    println!("[Update velocities]");

    for (e, vel) in (&mut vel).include(&imv).iter().entities() {
        println!("{:?} is immovable; set its velocity to (0, 0)", e);
        *vel = Velocity(0, 0);
    }

    println!();
}

fn update_position(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
    println!("[Update positions]");

    for (e, (pos, vel)) in (&mut pos, &vel).iter().entities() {
        pos.0 += vel.0;
        pos.1 += vel.1;

        println!("{:?}, {:?}, {:?}", e, *pos, vel);
    }

    println!();
}

fn main() {
    let mut dispatcher = Dispatcher::builder()
        .add_system(update_velocity.system())
        .add_system(update_position.system())
        .build();

    let mut world = World::default();
    dispatcher.register_storages(&mut world);

    world.create_entity((Position(0, 0), Velocity(1, 1)));
    world.create_entity((Position(0, 0), Velocity(2, 2)));
    world.create_entity((Position(0, 0), Velocity(3, 3), Immovable));

    for _ in 0..3 {
        dispatcher.run_seq(&mut world).unwrap();
    }
}
