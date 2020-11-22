use ecstasy::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Position(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Velocity(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Acceleration(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Immobile;

fn main() {
    let mut world = World::default();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<Immobile>();

    world.push((Position(0.0, 0.0), Velocity(1.0, 1.0)));
    world.push((Position(1.0, 1.0), Velocity(3.0, 3.0), Immobile));
    world.push((Position(2.0, 2.0), Velocity(0.0, 0.0), Acceleration(0.5, 0.5)));

    let (mut poss, mut vels, accels, immobs) = <(
        CompMut<Position>,
        CompMut<Velocity>,
        Comp<Acceleration>,
        Comp<Immobile>,
    )>::borrow(&world);

    for (pos, vel, accel, _) in (&mut poss, &mut vels, maybe(&accels), not(&immobs)).iter() {
        println!("{:?}, {:?}, {:?}", pos, vel, accel);
    }
}
