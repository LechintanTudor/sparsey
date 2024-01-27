//! Group layout example.

use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Sprite {
    #[allow(dead_code)]
    id: u32,
}

#[derive(Clone, Copy, Debug)]
struct Transparent;

fn print_sprites(
    positions: Comp<Position>,
    sprites: Comp<Sprite>,
    transparencies: Comp<Transparent>,
) {
    let (position_slice, sprite_slice) = (&positions, &sprites)
        .group_components()
        .expect("Not a group");

    println!("[All sprites]");
    println!("Positions: {:?}", position_slice);
    println!("  Sprites: {:?}", sprite_slice);

    let (position_slice, sprite_slice) = (&positions, &sprites)
        .include(&transparencies)
        .group_components()
        .expect("Not a group");

    println!("\n[Transparent sprites]");
    println!("Positions: {:?}", position_slice);
    println!("  Sprites: {:?}", sprite_slice);

    let (position_slice, sprite_slice) = (&positions, &sprites)
        .exclude(&transparencies)
        .group_components()
        .expect("Not a group");

    println!("\n[Opaque sprites]");
    println!("Positions: {:?}", position_slice);
    println!("  Sprites: {:?}", sprite_slice);
}

fn main() {
    let layout = GroupLayout::builder()
        .add_group::<(Position, Sprite)>()
        .add_group::<(Position, Sprite, Transparent)>()
        .build();

    let mut entities = EntityStorage::new(&layout);
    entities.create((Position(0, 0), Sprite { id: 0 }));
    entities.create((Position(1, 1), Sprite { id: 1 }));
    entities.create((Position(2, 2), Sprite { id: 2 }, Transparent));
    entities.create((Position(3, 3), Sprite { id: 3 }, Transparent));

    entities.run(print_sprites);
}
