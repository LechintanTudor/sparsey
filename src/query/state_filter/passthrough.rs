use crate::components::Entity;
use crate::query::StateFilter;
use std::marker::PhantomData;

#[derive(Default, Debug)]
pub struct Passthrough(PhantomData<()>);

impl Passthrough {
	pub const fn new() -> Self {
		Self(PhantomData)
	}
}

impl StateFilter for Passthrough {
	fn matches(&self, _entity: Entity) -> bool {
		true
	}
}
