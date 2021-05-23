#[macro_use]
mod split;

mod base;
mod component_filter;
mod component_info_filter;
mod component_view;
mod iter;
mod modifiers;

pub use self::base::BaseQueryModifiers;
pub use self::component_filter::*;
pub use self::component_info_filter::*;
pub use self::component_view::*;
pub use self::iter::*;
pub use self::modifiers::*;

use crate::components::{Entity, Ticks};
use crate::query::base::BaseQuery;
use crate::world::{CombinedGroupInfo, QueryGroupInfo};

pub unsafe trait Query<'a> {
	type Item;
	type Iterator: Iterator<Item = Self::Item>;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn iter(self) -> Self::Iterator;
}

unsafe impl<'a, Q> Query<'a> for Q
where
	Q: BaseQuery<'a>,
{
	type Item = <Q as BaseQuery<'a>>::Item;
	type Iterator = Iter<'a, Q, (), (), PassthroughFilter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		<Q as BaseQuery<'a>>::get(self, entity)
	}

	fn contains(&self, entity: Entity) -> bool {
		<Q as BaseQuery<'a>>::contains(self, entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self, (), (), PassthroughFilter)
	}
}

unsafe impl<'a, Q, I> Query<'a> for Include<Q, I>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
{
	type Item = <Q as BaseQuery<'a>>::Item;
	type Iterator = Iter<'a, Q, I, (), PassthroughFilter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.include.includes_all(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.include.includes_all(entity) && self.query.contains(entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self.query, self.include, (), PassthroughFilter)
	}
}

unsafe impl<'a, Q, I, E> Query<'a> for IncludeExclude<Q, I, E>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
{
	type Item = <Q as BaseQuery<'a>>::Item;
	type Iterator = Iter<'a, Q, I, E, PassthroughFilter>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		if self.exclude.excludes_all(entity) && self.include.includes_all(entity) {
			self.query.get(entity)
		} else {
			None
		}
	}

	fn contains(&self, entity: Entity) -> bool {
		self.exclude.excludes_all(entity)
			&& self.include.includes_all(entity)
			&& self.query.contains(entity)
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self.query, self.include, self.exclude, PassthroughFilter)
	}
}

unsafe impl<'a, Q, I, E, F> Query<'a> for IncludeExcludeFilter<Q, I, E, F>
where
	Q: BaseQuery<'a>,
	I: BaseComponentFilter<'a>,
	E: BaseComponentFilter<'a>,
	F: QueryComponentInfoFilter,
{
	type Item = <Q as BaseQuery<'a>>::Item;
	type Iterator = Iter<'a, Q, I, E, F>;

	fn get(self, entity: Entity) -> Option<Self::Item> {
		todo!()
	}

	fn contains(&self, entity: Entity) -> bool {
		todo!()
	}

	fn iter(self) -> Self::Iterator {
		Iter::new(self.query, self.include, self.exclude, self.filter)
	}
}
