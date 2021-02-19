use ecstasy::data::*;
use ecstasy::dispatcher::*;
use ecstasy::query::*;
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
    let entity = Entity::new(0, Version::new(1));

    if let Some(data) = (&mut velocities, &mut accelerations, &immobiles).get(entity) {
        println!("{:?}, {:?}, {:?}", *data.0, *data.1, *data.2);
    }
}

fn movement(
    mut positions: CompMut<Position>,
    mut velocities: CompMut<Velocity>,
    accelerations: Comp<Acceleration>,
) {
    let entity = Entity::new(0, Version::new(1));

    if let Some(data) = (&mut positions, &mut velocities, &accelerations).get(entity) {
        let (mut position, mut velocity, acceleration) = data;

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

    let _ = world.create((
        Position(0.0, 0.0),
        Velocity(1.0, 1.0),
        Acceleration(1.0, 1.0),
    ));
    let _ = world.create((
        Position(0.0, 0.0),
        Velocity(1.0, 1.0),
        Acceleration(1.0, 1.0),
        Immobile,
    ));

    //world.destroy(e0);
    //world.destroy(e1);

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
