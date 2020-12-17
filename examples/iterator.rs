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
    let mut world = World::new::<()>();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<Immobile>();

    world.create((Position(0.0, 0.0), Velocity(1.0, 1.0)));
    world.create((Position(1.0, 1.0), Velocity(3.0, 3.0), Immobile));
    world.create((
        Position(2.0, 2.0),
        Velocity(0.0, 0.0),
        Acceleration(0.5, 0.5),
    ));

    let (entities, mut positions, mut velocities, accelerations, immobiles) = <(
        Entities,
        CompMut<Position>,
        CompMut<Velocity>,
        Comp<Acceleration>,
        Comp<Immobile>,
    )>::borrow(&world);

    for (entity, position, velocity, acceleration, _) in (
        &entities,
        &mut positions,
        &mut velocities,
        maybe(&accelerations),
        not(&immobiles),
    )
        .iter()
    {
        println!(
            "{:?}: {:?}, {:?}, {:?}",
            entity, position, velocity, acceleration
        );
    }
}
