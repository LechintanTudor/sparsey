#![allow(unused_variables)]

use ecstasy::data::filter::*;
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

#[rustfmt::skip]
type WorldLayout = (
    (
        (A, B), 
        (A, B, C), 
    ),
    (
        (D, E),
    ),
);

fn main() {
    let mut world = World::new::<WorldLayout>();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();
    world.register::<D>();
    world.register::<E>();

    let e0 = world.create((A,));
    let e1 = world.create((A, B, C));
    let e2 = world.create((A, B, C, D, E));

    println!("{:?}, {:?}, {:?}", e0, e1, e2);

    /*
    let (mut a, mut b) = world.borrow::<(CompMut<A>, CompMut<B>)>();
    let (mut c, mut d) = resources.borrow::<(ResMut<C>, ResMut<D>)>();
    let (e, f, g, h) = registry.borrow::<(Comp<E>, Comp<F>, Res<G>, Res<H>)>();
    */

    {
        println!("Before maintain:");

        let (mut a, mut b) = <(CompMut<A>, CompMut<B>)>::borrow_world(&world);

        for (a, b) in (added(&mut a), &mut b).join() {
            println!("{:?}, {:?}", *a, *b);
        }
    }

    world.maintain();

    {
        println!("\nAfter maintain:");

        let (mut a, mut b) = <(CompMut<A>, CompMut<B>)>::borrow_world(&world);

        for (a, b) in (added(&mut a), &mut b).join() {
            println!("{:?}, {:?}", *a, *b);
        }
    }
}
