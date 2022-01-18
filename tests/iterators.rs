mod common;

use common::*;
use sparsey::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_sparse() {
    let mut world = World::default();
    let e0 = world.create_entity((A(0), B(0)));
    let e1 = world.create_entity((A(1), B(1), C(1)));
    let e2 = world.create_entity((A(2), B(2), C(2), D(2)));

    let (a, b, c, d) = world.borrow::<(Comp<A>, Comp<B>, Comp<C>, Comp<D>)>();

    let i = (&a, &b).iter();
    assert!(i.is_sparse());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0, e1, e2]));

    let i = (&a, &b, &c).iter();
    assert!(i.is_sparse());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1, e2]));

    let i = (&a, &b, &c, &d).iter();
    assert!(i.is_sparse());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e2]));

    let i = (&a, &b).exclude(&c).iter();
    assert!(i.is_sparse());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0]));

    let i = (&a, &b, &c).exclude(&d).iter();
    assert!(i.is_sparse());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1]));
}

#[test]
fn test_dense() {
    let layout = Layout::builder()
        .add_group(<(A, B)>::group())
        .add_group(<(A, B, C)>::group())
        .add_group(<(A, B, C, D)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    let e0 = world.create_entity((A(0), B(0)));
    let e1 = world.create_entity((A(1), B(1), C(1)));
    let e2 = world.create_entity((A(2), B(2), C(2), D(2)));

    let (a, b, c, d) = world.borrow::<(Comp<A>, Comp<B>, Comp<C>, Comp<D>)>();

    let i = (&a, &b).iter();
    assert!(i.is_dense());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0, e1, e2]));

    let i = (&a, &b, &c).iter();
    assert!(i.is_dense());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1, e2]));

    let i = (&a, &b, &c, &d).iter();
    assert!(i.is_dense());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e2]));

    let i = (&a, &b).exclude(&c).iter();
    assert!(i.is_dense());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0]));

    let i = (&a, &b, &c).exclude(&d).iter();
    assert!(i.is_dense());
    let e = i.entities().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1]));
}
