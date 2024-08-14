use sparsey::World;

#[test]
fn test_entities_crud() {
    let mut world = World::default();

    // World is empty at creation
    assert!(world.is_empty());
    assert_eq!(world.entities(), &[]);

    // Create an entity
    let e0 = world.create(());
    assert!(!world.is_empty());
    assert!(world.contains_entity(e0));
    assert_eq!(world.entities(), &[e0]);

    // Create a second entity
    let e1 = world.create(());
    assert!(!world.is_empty());
    assert!(world.contains_entity(e0));
    assert!(world.contains_entity(e1));
    assert_eq!(world.entities(), &[e0, e1]);

    // Destroy first entity
    assert!(world.destroy(e0));
    assert!(!world.is_empty());
    assert!(!world.contains_entity(e0));
    assert!(world.contains_entity(e1));
    assert_eq!(world.entities(), &[e1]);

    // Try to destroy missing entity
    assert!(!world.destroy(e0));

    // Remove all entities
    world.clear();
    assert!(world.is_empty());
    assert!(!world.contains_entity(e0));
    assert!(!world.contains_entity(e1));
    assert_eq!(world.entities(), &[]);
}
