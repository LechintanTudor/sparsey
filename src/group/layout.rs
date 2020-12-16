use std::any::TypeId;

#[derive(Debug)]
pub struct SubgroupLayout {
    components: Box<[TypeId]>,
}

impl SubgroupLayout {
    pub fn builder() -> SubgroupLayoutBuilder {
        Default::default()
    }
}

#[derive(Default)]
pub struct SubgroupLayoutBuilder {
    components: Vec<TypeId>,
}

impl SubgroupLayoutBuilder {
    pub fn add(&mut self, component: TypeId) {
        self.components.push(component);
    }

    pub fn build(mut self) -> SubgroupLayout {
        assert!(
            self.components.len() >= 2,
            "Subgroup must contain at least 2 types",
        );

        let initial_len = self.components.len();
        self.components.dedup();

        assert_eq!(
            initial_len,
            self.components.len(),
            "Subgroup cannot contain the same type more than once",
        );

        SubgroupLayout {
            components: self.components.into(),
        }
    }
}

#[derive(Debug)]
pub struct GroupLayout {
    components: Box<[TypeId]>,
    subgroup_arities: Box<[usize]>,
}

impl GroupLayout {
    pub fn builder() -> GroupLayoutBuilder {
        Default::default()
    }

    pub fn into_components_and_arities(self) -> (Box<[TypeId]>, Box<[usize]>) {
        (self.components, self.subgroup_arities)
    }
}

#[derive(Default)]
pub struct GroupLayoutBuilder {
    components: Vec<TypeId>,
    subgroup_arities: Vec<usize>,
}

impl GroupLayoutBuilder {
    pub fn add(&mut self, subgroup_layout: SubgroupLayout) {
        assert!(
            subgroup_layout.components.len() > self.components.len(),
            "Child subgroup must contain more types than the parent subgroup",
        );

        let mut overlapped_count = 0_usize;
        let mut new_components = Vec::<TypeId>::new();

        for &component in subgroup_layout.components.iter() {
            if self.components.contains(&component) {
                overlapped_count += 1;
            } else {
                new_components.push(component);
            }
        }

        assert_eq!(
            overlapped_count,
            self.subgroup_arities.last().copied().unwrap_or(0),
            "Child subgroup must contain all types from the parent subgroup",
        );

        for &component in &new_components {
            self.components.push(component);
        }

        self.subgroup_arities.push(self.components.len());
    }

    pub fn build(mut self) -> GroupLayout {
        GroupLayout {
            components: self.components.drain(..).collect::<Vec<_>>().into(),
            subgroup_arities: self.subgroup_arities.into(),
        }
    }
}

#[derive(Debug)]
pub struct WorldLayout {
    group_layouts: Box<[GroupLayout]>,
}

impl WorldLayout {
    pub fn builder() -> WorldLayoutBuilder {
        Default::default()
    }

    pub fn into_group_layouts(self) -> Box<[GroupLayout]> {
        self.group_layouts
    }
}

#[derive(Default)]
pub struct WorldLayoutBuilder {
    group_layouts: Vec<GroupLayout>,
}

impl WorldLayoutBuilder {
    pub fn add(&mut self, group_layout: GroupLayout) {
        for other_group_layout in self.group_layouts.iter() {
            for component in group_layout.components.iter() {
                assert!(
                    !other_group_layout.components.contains(component),
                    "Groups must not contain overlapping types",
                );
            }
        }

        self.group_layouts.push(group_layout);
    }

    pub fn build(self) -> WorldLayout {
        WorldLayout {
            group_layouts: self.group_layouts.into(),
        }
    }
}
