mod common;

use self::common::*;
use sparsey::World;

#[test]
fn test_components_register() {
    let mut world = World::default();

    // No components are registered at creation
    assert!(!world.is_registered::<A>());
    assert!(!world.is_registered::<B>());

    // Register component A
    world.register::<A>();
    assert!(world.is_registered::<A>());

    // Register component B
    world.register::<B>();
    assert!(world.is_registered::<A>());
    assert!(world.is_registered::<B>());

    // Components remain registered even after clear
    world.clear();
    assert!(world.is_registered::<A>());
    assert!(world.is_registered::<B>());

    // Components remain registered even after reset
    world.reset();
    assert!(world.is_registered::<A>());
    assert!(world.is_registered::<B>());
}
