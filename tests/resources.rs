mod common;

use common::*;
use sparsey::prelude::*;
use std::any::TypeId;

#[test]
fn test_crud() {
    let mut world = World::default();

    // Insert
    assert!(!world.contains_resource(&TypeId::of::<A>()));
    assert_eq!(world.insert_resource(A(0)), None);
    assert_eq!(world.insert_resource(A(1)), Some(A(0)));
    assert!(world.contains_resource(&TypeId::of::<A>()));

    // Borrow
    assert_eq!(*world.borrow::<Res<A>>(), A(1));

    // Remove
    assert_eq!(world.remove_resource::<A>(), Some(A(1)));
    assert_eq!(world.remove_resource::<A>(), None);
    assert_eq!(world.remove_resource::<B>(), None);
    assert!(!world.contains_resource(&TypeId::of::<A>()));
    assert!(!world.contains_resource(&TypeId::of::<B>()));

    // Clear
    world.insert_resource(A(0));
    world.insert_resource(B(0));
    world.clear_resources();
    assert!(!world.contains_resource(&TypeId::of::<A>()));
    assert!(!world.contains_resource(&TypeId::of::<B>()));
}
