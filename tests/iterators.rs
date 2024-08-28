mod common;

use common::*;
use sparsey::entity::Entity;
use sparsey::query::Query;
use sparsey::World;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_sparse() {
    let mut world = World::builder()
        .register::<A>()
        .register::<B>()
        .register::<C>()
        .register::<D>()
        .build();

    let e0 = world.create((A(0), B(0)));
    let e1 = world.create((A(1), B(1), C(1)));
    let e2 = world.create((A(2), B(2), C(2), D(2)));

    let world = &mut world;
    test_iter::<(&A, &B), ()>(world, false, &[e0, e1, e2]);
    test_iter::<(&A, &B, &C), ()>(world, false, &[e1, e2]);
    test_iter::<(&A, &B, &C, &D), ()>(world, false, &[e2]);
    test_iter::<(&A, &B), &C>(world, false, &[e0]);
    test_iter::<(&A, &B, &C), &D>(world, false, &[e1]);
}

#[test]
fn test_dense() {
    let mut world = World::builder()
        .add_group::<(A, B)>()
        .add_group::<(A, B, C)>()
        .add_group::<(A, B, C, D)>()
        .build();

    let e0 = world.create((A(0), B(0)));
    let e1 = world.create((A(1), B(1), C(1)));
    let e2 = world.create((A(2), B(2), C(2), D(2)));

    let world = &mut world;
    test_iter::<(&A, &B), ()>(world, true, &[e0, e1, e2]);
    test_iter::<(&A, &B, &C), ()>(world, true, &[e1, e2]);
    test_iter::<(&A, &B, &C, &D), ()>(world, true, &[e2]);
    test_iter::<(&A, &B), &C>(world, true, &[e0]);
    test_iter::<(&A, &B, &C), &D>(world, true, &[e1]);
}

#[track_caller]
fn test_iter<I, E>(world: &World, is_dense: bool, expected_entities: &[Entity])
where
    I: Query,
    E: Query,
{
    let mut query = world.query_all::<Entity>().include::<I>().exclude::<E>();

    let iter = query.iter();
    assert_eq!(iter.is_dense(), is_dense);

    let entities = iter.collect::<HashSet<_>>();
    assert_eq!(
        entities,
        HashSet::from_iter(expected_entities.iter().copied()),
    );
}
