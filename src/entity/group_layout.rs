use crate::entity::Component;
use crate::util::TypeData;

/// Minimum number of component types required to form a group.
pub const MIN_GROUP_ARITY: usize = 2;

/// Maximum number of component types that can form a group.
pub const MAX_GROUP_ARITY: usize = 16;

/// Maximum number of groups that can be set on an [`EntityStorage`](crate::entity::EntityStorage).
pub const MAX_GROUP_COUNT: usize = 64;

/// Describes the layout of the component groups that can be set on an
/// [`EntityStorage`](crate::entity::EntityStorage).
#[derive(Clone, Default, Debug)]
pub struct GroupLayout {
    families: Vec<GroupFamily>,
}

impl GroupLayout {
    /// Returns a builder that can be used to construct a new [`GroupLayout`].
    #[inline]
    pub fn builder() -> GroupLayoutBuilder {
        GroupLayoutBuilder::default()
    }

    /// Returns the group families of this layout.
    #[inline]
    #[must_use]
    pub(crate) fn families(&self) -> &[GroupFamily] {
        &self.families
    }
}

/// Describes a set of related component groups.
#[derive(Clone, Debug)]
pub(crate) struct GroupFamily {
    components: Vec<TypeData>,
    arities: Vec<usize>,
}

impl GroupFamily {
    fn new(components: Vec<TypeData>) -> Self {
        Self {
            arities: vec![components.len()],
            components,
        }
    }

    /// Returns the components that are part of the group family.
    #[inline]
    #[must_use]
    pub fn components(&self) -> &[TypeData] {
        &self.components
    }

    /// Returns the arities of the groups that form the group family.
    #[inline]
    #[must_use]
    pub fn arities(&self) -> &[usize] {
        &self.arities
    }

    fn try_add_group(&mut self, components: &[TypeData]) -> bool {
        assert!(
            components.len() >= self.components.len(),
            "Groups must be added from least restrictive to most restrictive",
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
    fn is_disjoint(&self, components: &[TypeData]) -> bool {
        !self.components.iter().any(|c| components.contains(c))
    }

    #[must_use]
    fn is_subset_of(&self, components: &[TypeData]) -> bool {
        self.components.iter().all(|c| components.contains(c))
    }
}

/// Builder that can be used to construct a new [`GroupLayout`].
#[must_use]
#[derive(Clone, Default, Debug)]
pub struct GroupLayoutBuilder {
    groups: Vec<Vec<TypeData>>,
}

impl GroupLayoutBuilder {
    /// Adds a new group to the layout.
    pub fn add_group<G>(&mut self) -> &mut Self
    where
        G: GroupDescriptor,
    {
        self.add_group_dyn(G::COMPONENTS)
    }

    /// Adds a new group to the layout, created from the given `components`.
    pub fn add_group_dyn(&mut self, components: &[TypeData]) -> &mut Self {
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

    /// Builds the group layout from the previously added groups.
    pub fn build(&mut self) -> GroupLayout {
        self.groups.sort_by_key(Vec::len);

        let mut families = Vec::<GroupFamily>::new();

        for group in self.groups.drain(..) {
            let successes = families
                .iter_mut()
                .map(|f| usize::from(f.try_add_group(&group)))
                .sum::<usize>();

            if successes == 0 {
                families.push(GroupFamily::new(group));
            } else {
                assert_eq!(successes, 1, "Group must belong to a single family");
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

/// Helper trait for creating groups in a [`GroupLayout`](crate::entity::GroupLayout).
pub trait GroupDescriptor {
    /// Slice containing the component data of the components present in the group.
    const COMPONENTS: &'static [TypeData];
}

macro_rules! impl_group_descriptor {
    ($($Comp:ident),*) => {
        impl<$($Comp,)*> GroupDescriptor for ($($Comp,)*)
        where
            $($Comp: Component,)*
        {
            const COMPONENTS: &'static [TypeData] = &[
                $(TypeData::new::<$Comp>(),)*
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
