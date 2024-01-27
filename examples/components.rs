//! Components example.

use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Velocity(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Frozen;

fn update_velocities(mut velocities: CompMut<Velocity>, frozens: Comp<Frozen>) {
    println!("[Update velocities]");

    (&mut velocities)
        .include(&frozens)
        .for_each_with_entity(|(entity, velocity)| {
            println!("{:?} is frozen. Set its velocity to (0, 0)", entity);

            *velocity = Velocity(0, 0);
        });

    println!();
}

fn update_positions(mut positions: CompMut<Position>, velocities: Comp<Velocity>) {
    println!("[Update positions]");

    (&mut positions, &velocities).for_each_with_entity(|(entities, (position, velocity))| {
        position.0 += velocity.0;
        position.1 += velocity.1;

        println!("{:?}, {:?}, {:?}", entities, *position, velocity);
    });

    println!();
}

fn main() {
    let mut entities = EntityStorage::default();
    entities.register::<Position>();
    entities.register::<Velocity>();
    entities.register::<Frozen>();

    entities.create((Position(0, 0), Velocity(1, 1)));
    entities.create((Position(0, 0), Velocity(2, 2)));
    entities.create((Position(0, 0), Velocity(3, 3), Frozen));

    for _ in 0..3 {
        entities.run(update_velocities);
        entities.run(update_positions);
    }
}
