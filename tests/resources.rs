mod common;

use common::*;
use sparsey::prelude::*;

#[test]
fn test_crud() {
    let mut resources = Resources::default();

    // Insert
    assert!(!resources.contains::<A>());
    assert_eq!(resources.insert(A(0)), None);
    assert_eq!(resources.insert(A(1)), Some(A(0)));
    assert!(resources.contains::<A>());

    // Borrow
    assert_eq!(*resources.borrow::<A>(), A(1));

    // Remove
    assert_eq!(resources.remove::<A>(), Some(A(1)));
    assert_eq!(resources.remove::<A>(), None);
    assert_eq!(resources.remove::<B>(), None);
    assert!(!resources.contains::<A>());
    assert!(!resources.contains::<B>());

    // Clear
    resources.insert(A(0));
    resources.insert(B(0));
    resources.clear();
    assert!(!resources.contains::<A>());
    assert!(!resources.contains::<B>());
}
