use crate::layout::{LayoutGroup, LayoutGroupFamily};
use std::mem;

pub(crate) const MAX_GROUP_COUNT: usize = 32;

/// Describes the layout of `ComponentStorage`s within a `World`.
pub struct Layout {
    families: Vec<LayoutGroupFamily>,
}

impl Layout {
    /// Returns an empty `LayoutBuilder`.
    pub fn builder() -> LayoutBuilder {
        LayoutBuilder::default()
    }

    pub(crate) fn families(&self) -> &[LayoutGroupFamily] {
        &self.families
    }
}

/// Enables creating a `Layout` using the builder pattern.
#[derive(Default)]
pub struct LayoutBuilder {
    families: Vec<Vec<LayoutGroup>>,
}

impl LayoutBuilder {
    /// Adds a `group` to the `Layout`. Panics if the group partially overlaps
    /// with previous groups.
    pub fn add_group(&mut self, group: LayoutGroup) -> &mut Self {
        let mut family_index = Option::<usize>::None;

        for (i, first_group) in
            self.families.iter().flat_map(|group_set| group_set.first()).enumerate()
        {
            if !group.components().is_disjoint(first_group.components()) {
                family_index = Some(i);

                for i in (i + 1)..self.families.len() {
                    assert!(
                        group
                            .components()
                            .is_disjoint(self.families[i].last().unwrap().components()),
                        "Groups are not allowed to only partially overlap",
                    )
                }

                break;
            }
        }

        match family_index {
            Some(i) => {
                let group_family = &mut self.families[i];

                for (i, old_group) in group_family.iter().enumerate() {
                    if group.components().is_subset(old_group.components()) {
                        group_family.insert(i, group);
                        return self;
                    } else {
                        assert!(
                            group.components().is_superset(old_group.components()),
                            "Groups are not allowed to only partially overlap",
                        );
                    }
                }

                group_family.push(group);
            }
            None => self.families.push(vec![group]),
        }

        self
    }

    /// Returns the `Layout` with the previously added groups.
    pub fn build(&mut self) -> Layout {
        let families = mem::take(&mut self.families)
            .iter()
            .map(|groups| unsafe { LayoutGroupFamily::new_unchecked(groups) })
            .collect::<Vec<_>>();

        let group_count = families.iter().map(|family| family.group_count()).sum::<usize>();

        assert!(
            group_count <= MAX_GROUP_COUNT,
            "Layouts can have at most {} groups",
            MAX_GROUP_COUNT
        );

        Layout { families }
    }
}
