use ecstasy::data::*;
use ecstasy::dispatcher::*;
use ecstasy::resources::*;
use ecstasy::world::*;

#[derive(Debug)]
struct Immobile;

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Velocity(f32, f32);

#[derive(Debug)]
struct Acceleration(f32, f32);

fn immobile(
    immobiles: Comp<Immobile>,
    mut velocities: CompMut<Velocity>,
    mut accelerations: CompMut<Acceleration>,
) {
    for (mut velocity, mut acceleration, _) in
        (&mut velocities, &mut accelerations, &immobiles).join()
    {
        *velocity = Velocity(0.0, 0.0);
        *acceleration = Acceleration(0.0, 0.0);
    }
}

fn movement(
    mut positions: CompMut<Position>,
    mut velocities: CompMut<Velocity>,
    accelerations: Comp<Acceleration>,
) {
    for (mut position, mut velocity, acceleration) in
        (&mut positions, &mut velocities, &accelerations).join()
    {
        velocity.0 += acceleration.0;
        velocity.1 += acceleration.1;

        position.0 += velocity.0;
        position.1 += velocity.1;

        println!("{:?}, {:?}, {:?}", *position, *velocity, *acceleration);
    }

    println!();
}

fn spawn(mut commands: Commands) {
    commands.queue(|world, _| {
        world.create((
            Position(0.0, 0.0),
            Velocity(0.0, 0.0),
            Acceleration(-1.0, 1.0),
        ));
    });
}

fn main() {
    let mut world = World::new::<()>();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<Immobile>();

    world.create((
        Position(0.0, 0.0),
        Velocity(1.0, 1.0),
        Acceleration(1.0, 1.0),
    ));
    world.create((
        Position(0.0, 0.0),
        Velocity(1.0, 1.0),
        Acceleration(1.0, 1.0),
        Immobile,
    ));

    let mut resources = Resources::default();

    let mut dispatcher = Dispatcher::builder()
        .with_system(immobile.system())
        .with_system(movement.system())
        .with_system(spawn.system())
        .build();

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(dispatcher.max_parallel_systems())
        .build()
        .unwrap();

    println!("Maximum parallelism: {}", dispatcher.max_parallel_systems());

    for _ in 0..3 {
        dispatcher.run(&mut world, &mut resources, &thread_pool);
    }
}
