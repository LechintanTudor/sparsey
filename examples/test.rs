use ecstasy::*;

#[derive(Copy, Clone, Debug)]
struct Position(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Velocity(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Immovable;

fn main() {
    let mut entities = Entities::default();
    let e1 = entities.create();
    let e2 = entities.create();
    let e3 = entities.create();
    let e4 = entities.create();

    let mut positions = SparseSet::<Position>::default();
    positions.insert(e4, Position(4.0, 4.0));
    positions.insert(e3, Position(3.0, 3.0));
    positions.insert(e2, Position(2.0, 2.0));
    positions.insert(e1, Position(1.0, 1.0));

    let mut velocities = SparseSet::<Velocity>::default();
    velocities.insert(e4, Velocity(4.0, 4.0));
    velocities.insert(e3, Velocity(3.0, 3.0));
    velocities.insert(e2, Velocity(2.0, 2.0));
    velocities.insert(e1, Velocity(1.0, 1.0));

    let mut immovables = SparseSet::<Immovable>::default();
    immovables.insert(e2, Immovable);
    immovables.insert(e3, Immovable);

    let iterator = Iterator4::<Entity, &mut Position, &Velocity, Option<&Immovable>>::new(
        &entities,
        &mut positions,
        &velocities,
        &immovables,
    );

    for (entity, position, velocity, immovable) in iterator {
        println!(
            "{:?}, {:?}, {:?}, {:?}",
            entity, position, velocity, immovable
        )
    }
}
