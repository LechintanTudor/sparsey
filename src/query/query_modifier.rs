use crate::components::QueryGroupInfo;
use crate::query::{GetComponent, GetImmutableComponent, Passthrough};
use crate::storage::{Entity, EntitySparseArray};

pub unsafe trait QueryModifier<'a> {
    type Sparse: 'a;

    fn includes(&self, entity: Entity) -> bool;

    fn excludes(&self, entity: Entity) -> bool;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn split(self) -> (Option<&'a [Entity]>, Self::Sparse);

    fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool;

    fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool;
}

unsafe impl<'a> QueryModifier<'a> for Passthrough {
    type Sparse = ();

    #[inline(always)]
    fn includes(&self, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes(&self, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        Some(info)
    }

    #[inline(always)]
    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        Some(info)
    }

    #[inline(always)]
    fn split(self) -> (Option<&'a [Entity]>, Self::Sparse) {
        (None, ())
    }

    #[inline(always)]
    fn includes_sparse(_sparse: &Self::Sparse, _entity: Entity) -> bool {
        true
    }

    #[inline(always)]
    fn excludes_sparse(_sparse: &Self::Sparse, _entity: Entity) -> bool {
        true
    }
}

unsafe impl<'a, G> QueryModifier<'a> for G
where
    G: GetImmutableComponent<'a>,
{
    type Sparse = &'a EntitySparseArray;

    fn includes(&self, entity: Entity) -> bool {
        GetComponent::get_index(self, entity).is_some()
    }

    fn excludes(&self, entity: Entity) -> bool {
        GetComponent::get_index(self, entity).is_none()
    }

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        info.include(GetComponent::group_info(self)?)
    }

    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        info.exclude(GetComponent::group_info(self)?)
    }

    fn split(self) -> (Option<&'a [Entity]>, Self::Sparse) {
        let (entities, sparse, _) = GetComponent::split(self);
        (Some(entities), sparse)
    }

    fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
        sparse.contains_entity(entity)
    }

    fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
        !sparse.contains_entity(entity)
    }
}
