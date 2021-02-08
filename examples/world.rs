use ecstasy::data::*;
use ecstasy::dispatcher::*;
use ecstasy::resources::*;
use ecstasy::storage::*;
use ecstasy::world::*;

struct Immobile;

struct Position(f32, f32);

struct Velocity(f32, f32);

struct Acceleration(f32, f32);

fn immobile(
    immobiles: Comp<Immobile>,
    // mut velocities: CompMut<Velocity>,
    // mut accelerations: CompMut<Acceleration>,
) {
    // for (mut velocity, mut acceleration, _) in
    //     (&mut velocities, &mut accelerations, &immobiles).join()
    // {
    //     *velocity = Velocity(0.0, 0.0);
    //     *acceleration = Acceleration(0.0, 0.0);
    // }
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
    }
}

fn main() {
    let system = immobile.thread_local_system();
    // let dispatcher = Dispatcher::builder().with_thread_local_system(immobile.thread_local_system());
}
