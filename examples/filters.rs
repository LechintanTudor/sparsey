use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Hp(u32);

#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
	Easy,
	Medium,
	Hard,
}

fn apply_difficulty(mut hps: CompMut<Hp>, difficulty: Res<Difficulty>) {
	use sparsey::filters::added;

	match *difficulty {
		Difficulty::Easy => {
			println!("[Easy mode, enemies have half Hp]");

			for (mut hp,) in (added(&mut hps),).iter() {
				hp.0 = hp.0 / 2;
			}
		}
		Difficulty::Medium => {
			println!("[Medium mode, enemies have full Hp]");
		}
		Difficulty::Hard => {
			println!("[Hard mode, enemies have double Hp]");

			for (mut hp,) in (added(&mut hps),).iter() {
				hp.0 = hp.0 * 2;
			}
		}
	}
}

fn print_health(hps: Comp<Hp>) {
	for (e, (hp,)) in (&hps,).iter().entities() {
		println!("{:?} has {} hp", e, hp.0);
	}

	println!();
}

fn main() {
	let mut dispatcher = Dispatcher::builder()
		.add_system(apply_difficulty.system())
		.add_system(print_health.system())
		.build();

	let mut world = World::default();
	dispatcher.set_up(&mut world);

	let mut resources = Resources::default();

	for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
		resources.insert(difficulty);

		world.clear();
		world.create((Hp(10),));
		world.create((Hp(100),));

		dispatcher.run_seq(&mut world, &mut resources).unwrap();
		world.advance_ticks().unwrap();
	}
}
