use crate::group::WorldLayout;
use crate::registry::Component;
use crate::storage::{
    AbstractSparseSet, AbstractSparseSetView, AbstractSparseSetViewMut, Entity, SparseSet,
};
use atomic_refcell::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::slice::SliceIndex;

#[derive(Default)]
pub struct Subgroup {
    arity: usize,
    length: usize,
}

impl Subgroup {
    pub fn from_arity(arity: usize) -> Self {
        Self { arity, length: 0 }
    }
}

#[derive(Default)]
struct ComponentInfo {
    group_index: usize,
    local_index: usize,
    _subgroup_index: usize,
}

#[derive(Default)]
pub struct ComponentGroup {
    components: Vec<AtomicRefCell<Box<dyn AbstractSparseSet>>>,
    subgroups: Vec<Subgroup>,
}

impl ComponentGroup {
    pub unsafe fn split(
        &mut self,
    ) -> (
        ComponentGroupSparseSetsViewMut,
        ComponentGroupSubgroupsViewMut,
    ) {
        (
            ComponentGroupSparseSetsViewMut::new(&mut self.components),
            ComponentGroupSubgroupsViewMut::new(&mut self.subgroups),
        )
    }
}

pub struct ComponentGroupSparseSetsViewMut<'a> {
    components: &'a mut [AtomicRefCell<Box<dyn AbstractSparseSet>>],
}

impl<'a> ComponentGroupSparseSetsViewMut<'a> {
    fn new(components: &'a mut [AtomicRefCell<Box<dyn AbstractSparseSet>>]) -> Self {
        Self { components }
    }

    pub fn iter_abstract_sparse_set_views<I>(
        &mut self,
        range: I,
    ) -> impl DoubleEndedIterator<Item = AbstractSparseSetView>
    where
        I: SliceIndex<
            [AtomicRefCell<Box<dyn AbstractSparseSet>>],
            Output = [AtomicRefCell<Box<dyn AbstractSparseSet>>],
        >,
    {
        (&mut self.components[range])
            .iter_mut()
            .map(|s| s.get_mut().as_abstract_view())
    }

    pub unsafe fn iter_abstract_sparse_set_views_mut<I>(
        &mut self,
        range: I,
    ) -> impl DoubleEndedIterator<Item = AbstractSparseSetViewMut>
    where
        I: SliceIndex<
            [AtomicRefCell<Box<dyn AbstractSparseSet>>],
            Output = [AtomicRefCell<Box<dyn AbstractSparseSet>>],
        >,
    {
        (&mut self.components[range])
            .iter_mut()
            .map(|s| s.get_mut().as_abstract_view_mut())
    }
}

pub struct ComponentGroupSubgroupsViewMut<'a> {
    subgroups: &'a mut [Subgroup],
}

impl<'a> ComponentGroupSubgroupsViewMut<'a> {
    fn new(subgroups: &'a mut [Subgroup]) -> Self {
        Self { subgroups }
    }

    pub unsafe fn iter_split_subgroups_mut<I>(
        &mut self,
        range: I,
    ) -> impl DoubleEndedIterator<Item = (usize, &mut usize)>
    where
        I: SliceIndex<[Subgroup], Output = [Subgroup]>,
    {
        (&mut self.subgroups[range])
            .iter_mut()
            .map(|s| (s.arity, &mut s.length))
    }
}

#[derive(Default)]
pub struct GroupedComponents {
    component_groups: Vec<ComponentGroup>,
    component_info: HashMap<TypeId, ComponentInfo>,
}

impl GroupedComponents {
    pub fn new(world_layout: WorldLayout) -> Self {
        let mut component_groups = Vec::<ComponentGroup>::new();
        let mut component_info = HashMap::<TypeId, ComponentInfo>::new();

        for group_layout in world_layout.group_layouts() {
            let mut component_group = ComponentGroup::default();

            let components = group_layout.components();
            let mut previous_arity = 0_usize;

            for (subgroup_index, &arity) in group_layout.subgroup_arities().iter().enumerate() {
                for component in &components[previous_arity..arity] {
                    component_info.insert(
                        component.component_type_id(),
                        ComponentInfo {
                            group_index: component_groups.len(),
                            local_index: component_group.components.len(),
                            _subgroup_index: subgroup_index,
                        },
                    );

                    component_group
                        .components
                        .push(AtomicRefCell::new(component.create_sparse_set()));
                }

                component_group.subgroups.push(Subgroup::from_arity(arity));
                previous_arity = arity;
            }

            component_groups.push(component_group);
        }

        Self {
            component_groups,
            component_info,
        }
    }

    pub fn destroy(&mut self, _entity: Entity) {
        // for group in self.component_groups.iter_mut() {
        //     let (sets, subgroups) = unsafe { group.split() };
        // }
    }

    pub fn maintain(&mut self) {
        for group in self.component_groups.iter_mut() {
            for sparse_set in group.components.iter_mut() {
                sparse_set.get_mut().maintain();
            }
        }
    }

    pub fn contains(&self, component: TypeId) -> bool {
        self.component_info.contains_key(&component)
    }

    pub fn group_index_for(&self, component: TypeId) -> Option<usize> {
        self.component_info.get(&component).map(|c| c.group_index)
    }

    pub unsafe fn get_component_group_split_view_mut_unchecked(
        &mut self,
        index: usize,
    ) -> (
        ComponentGroupSparseSetsViewMut,
        ComponentGroupSubgroupsViewMut,
    ) {
        self.component_groups.get_unchecked_mut(index).split()
    }

    pub fn borrow<T>(&self) -> Option<AtomicRef<SparseSet<T>>>
    where
        T: Component,
    {
        self.borrow_abstract(TypeId::of::<T>()).map(|s| {
            AtomicRef::map(s, |s| match s.as_any().downcast_ref::<SparseSet<T>>() {
                Some(s) => s,
                None => unsafe { unreachable_unchecked() },
            })
        })
    }

    pub fn borrow_mut<T>(&self) -> Option<AtomicRefMut<SparseSet<T>>>
    where
        T: Component,
    {
        self.borrow_abstract_mut(TypeId::of::<T>()).map(|s| {
            AtomicRefMut::map(s, |s| match s.as_mut_any().downcast_mut::<SparseSet<T>>() {
                Some(s) => s,
                None => unsafe { unreachable_unchecked() },
            })
        })
    }

    pub fn borrow_abstract(&self, component: TypeId) -> Option<AtomicRef<dyn AbstractSparseSet>> {
        self.component_info.get(&component).map(|c| unsafe {
            AtomicRef::map(
                self.component_groups
                    .get_unchecked(c.group_index)
                    .components
                    .get_unchecked(c.local_index)
                    .borrow(),
                |s| Box::as_ref(s),
            )
        })
    }

    pub fn borrow_abstract_mut(
        &self,
        component: TypeId,
    ) -> Option<AtomicRefMut<dyn AbstractSparseSet>> {
        self.component_info.get(&component).map(|c| unsafe {
            AtomicRefMut::map(
                self.component_groups
                    .get_unchecked(c.group_index)
                    .components
                    .get_unchecked(c.local_index)
                    .borrow_mut(),
                |s| Box::as_mut(s),
            )
        })
    }
}
