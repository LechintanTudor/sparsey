use crate::group::WorldLayout;
use crate::storage::AbstractSparseSet;
use atomic_refcell::*;
use std::any::TypeId;
use std::collections::HashMap;

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
}
