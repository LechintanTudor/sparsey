use crate::query2::{Passthrough, QueryElement, QueryElementFilter, SplitQueryElement};
use crate::storage::Entity;
use crate::utils::{ChangeTicks, Ticks};

pub struct Filter<F, E> {
	filter: F,
	element: E,
}

impl<F, E> Filter<F, E> {
	pub fn new(filter: F, element: E) -> Self {
		Self { filter, element }
	}
}

unsafe impl<'a, F, E> QueryElement<'a> for Filter<F, E>
where
	F: QueryElementFilter<E::Component>,
	E: QueryElement<'a, Filter = Passthrough>,
{
	type Item = E::Item;
	type Component = E::Component;
	type Filter = F;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		let (component, ticks) =
			self.element
				.get_with_ticks(entity)
				.map(|(component, ticks)| {
					(component as *const _ as *mut _, ticks as *const _ as *mut _)
				})?;

		unsafe {
			self.filter
				.matches(
					&*component,
					&*ticks,
					self.element.world_tick(),
					self.element.change_tick(),
				)
				.then(|| {
					E::get_from_parts(
						component,
						ticks,
						self.element.world_tick(),
						self.element.change_tick(),
					)
				})
		}
	}

	#[inline]
	fn get_with_ticks(&self, entity: Entity) -> Option<(&Self::Component, &ChangeTicks)> {
		self.element.get_with_ticks(entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		self.element
			.get_with_ticks(entity)
			.filter(|(component, ticks)| {
				self.filter.matches(
					component,
					ticks,
					self.element.world_tick(),
					self.element.change_tick(),
				)
			})
			.is_some()
	}

	#[inline]
	fn group_info(&self) -> crate::GroupInfo<'a> {
		self.element.group_info()
	}

	#[inline]
	fn world_tick(&self) -> crate::Ticks {
		self.element.world_tick()
	}

	#[inline]
	fn change_tick(&self) -> crate::Ticks {
		self.element.change_tick()
	}

	fn split(self) -> SplitQueryElement<'a, Self::Component, Self::Filter> {
		let split = self.element.split();
		SplitQueryElement::new(
			split.sparse,
			split.entities,
			split.components,
			split.ticks,
			self.filter,
		)
	}

	#[inline]
	unsafe fn get_from_parts(
		component: *mut Self::Component,
		ticks: *mut ChangeTicks,
		world_tick: Ticks,
		change_tick: Ticks,
	) -> Self::Item {
		E::get_from_parts(component, ticks, world_tick, change_tick)
	}
}
