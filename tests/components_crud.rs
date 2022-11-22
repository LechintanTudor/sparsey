mod common;

use common::*;
use sparsey::prelude::*;

#[test]
fn test_components_crud() {
    let mut world = World::default();
    world.register::<A>();
    world.register::<B>();

    let resources = Resources::default();

    // Create entity without any components
    let e0 = world.create(());
    sparsey::run(&world, &resources, |a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(!b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(a.get(e0), None);
    });

    // Add component A
    world.insert(e0, (A(1),));
    sparsey::run(&world, &resources, |a: Comp<A>, b: Comp<B>| {
        assert!(a.contains(e0));
        assert!(!b.contains(e0));

        assert_eq!(a.get(e0), Some(&A(1)));
        assert_eq!(b.get(e0), None);
    });

    // Add component B
    world.insert(e0, (B(1),));
    sparsey::run(&world, &resources, |a: Comp<A>, b: Comp<B>| {
        assert!(a.contains(e0));
        assert!(b.contains(e0));

        assert_eq!(a.get(e0), Some(&A(1)));
        assert_eq!(b.get(e0), Some(&B(1)));
    });

    // Remove component A
    assert_eq!(world.remove::<(A,)>(e0), (Some(A(1)),));
    sparsey::run(&world, &resources, |a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(b.get(e0), Some(&B(1)));
    });

    // Try to remove missing component
    assert_eq!(world.remove::<(A,)>(e0), (None,));
    sparsey::run(&world, &resources, |a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(b.get(e0), Some(&B(1)));
    });

    // Components are removed when the entity is destroyed
    world.destroy(e0);
    sparsey::run(&world, &resources, |a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(!b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(a.get(e0), None);
    });
}
