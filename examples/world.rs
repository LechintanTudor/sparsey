#![allow(unused_variables)]

use ecstasy::prelude::*;
use ecstasy::registry::RawViewMut;

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

    print_sets(&world);

    println!("{:?}", world.remove::<(B,)>(e1));
    println!("{:?}", world.remove::<(C,)>(e2));

    print_sets(&world);
}

fn print_sets(world: &World) {
    let (a, b, c, d, e) = <(
        RawViewMut<A>,
        RawViewMut<B>,
        RawViewMut<C>,
        RawViewMut<D>,
        RawViewMut<E>,
    )>::borrow(&world);

    println!("\nSets:");
    println!("A: {:?}", a.set.dense());
    println!("B: {:?}", b.set.dense());
    println!("C: {:?}", c.set.dense());
    println!("D: {:?}", d.set.dense());
    println!("E: {:?}", e.set.dense());

    println!("\nGroups:");
    unsafe {
        println!("{:?}", world.groups.get_unchecked(0).subgroup_lengths());
        println!("{:?}", world.groups.get_unchecked(1).subgroup_lengths());
    }

    println!();
}
