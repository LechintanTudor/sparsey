use ecstasy::SparseSet;

fn main() {
    let mut set = SparseSet::<u32>::default();
    set.insert(0, 10);
    set.insert(1, 20);
    set.insert(2, 30);

    assert_eq!(set.remove(0), Some(10));
    assert_eq!(set.remove(0), None);

    assert_eq!(set.remove(1), Some(20));
    assert_eq!(set.remove(1), None);

    assert_eq!(set.remove(2), Some(30));
    assert_eq!(set.remove(2), None);
}
