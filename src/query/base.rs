use crate::components::{Entity, Ticks};
use crate::query::{ComponentView, DenseSplitComponentView, SparseSplitComponentView};
use crate::world::CombinedGroupInfo;

#[derive(Debug)]
pub struct IterData<'a> {
	pub entities: &'a [Entity],
	pub world_tick: Ticks,
	pub last_system_tick: Ticks,
}

impl<'a> IterData<'a> {
	pub fn new(entities: &'a [Entity], world_tick: Ticks, last_system_tick: Ticks) -> Self {
		Self {
			entities,
			world_tick,
			last_system_tick,
		}
	}
}

pub unsafe trait BaseQuery<'a> {
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
                CombinedGroupInfo::new() $(.combine(self.$idx.group_info()))+
            }

            fn split_sparse(self) -> (Option<IterData<'a>>, Self::SparseSplit) {
                todo!()
            }

            fn split_dense(self) -> (Option<IterData<'a>>, Self::DenseSplit) {
                todo!()
            }

            unsafe fn get_from_sparse_split(
                sparse: &mut Self::SparseSplit,
                entity: Entity,
                world_tick: Ticks,
                last_system_tick: Ticks,
            ) -> Option<Self::Item> {
                todo!()
            }

            unsafe fn get_from_dense_split(
                dense: &mut Self::DenseSplit,
                index: usize,
                world_tick: Ticks,
                last_system_tick: Ticks,
            ) -> Option<Self::Item> {
                todo!()
            }
        }
    };
}

impl_base_query!((A, 0));
impl_base_query!((A, 0), (B, 1));
impl_base_query!((A, 0), (B, 1), (C, 2));
impl_base_query!((A, 0), (B, 1), (C, 2), (D, 3));
