mod common;

use common::*;
use sparsey::prelude::*;

#[test]
fn test_resources_crud() {
    let mut resources = ResourceStorage::default();

    // Resources is empty at creation
    assert!(resources.is_empty());
    assert_eq!(resources.len(), 0);

    // Add resource A
    assert_eq!(resources.insert(A(0)), None);
    assert!(!resources.is_empty());
    assert_eq!(resources.len(), 1);
    assert!(resources.contains::<A>());
    assert!(resources.try_borrow::<A>().is_some());

    // Replace resource A
    assert_eq!(resources.insert(A(1)), Some(A(0)));
    assert!(!resources.is_empty());
    assert_eq!(resources.len(), 1);

    // Add resource B
    assert_eq!(resources.insert(B(1)), None);
    assert!(!resources.is_empty());
    assert_eq!(resources.len(), 2);
    assert!(resources.contains::<A>());
    assert!(resources.contains::<B>());
    assert!(resources.try_borrow::<A>().is_some());
    assert!(resources.try_borrow::<B>().is_some());

    // Remove resource A
    assert_eq!(resources.remove::<A>(), Some(A(1)));
    assert!(!resources.is_empty());
    assert_eq!(resources.len(), 1);
    assert!(!resources.contains::<A>());
    assert!(resources.contains::<B>());
    assert!(resources.try_borrow::<A>().is_none());
    assert!(resources.try_borrow::<B>().is_some());

    // Try to remove missing resource
    assert_eq!(resources.remove::<A>(), None);
    assert!(!resources.is_empty());
    assert_eq!(resources.len(), 1);
    assert!(!resources.contains::<A>());
    assert!(resources.contains::<B>());
    assert!(resources.try_borrow::<A>().is_none());
    assert!(resources.try_borrow::<B>().is_some());

    // Remove all resources
    resources.clear();
    assert!(resources.is_empty());
    assert_eq!(resources.len(), 0);
    assert!(!resources.contains::<A>());
    assert!(!resources.contains::<B>());
    assert!(resources.try_borrow::<A>().is_none());
    assert!(resources.try_borrow::<B>().is_none());
}
