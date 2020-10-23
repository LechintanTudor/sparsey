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
        Group::builder().with::<C>().with::<E>().build(),
    ];

    groups.sort_by(|g1, g2| g1.partial_cmp(g2).expect("Incompatible groups"));
    println!("{:#?}", groups);
}
