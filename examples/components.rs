use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Velocity(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Frozen;

fn update_velocity(mut vel: CompMut<Velocity>, frz: Comp<Frozen>) {
    println!("[Update velocities]");

    (&mut vel).include(&frz).for_each_with_entity(|(e, vel)| {
        println!("{:?} is frozen. Set its velocity to (0, 0)", e);

        *vel = Velocity(0, 0);
    });

    println!();
}

fn update_position(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
    println!("[Update positions]");

    (&mut pos, &vel).for_each_with_entity(|(e, (pos, vel))| {
        pos.0 += vel.0;
        pos.1 += vel.1;

        println!("{:?}, {:?}, {:?}", e, *pos, vel);
    });

    println!();
}

fn main() {
    let mut schedule =
        Schedule::builder().add_system(update_velocity).add_system(update_position).build();

    let mut world = World::default();
    schedule.set_up(&mut world);

    world.create((Position(0, 0), Velocity(1, 1)));
    world.create((Position(0, 0), Velocity(2, 2)));
    world.create((Position(0, 0), Velocity(3, 3), Frozen));

    let mut resources = Resources::default();

    for _ in 0..3 {
        schedule.run_seq(&mut world, &mut resources);
    }
}
