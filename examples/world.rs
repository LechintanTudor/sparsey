use ecstasy::dispatcher::*;
use ecstasy::query::*;
use ecstasy::resources::*;
use ecstasy::world::*;
use std::iter;

fn check(a: Comp<u16>, b: Comp<u32>, c: Comp<u64>) {
    for (e, (a, b)) in (&a, &b).iter().entities() {
        println!("{:?}, {}, {}", e, a, b);
    }

    for (e, (a, b, c)) in (&a, &b, &c).iter().entities() {
        println!("{:?}, {}, {}, {}", e, a, b, c);
    }
}

// TODO: Dispatcher set_up

fn main() {
    let layout = Layout::builder()
        .add_group(<(u16, u32)>::group())
        .add_group(<(u16, u32, u64, u128)>::group())
        .add_group(<(u16, u64, u32)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    let mut resources = Resources::default();

    world.register::<u16>();
    world.register::<u32>();
    world.register::<u64>();

    world.extend(iter::repeat((1_u16, 2_u32, 3_u64)).take(10));

    let mut dispatcher = Dispatcher::builder().add_system(check.system()).build();
    dispatcher.run_locally(&mut world, &mut resources);
}
