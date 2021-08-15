use crate::group;
use crate::query::{
	IntoQueryParts, PassthroughFilter, QueryBase, QueryModifier, SliceableQueryBase,
	StoragesNotGrouped,
};
use crate::storage::Entity;
use std::hint::unreachable_unchecked;
use std::ops::Range;

/// Trait implemented by sliceable queries.
pub trait SliceQuery<'a>
where
	Self: Sized,
{
	type ComponentSlices;

	/// Returns a slice with all entities that match the query if the component
	/// storages are grouped.
	fn try_entities(self) -> Result<&'a [Entity], StoragesNotGrouped>;

	/// Returns a slice with all components that match the query if the
	/// component storages are grouped.
	fn try_components(self) -> Result<Self::ComponentSlices, StoragesNotGrouped>;

	/// Returns a tuple containing a slice with all entities and a slice with
	/// all components that match the query if the component storages are
	/// grouped.
	fn try_entities_components(
		self,
	) -> Result<(&'a [Entity], Self::ComponentSlices), StoragesNotGrouped>;

	/// Same as `try_entities` but unwraps errors.
	fn entities(self) -> &'a [Entity] {
		self.try_entities().unwrap()
	}

	/// Same as `try_components` but unwraps errors.
	fn components(self) -> Self::ComponentSlices {
		self.try_components().unwrap()
	}

	/// Same as `try_entities_components` but unwraps errors.
	fn entities_components(self) -> (&'a [Entity], Self::ComponentSlices) {
		self.try_entities_components().unwrap()
	}
}

impl<'a, Q> SliceQuery<'a> for Q
where
	Q: IntoQueryParts<'a, Filter = PassthroughFilter>,
	Q::Base: SliceableQueryBase<'a>,
{
	type ComponentSlices = <Q::Base as SliceableQueryBase<'a>>::Slices;

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

	fn try_components(self) -> Result<Self::ComponentSlices, StoragesNotGrouped> {
		let (base, include, exclude, _) = self.into_parts();
		let range = group_range(&base, &include, &exclude)?;
		Ok(unsafe { base.slice_components(range) })
	}

	fn try_entities_components(
		self,
	) -> Result<(&'a [Entity], Self::ComponentSlices), StoragesNotGrouped> {
		let (base, include, exclude, _) = self.into_parts();
		let range = group_range(&base, &include, &exclude)?;

		unsafe {
			if !Q::Base::IS_VOID {
				Ok(base.slice_entities_and_components(range))
			} else {
				match include.into_entities() {
					Some(entities) => Ok((
						entities.get_unchecked(range.clone()),
						base.slice_components(range),
					)),
					// Unreacable because we checked earlier if the storages are grouped
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
	group::group_range(
		base.group_info(),
		include.group_info(),
		exclude.group_info(),
	)
	.ok_or(StoragesNotGrouped)
}
