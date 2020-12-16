use std::any::TypeId;

#[derive(Debug)]
pub struct Subgroup {
    components: Box<[TypeId]>,
}

impl Subgroup {
    pub fn builder() -> SubgroupBuilder {
        Default::default()
    }
}

#[derive(Default)]
pub struct SubgroupBuilder {
    components: Vec<TypeId>,
}

impl SubgroupBuilder {
    pub fn add(&mut self, component: TypeId) {
        self.components.push(component);
    }

    pub fn build(mut self) -> Subgroup {
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

        Subgroup {
            components: self.components.into(),
        }
    }
}

#[derive(Debug)]
pub struct Group {
    components: Box<[TypeId]>,
    subgroup_ends: Box<[usize]>,
}

impl Group {
    pub fn builder() -> GroupBuilder {
        Default::default()
    }

    pub fn into_components_and_subgroup_ends(self) -> (Box<[TypeId]>, Box<[usize]>) {
        (self.components, self.subgroup_ends)
    }
}

#[derive(Default)]
pub struct GroupBuilder {
    components: Vec<TypeId>,
    subgroup_ends: Vec<usize>,
}

impl GroupBuilder {
    pub fn add(&mut self, subgroup: Subgroup) {
        assert!(
            subgroup.components.len() > self.components.len(),
            "Child subgroup must contain more types than the parent subgroup",
        );

        let mut overlapped_count = 0_usize;
        let mut new_components = Vec::<TypeId>::new();

        for &component in subgroup.components.iter() {
            if self.components.contains(&component) {
                overlapped_count += 1;
            } else {
                new_components.push(component);
            }
        }

        assert_eq!(
            overlapped_count,
            self.subgroup_ends.last().copied().unwrap_or(0),
            "Child subgroup must contain all types from the parent subgroup",
        );

        for &component in &new_components {
            self.components.push(component);
        }

        self.subgroup_ends.push(self.components.len());
    }

    pub fn build(mut self) -> Group {
        Group {
            components: self.components.drain(..).collect::<Vec<_>>().into(),
            subgroup_ends: self.subgroup_ends.into(),
        }
    }
}

#[derive(Debug)]
pub struct GroupSet {
    groups: Box<[Group]>,
}

impl GroupSet {
    pub fn builder() -> GroupSetBuilder {
        Default::default()
    }

    pub fn into_groups(self) -> Box<[Group]> {
        self.groups
    }
}

#[derive(Default)]
pub struct GroupSetBuilder {
    groups: Vec<Group>,
}

impl GroupSetBuilder {
    pub fn add(&mut self, group: Group) {
        for other_group in self.groups.iter() {
            for component in group.components.iter() {
                assert!(
                    !other_group.components.contains(component),
                    "Groups must not contain overlapping types",
                );
            }
        }

        self.groups.push(group);
    }

    pub fn build(self) -> GroupSet {
        GroupSet {
            groups: self.groups.into(),
        }
    }
}
