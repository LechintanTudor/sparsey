use crate::components::QueryGroupInfo;
use crate::query::{GetComponent, GetImmutableComponent};
use crate::storage::{Entity, EntitySparseArray};

pub unsafe trait QueryModifier<'a> {
    type Sparse: 'a;

    fn includes(&self, entity: Entity) -> bool;

    fn excludes(&self, entity: Entity) -> bool;

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>>;

    fn split(self) -> (&'a [Entity], Self::Sparse);

    fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool;

    fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool;
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
        GetComponent::include_group_info(self, info)
    }

    fn exclude_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        GetImmutableComponent::exclude_group_info(self, info)
    }

    fn split(self) -> (&'a [Entity], Self::Sparse) {
        let (entities, sparse, _, _) = GetComponent::split(self);
        (entities, sparse)
    }

    fn includes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
        sparse.contains_entity(entity)
    }

    fn excludes_sparse(sparse: &Self::Sparse, entity: Entity) -> bool {
        !sparse.contains_entity(entity)
    }
}
