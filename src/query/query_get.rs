use crate::components::{Component, ComponentGroupInfo, QueryGroupInfo};
use crate::query::{ChangeTicksFilter, IntoQueryParts, IterData, Passthrough};
use crate::storage::{Entity, EntitySparseArray};
use crate::utils::{ChangeTicks, Ticks};

#[derive(Clone, Copy)]
pub struct ComponentViewData<T> {
    pub components: *mut T,
    pub ticks: *mut ChangeTicks,
}

impl<T> ComponentViewData<T> {
    pub const fn new(components: *mut T, ticks: *mut ChangeTicks) -> Self {
        Self { components, ticks }
    }
}

pub unsafe trait GetComponent<'a> {
    type Item: 'a;
    type Component: Component;

    fn group_info(&self) -> Option<ComponentGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<usize>;

    unsafe fn matches_unchecked<F>(&self, index: usize) -> bool
    where
        F: ChangeTicksFilter;

    unsafe fn get_unchecked<F>(self, index: usize) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter;

    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        *mut Self::Component,
        *mut ChangeTicks,
    );

    unsafe fn get_from_parts_unchecked<F>(
        components: *mut Self::Component,
        ticks: *mut ChangeTicks,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> (Self::Item, bool)
    where
        F: ChangeTicksFilter;
}

pub unsafe trait GetImmutableComponent<'a>
where
    Self: GetComponent<'a>,
{
    fn entities(&self) -> &'a [Entity];

    fn components(&self) -> &'a [Self::Component];
}

pub unsafe trait GetComponentSetUnfiltered<'a> {
    type Item: 'a;
    type Index: Copy;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn matches_unchecked<F>(&self, index: Self::Index) -> bool
    where
        F: ChangeTicksFilter;

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_from_sparse_unchecked<F>(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;

    unsafe fn get_from_dense_unchecked<F>(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter;
}

unsafe impl<'a, G> GetComponentSetUnfiltered<'a> for G
where
    G: GetComponent<'a>,
{
    type Item = G::Item;
    type Index = usize;
    type Sparse = &'a EntitySparseArray;
    type Data = ComponentViewData<G::Component>;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        QueryGroupInfo::new(GetComponent::group_info(self)?)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponent::change_detection_ticks(self)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        GetComponent::get_index(self, entity)
    }

    unsafe fn matches_unchecked<F>(&self, index: Self::Index) -> bool
    where
        F: ChangeTicksFilter,
    {
        GetComponent::matches_unchecked::<F>(self, index)
    }

    unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        let (item, matches) = GetComponent::get_unchecked::<F>(self, index);
        matches.then(|| item)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        let (entities, sparse, components, ticks) = GetComponent::split(self);
        (entities, sparse, ComponentViewData::new(components, ticks))
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        let (entities, _, components, ticks) = GetComponent::split(self);
        (entities, ComponentViewData::new(components, ticks))
    }

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index> {
        sparse.get_entity(entity).map(|e| e.dense())
    }

    unsafe fn get_from_sparse_unchecked<F>(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        let (item, matches) = G::get_from_parts_unchecked::<F>(
            data.components,
            data.ticks,
            index,
            world_tick,
            change_tick,
        );

        matches.then(|| item)
    }

    unsafe fn get_from_dense_unchecked<F>(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: ChangeTicksFilter,
    {
        let (item, matches) = G::get_from_parts_unchecked::<F>(
            data.components,
            data.ticks,
            index,
            world_tick,
            change_tick,
        );

        matches.then(|| item)
    }
}

pub unsafe trait GetComponentSet<'a> {
    type Item: 'a;
    type Filter: ChangeTicksFilter;
    type Index: Copy;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn get_index(&self, entity: Entity) -> Option<Self::Index>;

    unsafe fn matches_unchecked(&self, index: Self::Index) -> bool;

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item>;

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

    fn split_dense(self) -> (&'a [Entity], Self::Data);

    fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

    unsafe fn get_from_sparse_unchecked(
        data: &Self::Data,
        index: Self::Index,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;
}

unsafe impl<'a, G> GetComponentSet<'a> for G
where
    G: GetComponentSetUnfiltered<'a>,
{
    type Item = G::Item;
    type Filter = Passthrough;
    type Index = G::Index;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        GetComponentSetUnfiltered::group_info(self)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentSetUnfiltered::change_detection_ticks(self)
    }

    fn get_index(&self, entity: Entity) -> Option<Self::Index> {
        GetComponentSetUnfiltered::get_index(self, entity)
    }

    unsafe fn matches_unchecked(&self, index: Self::Index) -> bool {
        GetComponentSetUnfiltered::matches_unchecked::<Self::Filter>(self, index)
    }

    unsafe fn get_unchecked(self, index: Self::Index) -> Option<Self::Item> {
        GetComponentSetUnfiltered::get_unchecked::<Self::Filter>(self, index)
    }

    fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data) {
        GetComponentSetUnfiltered::split_sparse(self)
    }

    fn split_dense(self) -> (&'a [Entity], Self::Data) {
        GetComponentSetUnfiltered::split_dense(self)
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
        G::get_from_sparse_unchecked::<Self::Filter>(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked::<Self::Filter>(data, index, world_tick, change_tick)
    }
}

pub unsafe trait QueryGet<'a> {
    type Item: 'a;
    type Sparse: 'a;
    type Data: 'a;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>>;

    fn change_detection_ticks(&self) -> (Ticks, Ticks);

    fn contains(&self, entity: Entity) -> bool;

    fn get(self, entity: Entity) -> Option<Self::Item>;

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data);

    fn split_dense(self) -> (IterData<'a>, Self::Data);

    unsafe fn get_from_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>;
}

unsafe impl<'a, G> QueryGet<'a> for G
where
    G: GetComponentSet<'a>,
{
    type Item = G::Item;
    type Sparse = G::Sparse;
    type Data = G::Data;

    fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
        GetComponentSet::group_info(self)
    }

    fn change_detection_ticks(&self) -> (Ticks, Ticks) {
        GetComponentSet::change_detection_ticks(self)
    }

    fn contains(&self, entity: Entity) -> bool {
        let index = match GetComponentSet::get_index(self, entity) {
            Some(index) => index,
            None => return false,
        };

        unsafe { GetComponentSet::matches_unchecked(self, index) }
    }

    fn get(self, entity: Entity) -> Option<Self::Item> {
        let index = GetComponentSet::get_index(&self, entity)?;
        unsafe { GetComponentSet::get_unchecked(self, index) }
    }

    fn split_sparse(self) -> (IterData<'a>, Self::Sparse, Self::Data) {
        let (world_tick, change_tick) = GetComponentSet::change_detection_ticks(&self);
        let (entities, sparse, data) = GetComponentSet::split_sparse(self);

        (
            IterData::new(entities, world_tick, change_tick),
            sparse,
            data,
        )
    }

    fn split_dense(self) -> (IterData<'a>, Self::Data) {
        let (world_tick, change_tick) = GetComponentSet::change_detection_ticks(&self);
        let (entities, _, data) = GetComponentSet::split_sparse(self);

        (IterData::new(entities, world_tick, change_tick), data)
    }

    unsafe fn get_from_sparse_unchecked(
        sparse: &'a Self::Sparse,
        entity: Entity,
        data: &Self::Data,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        let index = G::get_index_from_sparse(sparse, entity)?;
        G::get_from_sparse_unchecked(data, index, world_tick, change_tick)
    }

    unsafe fn get_from_dense_unchecked(
        data: &Self::Data,
        index: usize,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item> {
        G::get_from_dense_unchecked(data, index, world_tick, change_tick)
    }
}

impl<'a, G> IntoQueryParts<'a> for G
where
    G: QueryGet<'a>,
{
    type Get = G;
    type Include = Passthrough;
    type Exclude = Passthrough;
    type Filter = Passthrough;

    fn into_query_parts(self) -> (Self::Get, Self::Include, Self::Exclude, Self::Filter) {
        (self, Passthrough, Passthrough, Passthrough)
    }
}

macro_rules! replace {
    ($from:tt, $to:tt) => {
        $to
    };
}

macro_rules! new_query_group_info {
    ($first:expr) => {
        Some(QueryGroupInfo::new($first))
    };
    ($first:expr $(, $other:expr)+) => {
        QueryGroupInfo::new($first) $(.and_then(|i| i.include($other)))+
    };
}

macro_rules! impl_query_get {
    ($($elem:ident, $idx:tt),+) => {
        unsafe impl<'a, $($elem),+> GetComponentSetUnfiltered<'a> for ($($elem,)+)
        where
            $($elem: GetComponent<'a>,)+
        {
            type Item = ($($elem::Item,)+);
            type Index = ($(replace!($elem, usize),)+);
            type Sparse = ($(&'a EntitySparseArray,)+);
            type Data = ($(ComponentViewData<$elem::Component>,)+);

            fn group_info(&self) -> Option<QueryGroupInfo<'a>> {
                new_query_group_info!(self.$idx)
            }

            fn change_detection_ticks(&self) -> (Ticks, Ticks) {
                self.0.change_detection_ticks()
            }

            fn get_index(&self, entity: Entity) -> Option<Self::Index> {
                Some((
                    $(self.$idx.get_index(entity)?,)+
                ))
            }

            fn contains<F>(&self, entity: Entity) -> bool
            where
                F: ChangeTicksFilter,
            {
                let index = match self.get_index(entity) {
                    Some(index) => index,
                    None => return false,
                };

                if F::IS_PASSTHROUGH {
                    true
                } else {
                    unsafe {
                        $(self.$idx.matches_unchecked::<F>(index.$idx))||+
                    }
                }
            }

            unsafe fn get_unchecked<F>(self, index: Self::Index) -> Option<Self::Item>
            where
                F: ChangeTicksFilter;

            fn split_sparse(self) -> (&'a [Entity], Self::Sparse, Self::Data);

            fn split_dense(self) -> (&'a [Entity], Self::Data);

            fn get_index_from_sparse(sparse: &Self::Sparse, entity: Entity) -> Option<Self::Index>;

            unsafe fn get_from_sparse_unchecked<F>(
                data: &Self::Data,
                index: Self::Index,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item>
            where
                F: ChangeTicksFilter;

            unsafe fn get_from_dense_unchecked<F>(
                data: &Self::Data,
                index: usize,
                world_tick: Ticks,
                change_tick: Ticks,
            ) -> Option<Self::Item>
            where
                F: ChangeTicksFilter;
                }
    };
}
