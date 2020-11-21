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
