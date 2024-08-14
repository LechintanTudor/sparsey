mod common;

use common::*;
use sparsey::World;

#[test]
fn test_components_crud() {
    let mut world = World::default();
    world.register::<A>();
    world.register::<B>();

    // Create entity without any components
    let e0 = world.create(());
    assert!(!world.contains::<&A>(e0));
    assert!(!world.contains::<&B>(e0));
    assert_eq!(world.query::<&A>().get(e0), None);
    assert_eq!(world.query::<&B>().get(e0), None);

    // Add component A
    world.insert(e0, (A(1),));
    assert!(world.contains::<&A>(e0));
    assert!(!world.contains::<&B>(e0));
    assert_eq!(world.query::<&A>().get(e0), Some(&A(1)));
    assert_eq!(world.query::<&B>().get(e0), None);

    // Add component B
    world.insert(e0, (B(1),));
    assert!(world.contains::<&A>(e0));
    assert!(world.contains::<&B>(e0));
    assert_eq!(world.query::<&A>().get(e0), Some(&A(1)));
    assert_eq!(world.query::<&B>().get(e0), Some(&B(1)));

    // Remove component A
    assert_eq!(world.remove::<(A,)>(e0), (Some(A(1)),));
    assert!(!world.contains::<&A>(e0));
    assert!(world.contains::<&B>(e0));
    assert_eq!(world.query::<&A>().get(e0), None);
    assert_eq!(world.query::<&B>().get(e0), Some(&B(1)));

    // Try to remove missing component
    assert_eq!(world.remove::<(A,)>(e0), (None,));
    assert!(!world.contains::<&A>(e0));
    assert!(world.contains::<&B>(e0));
    assert_eq!(world.query::<&A>().get(e0), None);
    assert_eq!(world.query::<&B>().get(e0), Some(&B(1)));

    // Components are removed when the entity is destroyed
    world.destroy(e0);
    assert!(!world.contains::<&A>(e0));
    assert!(!world.contains::<&B>(e0));
    assert_eq!(world.query::<&A>().get(e0), None);
    assert_eq!(world.query::<&B>().get(e0), None);
}
