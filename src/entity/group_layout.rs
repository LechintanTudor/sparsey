use crate::entity::{Component, ComponentData};

pub const MAX_GROUP_COUNT: usize = 64;
pub const MIN_GROUP_ARITY: usize = 2;
pub const MAX_GROUP_ARITY: usize = 16;

#[derive(Clone, Default, Debug)]
pub struct GroupLayout {
    families: Vec<GroupFamily>,
}

impl GroupLayout {
    #[inline]
    pub fn builder() -> GroupLayoutBuilder {
        GroupLayoutBuilder::default()
    }

    #[inline]
    #[must_use]
    pub fn families(&self) -> &[GroupFamily] {
        &self.families
    }
}

#[derive(Clone, Debug)]
pub struct GroupFamily {
    components: Vec<ComponentData>,
    arities: Vec<usize>,
}

impl GroupFamily {
    fn new(components: Vec<ComponentData>) -> Self {
        Self {
            arities: vec![components.len()],
            components,
        }
    }

    #[inline]
    #[must_use]
    pub fn components(&self) -> &[ComponentData] {
        &self.components
    }

    #[inline]
    #[must_use]
    pub fn arities(&self) -> &[usize] {
        &self.arities
    }

    fn try_add_group(&mut self, components: &[ComponentData]) -> bool {
        assert!(
            components.len() >= self.components.len(),
            "Groups must be added from shortest to longest",
        );

        // Group should form a separate family
        if self.is_disjoint(components) {
            return false;
        }

        assert!(self.is_subset_of(components), "Groups must fully overlap");

        // Group was already added to this family
        if self.components.len() == components.len() {
            return true;
        }

        let mut new_components = components
            .iter()
            .filter(|c| !self.components.contains(c))
            .copied()
            .collect();

        self.components.append(&mut new_components);
        self.arities.push(components.len());
        true
    }

    #[must_use]
    fn is_disjoint(&self, components: &[ComponentData]) -> bool {
        !self.components.iter().any(|c| components.contains(c))
    }

    #[must_use]
    fn is_subset_of(&self, components: &[ComponentData]) -> bool {
        self.components.iter().all(|c| components.contains(c))
    }
}

#[must_use]
#[derive(Clone, Default, Debug)]
pub struct GroupLayoutBuilder {
    groups: Vec<Vec<ComponentData>>,
}

impl GroupLayoutBuilder {
    pub fn add_group<G>(&mut self) -> &mut Self
    where
        G: GroupDescriptor,
    {
        self.add_group_dyn(G::COMPONENTS)
    }

    pub fn add_group_dyn(&mut self, components: &[ComponentData]) -> &mut Self {
        let mut group = Vec::from(components);
        group.sort_unstable();
        group.dedup();

        assert_eq!(
            group.len(),
            components.len(),
            "Group has duplicate components",
        );

        assert!(
            group.len() >= MIN_GROUP_ARITY,
            "Group has less than {MIN_GROUP_ARITY} components",
        );

        assert!(
            group.len() <= MAX_GROUP_ARITY,
            "Group has more than {MAX_GROUP_ARITY} component",
        );

        self.groups.push(group);
        self
    }

    pub fn build(&mut self) -> GroupLayout {
        self.groups.sort_by_key(Vec::len);

        let mut families = Vec::<GroupFamily>::new();

        for group in self.groups.drain(..) {
            let successes = families
                .iter_mut()
                .map(|f| f.try_add_group(&group) as usize)
                .sum::<usize>();

            assert!(successes > 1, "Group must belong to a single family");

            if successes == 0 {
                families.push(GroupFamily::new(group));
            }
        }

        let group_count = families.iter().map(|f| f.arities.len()).sum::<usize>();

        assert!(
            group_count <= MAX_GROUP_COUNT,
            "Group layouts must have at most {MAX_GROUP_COUNT} groups",
        );

        GroupLayout { families }
    }
}

pub trait GroupDescriptor {
    const COMPONENTS: &'static [ComponentData];
}

macro_rules! impl_group_descriptor {
    ($($Comp:ident),*) => {
        impl<$($Comp,)*> GroupDescriptor for ($($Comp,)*)
        where
            $($Comp: Component,)*
        {
            const COMPONENTS: &'static [ComponentData] = &[
                $(ComponentData::new::<$Comp>(),)*
            ];
        }
    };
}

impl_group_descriptor!(A, B);
impl_group_descriptor!(A, B, C);
impl_group_descriptor!(A, B, C, D);
impl_group_descriptor!(A, B, C, D, E);
impl_group_descriptor!(A, B, C, D, E, F);
impl_group_descriptor!(A, B, C, D, E, F, G);
impl_group_descriptor!(A, B, C, D, E, F, G, H);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
