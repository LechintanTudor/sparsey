mod common;

use common::*;
use sparsey::filters::*;
use sparsey::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_modifiers() {
    let mut world = World::default();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();

    let e0 = world.create_entity((A(0),));
    let e1 = world.create_entity((A(1), B(1)));
    let e2 = world.create_entity((A(2), B(2), C(2)));
    let e3 = world.create_entity((A(3), C(3)));

    let (a, b, c) = world.borrow::<(Comp<A>, Comp<B>, Comp<C>)>();

    // Include
    let entities = (&a,)
        .include(&b)
        .iter()
        .entities()
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e1, e2]));

    // Exclude
    let entities = (&a,)
        .exclude(&c)
        .iter()
        .entities()
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e0, e1]));

    // Include + Exclude
    let entities = (&a,)
        .include(&c)
        .exclude(&b)
        .iter()
        .entities()
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e3]));
}

#[test]
fn test_filters() {
    let mut world = World::default();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();

    let e0 = world.create_entity((A(0), B(0)));
    let e1 = world.create_entity((A(1), B(1), C(1)));
    let e2 = world.create_entity((A(2), C(2)));

    let (a, b, c) = world.borrow::<(Comp<A>, Comp<B>, Comp<C>)>();

    // And
    let entities = (&a,)
        .filter(contains(&b) & contains(&c))
        .iter()
        .entities()
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e1]));

    // Or
    let entities = (&a,)
        .filter(contains(&b) | contains(&c))
        .iter()
        .entities()
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e0, e1, e2]));

    // Xor
    let entities = (&a,)
        .filter(contains(&b) ^ contains(&c))
        .iter()
        .entities()
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e0, e2]));
}
