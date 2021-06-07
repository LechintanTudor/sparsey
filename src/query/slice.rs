use crate::components::Entity;
use crate::query::StoragesNotGrouped;

pub trait SliceQuery<'a>
where
	Self: Sized,
{
	type Slices;

	fn try_slice(self) -> Result<Self::Slices, StoragesNotGrouped>;

	fn try_entities(self) -> Result<&'a [Entity], StoragesNotGrouped>;

	fn try_slice_entities(self) -> Result<(&'a [Entity], Self::Slices), StoragesNotGrouped>;

	fn slice(self) -> Self::Slices {
		self.try_slice().unwrap()
	}

	fn entities(self) -> &'a [Entity] {
		self.try_entities().unwrap()
	}

	fn slice_entities(self) -> (&'a [Entity], Self::Slices) {
		self.try_slice_entities().unwrap()
	}
}

// macro_rules! impl_slice_query {
//     () => {

//     };
// }
