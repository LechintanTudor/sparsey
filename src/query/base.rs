use crate::components::{Entity, Ticks};
use crate::query::{
	ComponentView, DenseSplitComponentView, Include, IncludeExclude, IncludeExcludeFilter,
	IterData, QueryFilter, QueryModifier, SparseSplitComponentView, StoragesNotGrouped,
	UnfilteredComponentView,
};
use crate::world::CombinedGroupInfo;

pub unsafe trait QueryBase<'a>
where
	Self: Sized,
{
	type Item;
	type SparseSplit;
	type DenseSplit;

	fn get(self, entity: Entity) -> Option<Self::Item>;

	fn contains(&self, entity: Entity) -> bool;

	fn group_info(&self) -> CombinedGroupInfo<'a>;

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

pub unsafe trait UnfilteredQueryBase<'a>
where
	Self: QueryBase<'a>,
{
	type Slices;

	fn try_slice(self) -> Result<Self::Slices, StoragesNotGrouped> {
		todo!()
	}

	fn try_entities(self) -> Result<&'a [Entity], StoragesNotGrouped> {
		todo!()
	}

	fn try_slice_entities(self) -> Result<(&'a [Entity], Self::Slices), StoragesNotGrouped> {
		todo!()
	}
}

pub trait QueryBaseModifiers<'a>
where
	Self: Sized,
{
	fn include<I>(self, include: I) -> Include<Self, I>
	where
		I: QueryModifier<'a>,
	{
		Include::new(self, include)
	}

	fn exclude<E>(self, exclude: E) -> IncludeExclude<Self, (), E>
	where
		E: QueryModifier<'a>,
	{
		IncludeExclude::new(self, (), exclude)
	}

	fn filter<F>(self, filter: F) -> IncludeExcludeFilter<Self, (), (), F>
	where
		F: QueryFilter,
	{
		IncludeExcludeFilter::new(self, (), (), filter)
	}
}

impl<'a, Q> QueryBaseModifiers<'a> for Q where Q: QueryBase<'a> {}

macro_rules! impl_base_query {
    ($(($view:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($view),+> QueryBase<'a> for ($($view,)+)
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

            fn group_info(&self) -> CombinedGroupInfo<'a> {
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

        unsafe impl<'a, $($view),+> UnfilteredQueryBase<'a> for ($($view,)+)
        where
            $($view: UnfilteredComponentView<'a>,)+
        {
            type Slices = ($(&'a [$view::Component],)+);

            fn try_slice(self) -> Result<Self::Slices, StoragesNotGrouped> {
                todo!()
            }
        }
    };
}

impl_base_query!((A, 0));
impl_base_query!((A, 0), (B, 1));
impl_base_query!((A, 0), (B, 1), (C, 2));
impl_base_query!((A, 0), (B, 1), (C, 2), (D, 3));
