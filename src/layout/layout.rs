use crate::layout::{LayoutGroup, LayoutGroupFamily};

/// The maximum number of groups in a `Layout`.
pub const MAX_GROUP_COUNT: usize = 32;

/// Tracks which component storages should be grouped.
#[derive(Debug)]
pub struct Layout {
    families: Vec<LayoutGroupFamily>,
}

impl Layout {
    /// Enables creating a `Layout` using the builder pattern.
    pub fn builder() -> LayoutBuilder {
        Default::default()
    }

    pub(crate) fn families(&self) -> &[LayoutGroupFamily] {
        &self.families
    }
}

/// Enables creating a `Layout` using the builder pattern.
#[derive(Default)]
pub struct LayoutBuilder {
    groups: Vec<LayoutGroup>,
}

impl LayoutBuilder {
    /// Adds agroup to the layout.
    pub fn add_group(&mut self, group: LayoutGroup) -> &mut Self {
        self.groups.push(group);
        self
    }

    /// Builds the `Layout`.
    pub fn build(&mut self) -> Layout {
        self.groups.sort_by_key(LayoutGroup::arity);

        let mut families = Vec::<LayoutGroupFamily>::new();

        for group in &self.groups {
            let successes =
                families.iter_mut().map(|f| f.try_add_group(group) as usize).sum::<usize>();

            if successes == 0 {
                families.push(LayoutGroupFamily::new(group));
            } else if successes > 1 {
                panic!("Groups are not allowed to partially overlap");
            }
        }

        let group_count = families.iter().map(LayoutGroupFamily::arity).sum::<usize>();

        assert!(
            group_count <= MAX_GROUP_COUNT,
            "Layouts must have at most {} groups",
            MAX_GROUP_COUNT,
        );

        Layout { families }
    }
}
