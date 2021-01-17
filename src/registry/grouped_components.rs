use crate::group::WorldLayout;
use crate::storage::{AbstractSparseSet, SparseSet};
use atomic_refcell::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::hint::unreachable_unchecked;

use super::Component;

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
    subgroup_index: usize,
}

#[derive(Default)]
struct ComponentGroup {
    components: Vec<AtomicRefCell<Box<dyn AbstractSparseSet>>>,
    subgroups: Vec<Subgroup>,
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
                            subgroup_index,
                        },
                    );
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

    pub fn contains(&self, component: TypeId) -> bool {
        self.component_info.contains_key(&component)
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
