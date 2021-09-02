use crate::query2::{IntoQueryParts, Passthrough, QueryBase, SliceQueryElement};
use crate::storage::Entity;
use crate::{group, QueryModifier};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hint::unreachable_unchecked;
use std::ops::Range;

pub unsafe trait SliceQuery<'a>
where
	Self: IntoQueryParts<'a, Filter = Passthrough>,
	Self::Base: SliceQueryBase<'a>,
{
	/// Returns a slice with all entities that match the query if the component
	/// storages are grouped.
	fn entities(self) -> Result<&'a [Entity], UngroupedComponentStorages>;

	/// Returns a slice with all components that match the query if the
	/// component storages are grouped.
	fn components(
		self,
	) -> Result<<Self::Base as SliceQueryBase<'a>>::Slices, UngroupedComponentStorages>;

	/// Returns a tuple containing a slice with all entities and a slice with
	/// all components that match the query if the component storages are
	/// grouped.
	fn entities_components(
		self,
	) -> Result<
		(&'a [Entity], <Self::Base as SliceQueryBase<'a>>::Slices),
		UngroupedComponentStorages,
	>;
}

unsafe impl<'a, Q> SliceQuery<'a> for Q
where
	Q: IntoQueryParts<'a, Filter = Passthrough>,
	Q::Base: SliceQueryBase<'a>,
{
	fn entities(self) -> Result<&'a [Entity], UngroupedComponentStorages> {
		let (base, include, exclude, _) = self.into_query_parts();
		let range = group_range(&base, &include, &exclude)?;

		unsafe {
			if !Q::Base::IS_UNIT {
				Ok(base.slice_entities(range))
			} else {
				match include.split().0 {
					Some(data) => Ok(data.entities.get_unchecked(range)),
					// Returned earlier because storages aren't grouped
					None => unreachable_unchecked(),
				}
			}
		}
	}

	fn components(
		self,
	) -> Result<<Self::Base as SliceQueryBase<'a>>::Slices, UngroupedComponentStorages> {
		let (base, include, exclude, _) = self.into_query_parts();
		let range = group_range(&base, &include, &exclude)?;
		Ok(unsafe { base.slice_components(range) })
	}

	fn entities_components(
		self,
	) -> Result<
		(&'a [Entity], <Self::Base as SliceQueryBase<'a>>::Slices),
		UngroupedComponentStorages,
	> {
		let (base, include, exclude, _) = self.into_query_parts();
		let range = group_range(&base, &include, &exclude)?;

		unsafe {
			if !Q::Base::IS_UNIT {
				Ok(base.slice_entities_components(range))
			} else {
				match include.split().0 {
					Some(data) => Ok((
						data.entities.get_unchecked(range.clone()),
						base.slice_components(range),
					)),
					// Unreacable because we checked earlier if the storages are grouped
					None => unreachable_unchecked(),
				}
			}
		}
	}
}

/// Error returned when trying to slice a query with ungrouped component
/// storages.
#[derive(Debug)]
pub struct UngroupedComponentStorages;

impl Error for UngroupedComponentStorages {}

impl Display for UngroupedComponentStorages {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Tried to slice query with ungrouped component storages")
	}
}

fn group_range<'a, B, I, E>(
	base: &B,
	include: &I,
	exclude: &E,
) -> Result<Range<usize>, UngroupedComponentStorages>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
{
	(|| -> Option<Range<usize>> {
		group::group_range(
			base.group_info()?,
			include.group_info()?,
			exclude.group_info()?,
		)
	})()
	.ok_or(UngroupedComponentStorages)
}

pub unsafe trait SliceQueryBase<'a>
where
	Self: QueryBase<'a>,
{
	const IS_UNIT: bool;

	type Slices;

	unsafe fn slice_components(self, range: Range<usize>) -> Self::Slices;

	unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity];

	unsafe fn slice_entities_components(self, range: Range<usize>) -> (&'a [Entity], Self::Slices);
}

unsafe impl<'a> SliceQueryBase<'a> for () {
	const IS_UNIT: bool = true;

	type Slices = ();

	unsafe fn slice_components(self, _: Range<usize>) -> Self::Slices {
		()
	}

	unsafe fn slice_entities(self, _: Range<usize>) -> &'a [Entity] {
		&[]
	}

	unsafe fn slice_entities_components(self, _: Range<usize>) -> (&'a [Entity], Self::Slices) {
		(&[], ())
	}
}

macro_rules! slice_entities_components {
    ($self:ident, $range:ident, $first:tt $(, $other:tt)*) => {{
        let (entities, first_components) = $self.0.slice_entities_components($range.clone());
        (entities, (first_components, $($self.$other.slice_components($range.clone())),*))
    }};
}

macro_rules! impl_slice_query_base {
	($(($elem:ident, $idx:tt)),+) => {
        unsafe impl<'a, $($elem),+> SliceQueryBase<'a> for ($($elem,)+)
        where
            $($elem: SliceQueryElement<'a>,)+
        {
            const IS_UNIT: bool = false;

            type Slices = ($(&'a [$elem::Component],)+);

            unsafe fn slice_components(self, range: Range<usize>) -> Self::Slices {
                ($(self.$idx.slice_components(range.clone()),)+)
            }

            unsafe fn slice_entities(self, range: Range<usize>) -> &'a [Entity] {
                self.0.slice_entities(range)
            }

            unsafe fn slice_entities_components(self, range: Range<usize>) -> (&'a [Entity], Self::Slices) {
                slice_entities_components!(self, range, $($idx),+)
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
	use super::*;

	impl_slice_query_base!((A, 0));
    impl_slice_query_base!((A, 0), (B, 1));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
    impl_slice_query_base!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
}
