pub use self::components::*;
pub use self::layout::*;
pub use self::query::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::world::*;

mod components;
mod layout;
mod query;
mod resources;
mod systems;
mod world;

mod utils {
	use std::any;

	#[cold]
	#[inline(never)]
	pub fn panic_missing_comp<T>() -> ! {
		panic!(
			"Tried to access missing component storage `{}`",
			any::type_name::<T>()
		)
	}

	#[cold]
	#[inline(never)]
	pub fn panic_missing_res<T>() -> ! {
		panic!(
			"Tried to access missing resource `{}`",
			any::type_name::<T>()
		)
	}
}
