use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Sprite {
    #[allow(dead_code)]
    id: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Transparent;

fn print_sprites(pos: Comp<Position>, sprites: Comp<Sprite>, transparencies: Comp<Transparent>) {
    let (pos_slice, sprite_slice) =
        (&pos, &sprites).components().expect("Ungrouped component storages");

    println!("[All sprites]");
    println!("Positions: {:?}", pos_slice);
    println!("Sprites: {:?}", sprite_slice);

    let (pos_slice, sprite_slice) = (&pos, &sprites)
        .include(&transparencies)
        .components()
        .expect("Ungrouped component storages");

    println!("\n[Transparent sprites]");
    println!("Positions: {:?}", pos_slice);
    println!("Sprites: {:?}", sprite_slice);

    let (pos_slice, sprite_slice) = (&pos, &sprites)
        .exclude(&transparencies)
        .components()
        .expect("Ungrouped component storages");

    println!("\n[Opaque sprites]");
    println!("Positions: {:?}", pos_slice);
    println!("Sprites: {:?}", sprite_slice);
}

fn main() {
    let mut dispatcher = Dispatcher::builder().add_system(print_sprites.system()).build();

    let layout = Layout::builder()
        .add_group(<(Position, Sprite)>::group())
        .add_group(<(Position, Sprite, Transparent)>::group())
        .build();

    let mut world = World::with_layout(&layout);
    dispatcher.register_storages(&mut world);

    world.create_entity((Position(0, 0), Sprite { id: 0 }));
    world.create_entity((Position(1, 1), Sprite { id: 1 }));
    world.create_entity((Position(2, 2), Sprite { id: 2 }, Transparent));
    world.create_entity((Position(3, 3), Sprite { id: 3 }, Transparent));

    dispatcher.run_seq(&mut world).unwrap();
}
