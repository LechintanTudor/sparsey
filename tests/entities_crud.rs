use sparsey::prelude::*;

#[test]
fn test_entities_crud() {
    let mut entities = EntityStorage::default();

    // World is empty at creation
    assert!(entities.is_empty());
    assert_eq!(entities.entities(), &[]);

    // Create an entity
    let e0 = entities.create(());
    assert!(!entities.is_empty());
    assert!(entities.contains(e0));
    assert_eq!(entities.entities(), &[e0]);

    // Create a second entity
    let e1 = entities.create(());
    assert!(!entities.is_empty());
    assert!(entities.contains(e0));
    assert!(entities.contains(e1));
    assert_eq!(entities.entities(), &[e0, e1]);

    // Destroy first entity
    assert!(entities.destroy(e0));
    assert!(!entities.is_empty());
    assert!(!entities.contains(e0));
    assert!(entities.contains(e1));
    assert_eq!(entities.entities(), &[e1]);

    // Try to destroy missing entity
    assert!(!entities.destroy(e0));

    // Remove all entities
    entities.clear();
    assert!(entities.is_empty());
    assert!(!entities.contains(e0));
    assert!(!entities.contains(e1));
    assert_eq!(entities.entities(), &[]);
}
