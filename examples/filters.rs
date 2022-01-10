use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Hp(u32);

#[derive(Clone, Copy, Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn apply_difficulty(mut hps: CompMut<Hp>, difficulty: Res<Difficulty>) {
    use sparsey::filters::added;

    match *difficulty {
        Difficulty::Easy => {
            println!("[Easy mode, enemies have half Hp]");

            for mut hp in added(&mut hps).iter() {
                hp.0 = hp.0 / 2;
            }
        }
        Difficulty::Medium => {
            println!("[Medium mode, enemies have full Hp]");
        }
        Difficulty::Hard => {
            println!("[Hard mode, enemies have double Hp]");

            for mut hp in added(&mut hps).iter() {
                hp.0 = hp.0 * 2;
            }
        }
    }
}

fn print_health(hps: Comp<Hp>) {
    for (e, hp) in (&hps).iter().entities() {
        println!("{:?} has {} hp", e, hp.0);
    }

    println!();
}

fn main() {
    let mut dispatcher =
        Dispatcher::builder().add_system(apply_difficulty).add_system(print_health).build();

    let mut world = World::default();
    dispatcher.register_storages(&mut world);

    for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
        world.insert_resource(difficulty);

        world.clear_entities();
        world.create_entity((Hp(10),));
        world.create_entity((Hp(100),));

        dispatcher.run_seq(&mut world).unwrap();
        world.increment_tick();
    }
}
