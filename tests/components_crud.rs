mod common;

use common::*;
use sparsey::prelude::*;

#[test]
fn test_components_crud() {
    let mut entities = EntityStorage::default();
    entities.register::<A>();
    entities.register::<B>();

    // Create entity without any components
    let e0 = entities.create(());
    entities.run(|a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(!b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(a.get(e0), None);
    });

    // Add component A
    entities.insert(e0, (A(1),));
    entities.run(|a: Comp<A>, b: Comp<B>| {
        assert!(a.contains(e0));
        assert!(!b.contains(e0));

        assert_eq!(a.get(e0), Some(&A(1)));
        assert_eq!(b.get(e0), None);
    });

    // Add component B
    entities.insert(e0, (B(1),));
    entities.run(|a: Comp<A>, b: Comp<B>| {
        assert!(a.contains(e0));
        assert!(b.contains(e0));

        assert_eq!(a.get(e0), Some(&A(1)));
        assert_eq!(b.get(e0), Some(&B(1)));
    });

    // Remove component A
    assert_eq!(entities.remove::<(A,)>(e0), (Some(A(1)),));
    entities.run(|a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(b.get(e0), Some(&B(1)));
    });

    // Try to remove missing component
    assert_eq!(entities.remove::<(A,)>(e0), (None,));
    entities.run(|a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(b.get(e0), Some(&B(1)));
    });

    // Components are removed when the entity is destroyed
    entities.destroy(e0);
    entities.run(|a: Comp<A>, b: Comp<B>| {
        assert!(!a.contains(e0));
        assert!(!b.contains(e0));

        assert_eq!(a.get(e0), None);
        assert_eq!(a.get(e0), None);
    });
}
