#![allow(unused_variables)]

use ecstasy::prelude::*;

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

#[derive(Debug)]
struct C;

#[derive(Debug)]
struct D;

#[derive(Debug)]
struct E;

type WorldLayout = (((A, B), (A, B, C)), ((D, E),));

fn main() {
    let mut world = World::new::<WorldLayout>();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();
    world.register::<D>();
    world.register::<E>();

    let e0 = world.create((A, B));
    let e1 = world.create((A, B, C));
    let e2 = world.create((A, B, C, D, E));

    unsafe {
        let (mut a, mut b, mut c, d, e) =
            <(CompMut<A>, CompMut<B>, CompMut<C>, CompMut<D>, CompMut<E>)>::get_from_world(&world);

        let group_iter = DenseIter3::new_unchecked(&mut a, &mut b, &mut c, 2);

        for (a, b, c) in group_iter {
            println!("{:?}, {:?}, {:?}", *a, *b, *c);
        }
    }
}
