use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Lava {
	height: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Position(i32, i32);

fn raise_lava(mut lava: ResMut<Lava>) {
	lava.height += 2;
	println!("[Lava raised to y={}]", lava.height);
}

fn fall_in_lava(mut commands: Commands, pos: Comp<Position>, lava: Res<Lava>) {
	for (e, (pos,)) in (&pos,).iter().entities() {
		if pos.1 < lava.height {
			println!("{:?} with y={} fell into lava", e, pos.1);
			commands.destroy(e);
		}
	}

	println!();
}

fn main() {
	let mut dispatcher = Dispatcher::builder()
		.add_system(raise_lava.system())
		.add_system(fall_in_lava.system())
		.build();

	let mut world = World::default();
	dispatcher.set_up(&mut world);

	world.create((Position(0, 1),));
	world.create((Position(0, 2),));
	world.create((Position(0, 3),));
	world.create((Position(0, 4),));
	world.create((Position(0, 5),));
	world.create((Position(0, 6),));

	let mut resources = Resources::default();
	resources.insert(Lava { height: 1 });

	for _ in 0..3 {
		dispatcher.run_seq(&mut world, &mut resources).unwrap();
		world.advance_ticks().unwrap();
	}
}
