use crate::prelude::Entity;
use crate::query::{
    get_group_range, is_trivial_group, InvalidGroup, Passthrough, Query, SliceQueryGet,
};

/// Trait used for getting `Component` and `Entity` slices from grouped components.
pub unsafe trait SliceQuery<'a>: Query<'a, Filter = Passthrough>
where
    Self::Get: SliceQueryGet<'a>,
{
    /// Returns all entities that match the `Query`, if the components are grouped.
    fn entities(self) -> Result<&'a [Entity], InvalidGroup>;

    /// Returns all components that match the `Query`, if the components are grouped.
    fn components(self) -> Result<<Self::Get as SliceQueryGet<'a>>::Slices, InvalidGroup>;

    /// Returns all entities and components that match the `Query`, if the components are grouped.
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
