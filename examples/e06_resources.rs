use sparsey::{Dispatcher, IntoSystem, Res, ResMut, Resources, World};

// TODO: Better error messages on `Environment`.

#[derive(Copy, Clone, Debug)]
struct Time(u32);

/// `ResMut<T>` gives us an exclusive view over a resource of type `T`.
fn update_time(mut time: ResMut<Time>) {
    time.0 += 1;
}

/// `Res<T>` gives us a shared view over a resource of type `T`.
fn print_time(time: Res<Time>) {
    println!("{:?}", *time);
}

fn main() {
    let mut world = World::default();

    let mut resources = Resources::default();
    resources.insert(Time(0));

    let mut dispatcher = Dispatcher::builder()
        .add_system(update_time.system())
        .add_system(print_time.system())
        .build();

    dispatcher.run_locally(&mut world, &mut resources).unwrap();
}
