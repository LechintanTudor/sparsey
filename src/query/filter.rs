use crate::components::QueryGroupInfo;
use crate::query::{
    Added, And, ChangeTicksFilter, Changed, GetComponentSet, GetComponentSetUnfiltered, Mutated,
    Not, Or, Passthrough, QueryFilter, QueryGet, Xor,
};
use crate::storage::{Entity, Ticks};
use std::marker::PhantomData;

/// Type that a filter to the contained components.
pub struct Filter<F, G> {
    get: G,
    _phantom: PhantomData<F>,
}

impl<'a, F, G> Filter<F, G>
where
    F: ChangeTicksFilter,
{
    /// Applies a filter to the given components.
    pub fn new(get: G) -> Self {
        Self { get, _phantom: PhantomData }
    }
}

unsafe impl<'a, F, G> GetComponentSet<'a> for Filter<F, G>
where
    F: ChangeTicksFilter,
    G: GetComponentSetUnfiltered<'a>,
{
    const GETS_ONE: bool = G::GETS_ONE;

    type Item = G::Item;
    type Filter = F;
    type Index = G::Index;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        self.get.group_info()
    }

    fn include_group_info(&self, info: QueryGroupInfo<'a>) -> Option<QueryGroupInfo<'a>> {
        self.get.include_group_info(info)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        self.get.change_detection_ticks()
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        self.get.get_index(entity)
    }

    unsafe fn matches_unchecked(&self, index: Self::Index) -> bool {
        self.get.matches_unchecked::<F>(index)
    }

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item> {
        self.get.get_unchecked::<F>(index)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        self.get.split_sparse()
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        self.get.split_dense()
    }

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index> {
        G::get_index_from_sparse(sparse, entity)
    }

    unsafe fn get_from_sparse_unchecked(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_sparse_unchecked::<F>(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked::<F>(data, index, world_tick, change_tick)
    }
}

unsafe impl<'a, F, G> QueryGet<'a> for Filter<F, G>
where
    F: ChangeTicksFilter,
    G: GetComponentSetUnfiltered<'a>,
{
    const GETS_ONE: bool = G::GETS_ONE;

    type Item = G::Item;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        self.get.group_info()
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        self.get.change_detection_ticks()
    }

    fn contains(&self, entity: Entity) -> bool {
        let index = match self.get.get_index(entity) {
            Some(index) => index,
            None => return false,
        };

        unsafe { self.get.matches_unchecked::<F>(index) }
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let index = self.get.get_index(entity)?;
        unsafe { self.get.get_unchecked::<F>(index) }
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        self.get.split_sparse()
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        self.get.split_dense()
    }

    unsafe fn get_from_sparse_unchecked(
        sparse: &Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = G::get_index_from_sparse(sparse, entity)?;
        G::get_from_sparse_unchecked::<F>(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked::<F>(data, index, world_tick, change_tick)
    }
}

impl<'a, F, G> QueryFilter for Filter<F, G>
where
    F: ChangeTicksFilter,
    G: GetComponentSetUnfiltered<'a>,
{
    fn matches(&self, entity: Entity) -> bool {
        let index = match self.get.get_index(entity) {
            Some(index) => index,
            None => return false,
        };

        unsafe { self.get.matches_unchecked::<F>(index) }
    }
}

/// Applies a filter that matches all components.
pub fn contains<'a, G>(get: G) -> Filter<Passthrough, G>
where
    G: GetComponentSetUnfiltered<'a>,
{
    Filter::new(get)
}

/// Applies a filter that matches if any component was added.
pub fn added<'a, G>(get: G) -> Filter<Added, G>
where
    G: GetComponentSetUnfiltered<'a>,
{
    Filter::new(get)
}

/// Applies a filter that matches if any component was mutated.
pub fn mutated<'a, G>(get: G) -> Filter<Mutated, G>
where
    G: GetComponentSetUnfiltered<'a>,
{
    Filter::new(get)
}

/// Applies a filter that matches if any component was added or mutated.
pub fn changed<'a, G>(get: G) -> Filter<Changed, G>
where
    G: GetComponentSetUnfiltered<'a>,
{
    Filter::new(get)
}

impl<'a, F, E> std::ops::Not for Filter<F, E>
where
    F: ChangeTicksFilter,
{
    type Output = Filter<Not<F>, E>;

    fn not(self) -> Self::Output {
        Filter::new(self.get)
    }
}

impl<'a, F1, E, F2> std::ops::BitAnd<F2> for Filter<F1, E>
where
    F1: ChangeTicksFilter,
    F2: QueryFilter,
{
    type Output = And<Self, F2>;

    fn bitand(self, filter: F2) -> Self::Output {
        And(self, filter)
    }
}

impl<'a, F1, E, F2> std::ops::BitOr<F2> for Filter<F1, E>
where
    F1: ChangeTicksFilter,
    F2: QueryFilter,
{
    type Output = Or<Self, F2>;

    fn bitor(self, filter: F2) -> Self::Output {
        Or(self, filter)
    }
}

impl<'a, F1, E, F2> std::ops::BitXor<F2> for Filter<F1, E>
where
    F1: ChangeTicksFilter,
    F2: QueryFilter,
{
    type Output = Xor<Self, F2>;

    fn bitxor(self, filter: F2) -> Self::Output {
        Xor(self, filter)
    }
}
