mod common;

use self::common::*;
use sparsey::prelude::*;

#[test]
fn test_components_register() {
    let mut entities = EntityStorage::default();

    // No components are registered at creation
    assert!(!entities.is_registered::<A>());
    assert!(!entities.is_registered::<B>());

    // Register component A
    entities.register::<A>();
    assert!(entities.is_registered::<A>());

    // Register component B
    entities.register::<B>();
    assert!(entities.is_registered::<A>());
    assert!(entities.is_registered::<B>());

    // Components remain registered even after clear
    entities.clear();
    assert!(entities.is_registered::<A>());
    assert!(entities.is_registered::<B>());

    // Components remain registered even after reset
    entities.reset();
    assert!(entities.is_registered::<A>());
    assert!(entities.is_registered::<B>());
}
