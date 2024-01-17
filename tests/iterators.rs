mod common;

use common::*;
use sparsey::prelude::*;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_sparse() {
    let mut entities = EntityStorage::default();
    entities.register::<A>();
    entities.register::<B>();
    entities.register::<C>();
    entities.register::<D>();

    let e0 = entities.create((A(0), B(0)));
    let e1 = entities.create((A(1), B(1), C(1)));
    let e2 = entities.create((A(2), B(2), C(2), D(2)));

    let a = entities.borrow::<A>();
    let b = entities.borrow::<B>();
    let c = entities.borrow::<C>();
    let d = entities.borrow::<D>();

    let i = (&a, &b).iter();
    assert!(i.is_sparse());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0, e1, e2]));

    let i = (&a, &b, &c).iter();
    assert!(i.is_sparse());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1, e2]));

    let i = (&a, &b, &c, &d).iter();
    assert!(i.is_sparse());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e2]));

    let i = (&a, &b).exclude(&c).iter();
    assert!(i.is_sparse());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0]));

    let i = (&a, &b, &c).exclude(&d).iter();
    assert!(i.is_sparse());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1]));
}

#[test]
fn test_dense() {
    let layout = GroupLayout::builder()
        .add_group::<(A, B)>()
        .add_group::<(A, B, C)>()
        .add_group::<(A, B, C, D)>()
        .build();

    let mut entities = EntityStorage::new(&layout);
    let e0 = entities.create((A(0), B(0)));
    let e1 = entities.create((A(1), B(1), C(1)));
    let e2 = entities.create((A(2), B(2), C(2), D(2)));

    let a = entities.borrow::<A>();
    let b = entities.borrow::<B>();
    let c = entities.borrow::<C>();
    let d = entities.borrow::<D>();

    let i = (&a, &b).iter();
    assert!(i.is_dense());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0, e1, e2]));

    let i = (&a, &b, &c).iter();
    assert!(i.is_dense());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1, e2]));

    let i = (&a, &b, &c, &d).iter();
    assert!(i.is_dense());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e2]));

    let i = (&a, &b).exclude(&c).iter();
    assert!(i.is_dense());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e0]));

    let i = (&a, &b, &c).exclude(&d).iter();
    assert!(i.is_dense());
    let e = i.with_entity().map(|(e, _)| e).collect::<HashSet<_>>();
    assert_eq!(e, HashSet::from_iter([e1]));
}
