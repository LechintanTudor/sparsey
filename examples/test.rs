use ecstasy::{prelude::*, storage::SparseSet};

#[derive(Copy, Clone, Debug)]
struct Position(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Velocity(f32, f32);

#[derive(Copy, Clone, Debug)]
struct Acceleration(f32, f32);

fn main() {
    let mut p = SparseSet::<Position>::default();
    p.insert(Entity::new(2), Position(2.0, 2.0));
    p.insert(Entity::new(1), Position(1.0, 1.0));
    p.insert(Entity::new(0), Position(0.0, 0.0));

    let mut v = SparseSet::<Velocity>::default();
    v.insert(Entity::new(0), Velocity(0.0, 0.0));
    v.insert(Entity::new(1), Velocity(1.0, 1.0));
    v.insert(Entity::new(2), Velocity(2.0, 2.0));

    let mut a = SparseSet::<Acceleration>::default();
    a.insert(Entity::new(1), Acceleration(1.0, 1.0));
    a.insert(Entity::new(2), Acceleration(2.0, 2.0));

    for (pos, vel, acc) in (&mut p, &mut v, maybe(&a)).iter() {
        if let Some(acc) = acc {
            vel.0 += acc.0;
            vel.1 += acc.1;
        }

        pos.0 += vel.0;
        pos.1 += vel.1;

        println!("{:?}", pos);
    }
}
