use crate::components::Entity;
use crate::query::StateFilter;

#[derive(Default, Debug)]
pub struct Passthrough;

impl StateFilter for Passthrough {
	fn matches(&self, _entity: Entity) -> bool {
		true
	}
}
