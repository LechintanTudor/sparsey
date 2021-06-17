use crate::components::{Entity, Ticks};
use crate::query::{
	ComponentView, DenseSplitComponentView, ImmutableUnfilteredComponentView, Include,
	IncludeExclude, IncludeExcludeFilter, IntoQueryParts, IterData, Passthrough, QueryFilter,
	QueryModifier, SparseSplitComponentView,
};
use crate::world::CombinedGroupInfo;
use std::ops::Range;

pub unsafe trait QueryBase<'a>
where
	Self: Sized,
{
	const IS_VOID: bool;

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

impl<'a, Q> IntoQueryParts<'a> for Q
where
	Q: QueryBase<'a>,
{
	type Base = Self;
	type Include = ();
	type Exclude = ();
	type Filter = Passthrough;

	fn into_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter) {
		(self, (), (), Passthrough::default())
	}
}

pub unsafe trait SliceableQueryBase<'a>
where
	Self: QueryBase<'a>,
{
	type Slices;

	unsafe fn slice_data(self, range: Range<usize>) -> Self::Slices;

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity];

	unsafe fn slice_entities_and_data(self, range: Range<usize>) -> (&'a [Entity], Self::Slices);
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

unsafe impl<'a> QueryBase<'a> for () {
	const IS_VOID: bool = true;

	type Item = ();
	type SparseSplit = ();
	type DenseSplit = ();

	fn get(self, _: Entity) -> Option<Self::Item> {
		Some(())
	}

	fn contains(&self, _: Entity) -> bool {
		true
	}

	fn group_info(&self) -> CombinedGroupInfo<'a> {
		CombinedGroupInfo::Empty
	}

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::SparseSplit) {
		(None, ())
	}

	fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit) {
		(None, ())
	}

	unsafe fn get_from_sparse_split(
		_: &mut Self::SparseSplit,
		_: Entity,
		_: Ticks,
		_: Ticks,
	) -> Option<Self::Item> {
		Some(())
	}

	unsafe fn get_from_dense_split(
		_: &mut Self::DenseSplit,
		_: usize,
		_: Ticks,
		_: Ticks,
	) -> Option<Self::Item> {
		Some(())
	}
}

unsafe impl<'a> SliceableQueryBase<'a> for () {
	type Slices = ();

	unsafe fn slice_data(self, _: Range<usize>) -> Self::Slices {
		()
	}

	unsafe fn slice_entities(self, _: Range<usize>) -> &'a [Entity] {
		&[]
	}

	unsafe fn slice_entities_and_data(self, _: Range<usize>) -> (&'a [Entity], Self::Slices) {
		(&[], ())
	}
}

macro_rules! impl_base_query {
    ($(($view:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($view),+> QueryBase<'a> for ($($view,)+)
        where
            $($view: ComponentView<'a>,)+
        {
            const IS_VOID: bool = false;

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
                split_sparse!($(($view, self.$idx)),+)
            }

            fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit) {
                split_dense!($(($view, self.$idx)),+)
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

        unsafe impl<'a, $($view),+> SliceableQueryBase<'a> for ($($view,)+)
        where
            $($view: ImmutableUnfilteredComponentView<'a>,)+
        {
            type Slices = ($(&'a [$view::Component],)+);

            unsafe fn slice_data(self, range: Range<usize>) -> Self::Slices {
                ($(self.$idx.slice_data(range.clone()),)+)
            }

            unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity] {
                self.0.slice_entities(range)
            }

            unsafe fn slice_entities_and_data(self, range: Range<usize>) -> (&'a [Entity], Self::Slices) {
                slice_entities_and_data!(self, range, $($idx),+)
            }
        }
    };
}

macro_rules! slice_entities_and_data {
    ($self:ident, $range:ident, $first:tt $(, $other:tt)*) => {{
        let (entities, first_data) = $self.0.slice_entities_and_data($range.clone());
        (entities, (first_data, $($self.$other.slice_data($range.clone())),*))
    }};
}

impl_base_query!((A, 0));
impl_base_query!((A, 0), (B, 1));
impl_base_query!((A, 0), (B, 1), (C, 2));
impl_base_query!((A, 0), (B, 1), (C, 2), (D, 3));
