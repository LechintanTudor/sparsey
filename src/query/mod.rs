// mod filter;

// use crate::components::Entity;
// use crate::world::{Comp, CompMut, GroupInfo};
// use filter::GroupFilter;

// pub unsafe trait Query
// where
// 	Self: Sized,
// {
// 	type Item;
// }

// pub unsafe trait SimpleQuery
// where
// 	Self: Query,
// {
// 	fn include<F>(self, filter: F) -> Include<Self, F>
// 	where
// 		F: GroupFilter,
// 	{
// 		Include::new(self, filter)
// 	}

// 	fn exclude<F>(self, filter: F) -> Exclude<Include<Self, ()>, F>
// 	where
// 		F: GroupFilter,
// 	{
// 		Exclude::new(Include::new(self, ()), filter)
// 	}
// }

// pub struct Include<Q, F>
// where
// 	Q: Query,
// 	F: GroupFilter,
// {
// 	query: Q,
// 	filter: F,
// }

// impl<Q, F> Include<Q, F>
// where
// 	Q: Query,
// 	F: GroupFilter,
// {
// 	fn new(query: Q, filter: F) -> Self {
// 		Self { query, filter }
// 	}

// 	pub fn exclude<E>(self, filter: E) -> Exclude<Self, E>
// 	where
// 		E: GroupFilter,
// 	{
// 		Exclude::new(self, filter)
// 	}
// }

// unsafe impl<Q, F> Query for Include<Q, F>
// where
// 	Q: Query,
// 	F: GroupFilter,
// {
// 	type Item = Q::Item;
// }

// pub struct Exclude<Q, F>
// where
// 	Q: Query,
// 	F: GroupFilter,
// {
// 	query: Q,
// 	filter: F,
// }

// impl<Q, F> Exclude<Q, F>
// where
// 	Q: Query,
// 	F: GroupFilter,
// {
// 	fn new(query: Q, filter: F) -> Self {
// 		Self { query, filter }
// 	}
// }

// unsafe impl<Q, F> Query for Exclude<Q, F>
// where
// 	Q: Query,
// 	F: GroupFilter,
// {
// 	type Item = Q::Item;
// }

// pub unsafe trait QueryComponent<'a> {
// 	type Item: 'a;

// 	fn get(&self, entity: Entity) -> Option<Self::Item>;

// 	fn group_info(&self) -> Option<GroupInfo>;
// }
