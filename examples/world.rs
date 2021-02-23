use ecstasy::dispatcher::*;
use ecstasy::query::*;
use ecstasy::resources::*;
use ecstasy::world::*;

#[rustfmt::skip]
pub type Layout = (
    (
        (u16, u32),
        (u16, u32, u64),
    ),
);

fn check(a: Comp<u16>, b: Comp<u32>, c: Comp<u64>) {
    println!("AB: {}", (&a, &b).is_grouped());

    for (a, b) in (&a, &b).iter() {
        println!("{}, {}", a, b);
    }

    println!();

    println!("ABC: {}", (&a, &b, &c).is_grouped());

    for (a, b, c) in (&a, &b, &c).iter() {
        println!("{}, {}, {}", a, b, c);
    }

    println!("\n");
}

fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    world.register::<u16>();
    world.register::<u32>();
    world.register::<u64>();

    world.create((1_u16, 2_u32, 3_u64));
    world.create((2_u32, 3_u64));
    world.create((1_u16, 3_u64));
    world.create((1_u16, 2_u32, 3_u64));

    let mut dispatcher = Dispatcher::builder().add_system(check.system()).build();

    dispatcher.run_thread_local(&mut world, &mut resources);

    world.set_layout(&Layout::world_layout());

    dispatcher.run_thread_local(&mut world, &mut resources);
}
