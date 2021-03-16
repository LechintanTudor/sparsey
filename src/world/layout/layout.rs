use crate::world::{LayoutGroup, LayoutGroupSet};
use std::mem;

/// Describes the layout of grouped component storages in the `World`.
pub struct Layout {
    group_sets: Vec<LayoutGroupSet>,
}

impl Layout {
    /// Create a `LayoutBuilder` to enable creating a `Layout`
    /// using the builder pattern.
    pub fn builder() -> LayoutBuilder {
        LayoutBuilder::default()
    }

    pub(crate) fn group_sets(&self) -> &[LayoutGroupSet] {
        &self.group_sets
    }
}

/// Implements the builder pattern to create `Layout`.
#[derive(Default)]
pub struct LayoutBuilder {
    group_sets: Vec<Vec<LayoutGroup>>,
}

impl LayoutBuilder {
    /// Add a group to the `Layout`.
    pub fn add_group(&mut self, group: LayoutGroup) -> &mut Self {
        let mut group_set_index = Option::<usize>::None;

        for (i, first_group) in self
            .group_sets
            .iter()
            .flat_map(|group_set| group_set.first())
            .enumerate()
        {
            if !group.components().is_disjoint(first_group.components()) {
                group_set_index = Some(i);

                for i in (i + 1)..self.group_sets.len() {
                    assert!(
                        group
                            .components()
                            .is_disjoint(self.group_sets[i].last().unwrap().components()),
                        "Groups are not allowed to partially overlap",
                    )
                }

                break;
            }
        }

        match group_set_index {
            Some(i) => {
                let group_set = &mut self.group_sets[i];

                for (i, old_group) in group_set.iter().enumerate() {
                    if group.components().is_subset(old_group.components()) {
                        group_set.insert(i, group);
                        return self;
                    } else {
                        assert!(
                            group.components().is_superset(old_group.components()),
                            "Groups are not allowed to partially overlap",
                        );
                    }
                }

                group_set.push(group);
            }
            None => self.group_sets.push(vec![group]),
        }

        self
    }

    /// Build the `Layout` using the previously given `LayoutGroups`.
    pub fn build(&mut self) -> Layout {
        let group_sets = mem::take(&mut self.group_sets);

        Layout {
            group_sets: group_sets
                .iter()
                .map(|groups| unsafe { LayoutGroupSet::new_unchecked(&groups) })
                .collect(),
        }
    }
}
