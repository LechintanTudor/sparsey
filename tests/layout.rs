mod common;

use common::*;
use sparsey::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_iterators() {
    let layout = Layout::builder()
        .add_group(<(A, B)>::group())
        .add_group(<(A, B, C, D)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    let e0 = world.create_entity((A(0), B(0)));
    let e1 = world.create_entity((A(1), B(1)));
    let e2 = world.create_entity((A(2), B(2), C(2), D(2)));
    let e3 = world.create_entity((A(3), B(3), C(3), D(3)));

    let (a, b, c, d) = world.borrow::<(CompMut<A>, CompMut<B>, CompMut<C>, CompMut<D>)>();

    assert!((&a, &b).iter().is_dense());
    let entities = (&a, &b)
        .iter()
        .entities()
        .map(|(e, (a, b))| {
            assert_eq!(a.0, b.0);
            e
        })
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e0, e1, e2, e3]));

    assert!((&a, &b, &c, &d).iter().is_dense());
    let entities = (&a, &b, &c, &d)
        .iter()
        .entities()
        .map(|(e, (a, b, c, d))| {
            assert_eq!(a.0, b.0);
            assert_eq!(b.0, c.0);
            assert_eq!(c.0, d.0);
            e
        })
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e2, e3]));

    assert!((&a, &b).exclude((&c, &d)).iter().is_dense());
    let entities = (&a, &b)
        .exclude((&c, &d))
        .iter()
        .entities()
        .map(|(e, (a, b))| {
            assert_eq!(a.0, b.0);
            e
        })
        .collect::<HashSet<_>>();
    assert_eq!(entities, HashSet::from_iter([e0, e1]));
}

#[test]
fn test_slicing() {
    let layout = Layout::builder()
        .add_group(<(A, B)>::group())
        .add_group(<(A, B, C, D)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    let e0 = world.create_entity((A(0), B(0)));
    let e1 = world.create_entity((A(1), B(1)));
    let e2 = world.create_entity((A(2), B(2), C(2), D(2)));
    let e3 = world.create_entity((A(3), B(3), C(3), D(3)));

    let (a, b, c, d) = world.borrow::<(CompMut<A>, CompMut<B>, CompMut<C>, CompMut<D>)>();

    {
        let (e, (a, b)) = (&a, &b).entities_components().unwrap();
        assert!(a.iter().zip(b).all(|(a, b)| a.0 == b.0));
        assert_eq!(
            HashSet::<Entity>::from_iter(e.iter().copied()),
            HashSet::from_iter([e0, e1, e2, e3])
        );
    }

    {
        let (e, (a, b, c, d)) = (&a, &b, &c, &d).entities_components().unwrap();
        assert!(a
            .iter()
            .zip(b)
            .zip(c)
            .zip(d)
            .all(|(((a, b), c), d)| a.0 == b.0 && b.0 == c.0 && c.0 == d.0));
        assert_eq!(
            HashSet::<Entity>::from_iter(e.iter().copied()),
            HashSet::from_iter([e2, e3])
        );
    }

    {
        let (e, (a, b)) = (&a, &b).exclude((&c, &d)).entities_components().unwrap();
        assert!(a.iter().zip(b).all(|(a, b)| a.0 == b.0));
        assert_eq!(
            HashSet::<Entity>::from_iter(e.iter().copied()),
            HashSet::from_iter([e0, e1])
        );
    }
}
