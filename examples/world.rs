use ecstasy::*;
use std::iter;

#[derive(Debug)]
pub struct Droppable(u32);

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping: {}", self.0);
    }
}

fn check(mut a: CompMut<u16>, b: Comp<u32>, c: Comp<u64>, d: Comp<Droppable>) {
    if let Some((e, (_, _, _))) = (&a, &b, &c).slice_entities() {
        println!("Slice length: {}", e.len());
    }

    if let Some(e) = (&a, &b, &c).entities() {
        println!("Slice length: {}", e.len());
    }

    println!();

    for (e, (a, b)) in (&a, &b).iter().entities() {
        println!("{:?} => {}, {}", e, a, b);
    }

    println!();

    for (e, a) in a.iter_mut().entities() {
        println!("{:?} => {}", e, *a);
    }

    println!();

    for (e, (a, b, c)) in (&a, &b, &c).iter().entities() {
        println!("{:?} => {}, {}, {}", e, a, b, c);
    }

    println!();

    for (e, d) in d.iter().entities() {
        println!("{:?} => {:?}", e, d);
    }

    println!("\n");
}

fn main() {
    let layout = Layout::builder()
        .add_group(<(u16, u32)>::group())
        .add_group(<(u16, u32, u64)>::group())
        .add_group(<(u16, u32, u64, u128)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    let mut resources = Resources::default();

    let mut dispatcher = Dispatcher::builder()
        .add_system(check.system())
        .add_flush()
        .build();

    dispatcher.set_up(&mut world);

    world.clear_flags();
    world.extend(iter::repeat((1_u16, 2_u32, 3_u64)).take(10));
    world.extend((1..=5).into_iter().map(|i| (Droppable(i),)));

    dispatcher.run_locally(&mut world, &mut resources);
}
