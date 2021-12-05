use crate::prelude::Entity;
use crate::query::{
    get_group_range, is_trivial_group, InvalidGroup, Passthrough, Query, SliceQueryGet,
};

pub unsafe trait SliceQuery<'a>: Query<'a, Filter = Passthrough>
where
    Self::Get: SliceQueryGet<'a>,
{
    fn entities(self) -> Result<&'a [Entity], InvalidGroup>;

    fn components(self) -> Result<<Self::Get as SliceQueryGet<'a>>::Slices, InvalidGroup>;

    fn entities_components(
        self,
    ) -> Result<(&'a [Entity], <Self::Get as SliceQueryGet<'a>>::Slices), InvalidGroup>;
}

unsafe impl<'a, Q> SliceQuery<'a> for Q
where
    Q: Query<'a, Filter = Passthrough>,
    Q::Get: SliceQueryGet<'a>,
{
    fn entities(self) -> Result<&'a [Entity], InvalidGroup> {
        if is_trivial_group::<Q::Get, Q::Include, Q::Exclude>() {
            let (get, _, _, _) = self.into_query_parts();
            unsafe { Ok(get.get_entities_unchecked(..)) }
        } else {
            let (get, include, exclude, _) = self.into_query_parts();
            let range = get_group_range(&get, &include, &exclude)?;
            unsafe { Ok(get.get_entities_unchecked(range)) }
        }
    }

    fn components(self) -> Result<<Self::Get as SliceQueryGet<'a>>::Slices, InvalidGroup> {
        if is_trivial_group::<Q::Get, Q::Include, Q::Exclude>() {
            let (get, _, _, _) = self.into_query_parts();
            unsafe { Ok(get.get_components_unchecked(..)) }
        } else {
            let (get, include, exclude, _) = self.into_query_parts();
            let range = get_group_range(&get, &include, &exclude)?;
            unsafe { Ok(get.get_components_unchecked(range)) }
        }
    }

    fn entities_components(
        self,
    ) -> Result<(&'a [Entity], <Self::Get as SliceQueryGet<'a>>::Slices), InvalidGroup> {
        if is_trivial_group::<Q::Get, Q::Include, Q::Exclude>() {
            let (get, _, _, _) = self.into_query_parts();
            unsafe { Ok(get.get_entities_components_unchecked(..)) }
        } else {
            let (get, include, exclude, _) = self.into_query_parts();
            let range = get_group_range(&get, &include, &exclude)?;
            unsafe { Ok(get.get_entities_components_unchecked(range)) }
        }
    }
}
