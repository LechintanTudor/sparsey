use rayon::ThreadPoolBuilder;
use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Hp(i32);

#[derive(Clone, Copy, Debug)]
struct HpRegen(i32);

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Velocity(i32, i32);

fn update_health(mut hps: CompMut<Hp>, regens: Comp<HpRegen>) {
    println!("Update HPs...");
    for (hp, regen) in (&mut hps, &regens).iter() {
        hp.0 += regen.0;
    }
}

fn update_movement(mut pos: CompMut<Position>, vels: Comp<Velocity>) {
    println!("Update positions...");
    for (pos, vel) in (&mut pos, &vels).iter() {
        pos.0 += vel.0;
        pos.1 += vel.1;
    }
}

fn main() {
    let mut dispatcher = Dispatcher::builder()
        .add_system(update_health.system())
        .add_system(update_movement.system())
        .build();

    let mut world = World::default();
    dispatcher.register_storages(&mut world);

    world.create_entities((0..100).map(|i| (Position(0, 0), Velocity(i, i), Hp(100), HpRegen(i))));

    let num_threads = dispatcher.max_concurrecy();
    println!("Create thread pool with {} threads", num_threads);

    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    for _ in 0..3 {
        dispatcher.run_par(&mut world, &thread_pool).unwrap();
    }
}
