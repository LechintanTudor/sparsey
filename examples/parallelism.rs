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

    (&mut hps, &regens).for_each(|(hp, regen)| {
        hp.0 += regen.0;
    });
}

fn update_movement(mut pos: CompMut<Position>, vels: Comp<Velocity>) {
    println!("Update positions...");

    (&mut pos, &vels).for_each(|(pos, vel)| {
        pos.0 += vel.0;
        pos.1 += vel.1;
    });
}

fn main() {
    let mut schedule =
        Schedule::builder().add_system(update_health).add_system(update_movement).build();

    let mut world = World::default();
    schedule.set_up(&mut world);

    world.bulk_create((0..100).map(|i| (Position(0, 0), Velocity(i, i), Hp(100), HpRegen(i))));

    let mut resources = Resources::default();

    for _ in 0..5 {
        schedule.run(&mut world, &mut resources);
    }
}
