use sparsey::filters::*;
use sparsey::prelude::*;

fn main() {
    let mut world = World::default();
    world.register::<i32>();
    world.register::<u32>();

    let e = world.create_entity((0_i32, 0_u32));
    let (mut i, mut u) = world.borrow::<(CompMut<i32>, CompMut<u32>)>();

    // if let Some(((i, u),)) = (mutated((&i, &u)),).get(e) {
    //     println!("{}, {}", i, u)
    // }

    if let Some((mut i, mut u)) = (&mut i, &mut u).get(e) {
        //*i = 1;
        //*u = 2;
    }

    println!("{:?}", (&i).get(e));
    println!("{:?}", mutated(&i).get(e));
    println!("{:?}", mutated((&i, &u)).get(e));
    println!("{:?}", (mutated(&i), mutated(&u)).get(e));

    //println!("{:?}", mutated((&i, &u)).get(e));

    /*
        iter!(&position, &mut entity, mutated!
        (&i, &u).mutated(), !(&mut i, &mut u).changed().maybe()
    */
}
