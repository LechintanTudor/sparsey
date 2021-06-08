use crate::components::Entity;
use crate::query::{
	IntoQueryParts, PassthroughFilter, QueryBase, QueryGroupInfo, QueryModifier,
	StoragesNotGrouped, UnfilteredQueryBase,
};
use std::hint::unreachable_unchecked;
use std::ops::Range;

pub trait SliceQuery<'a>
where
	Self: Sized,
{
	type Slices;

	fn try_slices(self) -> Result<Self::Slices, StoragesNotGrouped>;

	fn try_entities(self) -> Result<&'a [Entity], StoragesNotGrouped>;

	fn try_entities_slices(self) -> Result<(&'a [Entity], Self::Slices), StoragesNotGrouped>;

	fn slices(self) -> Self::Slices {
		self.try_slices().unwrap()
	}

	fn entities(self) -> &'a [Entity] {
		self.try_entities().unwrap()
	}

	fn entities_slices(self) -> (&'a [Entity], Self::Slices) {
		self.try_entities_slices().unwrap()
	}
}

impl<'a, Q> SliceQuery<'a> for Q
where
	Q: IntoQueryParts<'a, Filter = PassthroughFilter>,
	Q::Base: UnfilteredQueryBase<'a>,
{
	type Slices = <Q::Base as UnfilteredQueryBase<'a>>::Slices;

	fn try_slices(self) -> Result<Self::Slices, StoragesNotGrouped> {
		let (base, include, exclude, _) = self.into_parts();
		let range = group_range(&base, &include, &exclude)?;
		Ok(unsafe { base.slice_data(range) })
	}

	fn try_entities(self) -> Result<&'a [Entity], StoragesNotGrouped> {
		let (base, include, exclude, _) = self.into_parts();
		let range = group_range(&base, &include, &exclude)?;

		unsafe {
			if !Q::Base::IS_VOID {
				Ok(base.slice_entities(range))
			} else {
				match include.into_entities() {
					Some(entities) => Ok(entities.get_unchecked(range)),
					// Returned earlier because storages aren't grouped
					None => unreachable_unchecked(),
				}
			}
		}
	}

	fn try_entities_slices(self) -> Result<(&'a [Entity], Self::Slices), StoragesNotGrouped> {
		let (base, include, exclude, _) = self.into_parts();
		let range = group_range(&base, &include, &exclude)?;

		unsafe {
			if !Q::Base::IS_VOID {
				Ok(base.slice_entities_and_data(range))
			} else {
				match include.into_entities() {
					Some(entities) => Ok((
						entities.get_unchecked(range.clone()),
						base.slice_data(range),
					)),
					// Returned earlier because storages aren't grouped
					None => unreachable_unchecked(),
				}
			}
		}
	}
}

fn group_range<'a, B, I, E>(
	base: &B,
	include: &I,
	exclude: &E,
) -> Result<Range<usize>, StoragesNotGrouped>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
{
	QueryGroupInfo::new(
		base.group_info(),
		include.group_info(),
		exclude.group_info(),
	)
	.group_range()
	.ok_or(StoragesNotGrouped)
}
