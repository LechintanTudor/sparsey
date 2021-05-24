use crate::components::{Entity, Ticks};
use crate::query::{
	ComponentView, DenseSplitComponentView, Include, IncludeExclude, IncludeExcludeFilter,
	IterData, QueryComponentFilter, QueryComponentInfoFilter, SparseSplitComponentView,
};
use crate::world::CombinedGroupInfo;

pub unsafe trait BaseQuery<'a>
where
	Self: Sized,
{
	type Item;
	type SparseSplit;
	type DenseSplit;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> CombinedGroupInfo;

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::SparseSplit);

	fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit);

	unsafe fn get_from_sparse_split(
		sparse: &mut Self::SparseSplit,
		entity: Entity,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;

	unsafe fn get_from_dense_split(
		dense: &mut Self::DenseSplit,
		index: usize,
		world_tick: Ticks,
		last_system_tick: Ticks,
	) -> Option<Self::Item>;
}

pub trait BaseQueryModifiers<'a>
where
	Self: Sized,
{
	fn include<I>(self, include: I) -> Include<Self, I>
	where
		I: QueryComponentFilter<'a>,
	{
		Include::new(self, include)
	}

	fn exclude<E>(self, exclude: E) -> IncludeExclude<Self, (), E>
	where
		E: QueryComponentFilter<'a>,
	{
		IncludeExclude::new(self, (), exclude)
	}

	fn filter<F>(self, filter: F) -> IncludeExcludeFilter<Self, (), (), F>
	where
		F: QueryComponentInfoFilter,
	{
		IncludeExcludeFilter::new(self, (), (), filter)
	}
}

impl<'a, Q> BaseQueryModifiers<'a> for Q where Q: BaseQuery<'a> {}

macro_rules! impl_base_query {
    ($(($view:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($view),+> BaseQuery<'a> for ($($view,)+)
        where
            $($view: ComponentView<'a>,)+
        {
            type Item = ($($view::Item,)+);
            type SparseSplit = ($(SparseSplitComponentView<'a, $view::Component>,)+);
            type DenseSplit = ($(DenseSplitComponentView<'a, $view::Component>,)+);

            fn get(self, entity: Entity) -> Option<Self::Item> {
                Some((
                    $(self.$idx.get(entity)?,)+
                ))
            }

            fn contains(&self, entity: Entity) -> bool {
                $(self.$idx.contains(entity))&&+
            }

            fn group_info(&self) -> CombinedGroupInfo {
                CombinedGroupInfo::Empty $(.combine(self.$idx.group_info()))+
            }

            fn split_sparse(self) -> (Option<IterData<'a>>, Self::SparseSplit) {
                split_sparse!(split_sparse, $(($view, self.$idx)),+)
            }

            fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit) {
                split_dense!(split_dense, $(($view, self.$idx)),+)
            }

            unsafe fn get_from_sparse_split(
                split: &mut Self::SparseSplit,
                entity: Entity,
                world_tick: Ticks,
                last_system_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    split.$idx.get::<$view>(entity, world_tick, last_system_tick)?,
                )+))
            }

            unsafe fn get_from_dense_split(
                split: &mut Self::DenseSplit,
                index: usize,
                world_tick: Ticks,
                last_system_tick: Ticks,
            ) -> Option<Self::Item> {
                Some(($(
                    split.$idx.get::<$view>(index, world_tick, last_system_tick)?,
                )+))
            }
        }
    };
}

impl_base_query!((A, 0));
impl_base_query!((A, 0), (B, 1));
impl_base_query!((A, 0), (B, 1), (C, 2));
impl_base_query!((A, 0), (B, 1), (C, 2), (D, 3));
