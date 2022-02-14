mod common;

use common::*;
use sparsey::prelude::*;
use std::any::TypeId;

#[test]
fn test_entities() {
    let mut world = World::default();

    // Create
    let e0 = world.create(());
    assert!(world.contains(e0));
    assert_eq!(world.entities(), &[e0]);

    // Destroy
    assert!(world.destroy(e0));
    assert!(!world.destroy(e0));
    assert!(!world.contains(e0));
    assert_eq!(world.entities(), &[]);

    // Clear
    let e0 = world.create(());
    let e1 = world.create(());
    world.clear();
    assert!(!world.contains(e0));
    assert!(!world.contains(e1));
    assert_eq!(world.entities(), &[]);
}

#[test]
fn test_register() {
    let mut world = World::default();

    assert!(!world.is_registered(&TypeId::of::<A>()));
    assert!(!world.is_registered(&TypeId::of::<B>()));

    world.register::<A>();
    world.register::<B>();

    assert!(world.is_registered(&TypeId::of::<A>()));
    assert!(world.is_registered(&TypeId::of::<B>()));
}

#[test]
fn test_components() {
    let mut world = World::default();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();

    // Create
    let e0 = world.create((A(0), B(0)));

    {
        let a = world.borrow::<A>();
        let b = world.borrow::<B>();

        assert_eq!(a.get(e0).copied(), Some(A(0)));
        assert_eq!(b.get(e0).copied(), Some(B(0)));
    }

    // Insert
    assert!(world.insert(e0, (C(0),)));

    {
        let a = world.borrow::<A>();
        let b = world.borrow::<B>();
        let c = world.borrow::<C>();

        assert_eq!(a.get(e0).copied(), Some(A(0)));
        assert_eq!(b.get(e0).copied(), Some(B(0)));
        assert_eq!(c.get(e0).copied(), Some(C(0)));
    }

    // Remove
    assert_eq!(world.remove::<(A, B)>(e0), (Some(A(0)), Some(B(0))));
    assert_eq!(world.remove::<(A, B)>(e0), (None, None));

    {
        let a = world.borrow::<A>();
        let b = world.borrow::<B>();
        let c = world.borrow::<C>();

        assert_eq!(a.get(e0), None);
        assert_eq!(b.get(e0), None);
        assert_eq!(c.get(e0).copied(), Some(C(0)));
    }

    // Delete
    world.delete::<(C,)>(e0);

    {
        let c = world.borrow::<C>();
        assert_eq!(c.get(e0), None);
    }
}
