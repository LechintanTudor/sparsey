use ecstasy::*;

struct A;
struct B;
struct C;
struct D;
struct E;

fn main() {
    let mut groups = vec![
        Group::builder()
            .with::<A>()
            .with::<B>()
            .with::<C>()
            .with::<D>()
            .with::<E>()
            .build(),

        Group::builder()
            .with::<B>()
            .with::<C>()
            .with::<D>()
            .with::<E>()
            .build(),

        Group::builder()
            .with::<C>()
            .with::<E>()
            .build(),
    ];

    groups.sort_by(|g1, g2| g1.partial_cmp(g2).expect("Incompatible groups"));
    println!("{:#?}", groups);

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
