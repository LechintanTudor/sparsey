use crate::component::{Component, ComponentData};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// Minimum number of component types required to form a group.
pub const MIN_GROUP_ARITY: usize = 2;

/// Maximum number of component types that can form a group.
pub const MAX_GROUP_ARITY: usize = 16;

/// Maximum number of groups that can be set on a [`World`](crate::world::World).
pub const MAX_GROUP_COUNT: usize = 64;

/// Describes the layout of the component groups that can be set on a
/// [`World`](crate::world::World).
#[derive(Clone, Default, Debug)]
pub struct GroupLayout {
    families: Vec<GroupFamily>,
}

impl GroupLayout {
    /// Adds a new group to the layout.
    pub fn add_group<G>(&mut self)
    where
        G: GroupDescriptor,
    {
        self.add_group_dyn(G::COMPONENTS);
    }

    /// Adds a new group to the layout.
    pub fn add_group_dyn(&mut self, components: &[ComponentData]) {
        let mut components = Vec::from(components);
        components.sort_unstable();
        components.dedup();

        if components.len() <= 1 {
            return;
        }

        assert!(
            components.len() <= MAX_GROUP_ARITY,
            "Groups must have at most {MAX_GROUP_ARITY} component types",
        );

        let successes = self
            .families
            .iter_mut()
            .map(|f| usize::from(f.try_add_group(&components)))
            .sum::<usize>();

        if successes == 0 {
            self.families.push(GroupFamily::new(components));
        } else {
            assert_eq!(
                successes, 1,
                "Groups families may not have any component types in common",
            );
        }
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

    /// Returns the components that are part of the group family.
    #[inline]
    #[must_use]
    pub fn components(&self) -> &[ComponentData] {
        &self.components
    }

    /// Returns the arities of the groups that form the group family.
    #[inline]
    #[must_use]
    pub fn arities(&self) -> &[usize] {
        &self.arities
    }

    #[must_use]
    fn try_add_group(&mut self, components: &[ComponentData]) -> bool {
        // Check if groups are disjoint.
        if self.components.iter().all(|c| !components.contains(c)) {
            return false;
        }

        // Find insertion index for new group.
        let mut index = Option::<usize>::None;

        for (i, &arity) in self.arities.iter().enumerate() {
            let prev_arity = i.checked_sub(1).map_or(0, |i| self.arities[i]);

            match arity.cmp(&components.len()) {
                Ordering::Less => {
                    let is_subset = self.components[prev_arity..arity]
                        .iter()
                        .all(|c| components.contains(c));

                    if !is_subset {
                        panic_incompatible_groups(components, &self.components[..arity]);
                    }
                }
                Ordering::Equal => {
                    let is_equal = self.components[prev_arity..arity]
                        .iter()
                        .all(|c| components.contains(c));

                    if !is_equal {
                        panic_incompatible_groups(components, &self.components[..arity]);
                    }

                    return true;
                }
                Ordering::Greater => {
                    let is_superset = self.components[prev_arity..arity]
                        .iter()
                        .all(|c| !components.contains(c));

                    if !is_superset {
                        panic_incompatible_groups(components, &self.components[..arity]);
                    }

                    index = Some(i);
                    break;
                }
            }
        }

        // Insert new group.
        if let Some(index) = index {
            let next_arity = self.arities[index];
            self.components[..next_arity].sort_by_cached_key(|c| components.contains(c));
            self.arities.insert(index, components.len());
        } else {
            for component in components {
                if !self.components.contains(component) {
                    self.components.push(*component);
                }
            }

            self.arities.push(components.len());
        }

        true
    }
}

/// Helper trait for creating groups in a [`GroupLayout`](crate::entity::GroupLayout).
pub trait GroupDescriptor {
    /// Slice containing the component data of the components present in the group.
    const COMPONENTS: &'static [ComponentData];
}

#[cold]
#[inline(never)]
fn panic_incompatible_groups(new_group: &[ComponentData], old_group: &[ComponentData]) -> ! {
    panic!("Cannot create GroupLayout due to incomptaible groups:\n -> {new_group:#?}\n -> {old_group:#?}")
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
