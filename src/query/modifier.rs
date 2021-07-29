use crate::components::{Entity, SparseArrayView};
use crate::group::CombinedGroupInfo;
use crate::query::{ComponentView, ImmutableUnfilteredComponentView, IterData};

/// Trait implemented by include and exclude query modifiers.
pub trait QueryModifier<'a> {
	type Split;

	fn includes(&self, entity: Entity) -> bool;

	fn excludes(&self, entity: Entity) -> bool;

	fn group_info(&self) -> CombinedGroupInfo<'a>;

	fn into_entities(self) -> Option<&'a [Entity]>;

	fn into_iter_data(self) -> Option<IterData<'a>>;

	fn split(self) -> (Option<IterData<'a>>, Self::Split);

	fn split_includes(split: &Self::Split, entity: Entity) -> bool;

	fn split_excludes(split: &Self::Split, entity: Entity) -> bool;
}

impl<'a, C> QueryModifier<'a> for C
where
	C: ImmutableUnfilteredComponentView<'a>,
{
	type Split = SparseArrayView<'a>;

	fn includes(&self, entity: Entity) -> bool {
		<C as ComponentView<'a>>::contains(self, entity)
	}

	fn excludes(&self, entity: Entity) -> bool {
		!<C as ComponentView<'a>>::contains(self, entity)
	}

	fn group_info(&self) -> CombinedGroupInfo<'a> {
		let group_info = <C as ComponentView<'a>>::group_info(self);
		CombinedGroupInfo::from_group_info(group_info)
	}

	fn into_entities(self) -> Option<&'a [Entity]> {
		Some(C::into_parts(self).1)
	}

	fn into_iter_data(self) -> Option<IterData<'a>> {
		let world_tick = self.world_tick();
		let last_system_tick = self.last_system_tick();
		let entities = C::into_parts(self).1;

		Some(IterData::new(entities, world_tick, last_system_tick))
	}

	fn split(self) -> (Option<IterData<'a>>, Self::Split) {
		let world_tick = self.world_tick();
		let last_system_tick = self.last_system_tick();
		let (sparse, entities, _, _) = self.into_parts();
		let iter_data = IterData::new(entities, world_tick, last_system_tick);

		(Some(iter_data), sparse)
	}

	fn split_includes(split: &Self::Split, entity: Entity) -> bool {
		split.contains(entity)
	}

	fn split_excludes(split: &Self::Split, entity: Entity) -> bool {
		!split.contains(entity)
	}
}

impl<'a> QueryModifier<'a> for () {
	type Split = ();

	fn includes(&self, _: Entity) -> bool {
		true
	}

	fn excludes(&self, _: Entity) -> bool {
		true
	}

	fn group_info(&self) -> CombinedGroupInfo<'a> {
		CombinedGroupInfo::Empty
	}

	fn into_entities(self) -> Option<&'a [Entity]> {
		None
	}

	fn into_iter_data(self) -> Option<IterData<'a>> {
		None
	}

	fn split(self) -> (Option<IterData<'a>>, Self::Split) {
		(None, ())
	}

	fn split_includes(_: &Self::Split, _: Entity) -> bool {
		true
	}

	fn split_excludes(_: &Self::Split, _: Entity) -> bool {
		true
	}
}

macro_rules! impl_query_modifier {
	($(($view:ident, $idx:tt)),+) => {
		impl<'a, $($view),+> QueryModifier<'a> for ($($view,)+)
		where
			$($view: ImmutableUnfilteredComponentView<'a>,)+
		{
			type Split = ($(to_sparse_array_view!($view),)+);

			fn includes(&self, entity: Entity) -> bool {
				$(self.$idx.includes(entity))&&+
			}

			fn excludes(&self, entity: Entity) -> bool {
				$(!self.$idx.includes(entity))&&+
			}

			fn group_info(&self) -> CombinedGroupInfo<'a> {
				CombinedGroupInfo::Empty $(.combine(self.$idx.group_info()))+
			}

			fn into_entities(self) -> Option<&'a [Entity]> {
				Some(self.0.into_parts().1)
			}

			fn into_iter_data(self) -> Option<IterData<'a>> {
				let view = self.0;
				let world_tick = view.world_tick();
				let last_system_tick = view.last_system_tick();
				let entities = view.into_parts().1;

				Some(IterData::new(entities, world_tick, last_system_tick))
			}

			fn split(self) -> (Option<IterData<'a>>, Self::Split) {
				split_modifier!($(($view, self.$idx)),+)
			}

			fn split_includes(split: &Self::Split, entity: Entity) -> bool {
				$($view::split_includes(&split.$idx, entity))&&+
			}

			fn split_excludes(split: &Self::Split, entity: Entity) -> bool {
				$(!$view::split_includes(&split.$idx, entity))&&+
			}
		}
	};
}

macro_rules! to_sparse_array_view {
	($ident:ident) => {
		SparseArrayView<'a>
	};
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_query_modifier!((A, 0));
    impl_query_modifier!((A, 0), (B, 1));
    impl_query_modifier!((A, 0), (B, 1), (C, 2));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_query_modifier!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
