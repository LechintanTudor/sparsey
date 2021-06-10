use crate::layout::{LayoutGroup, LayoutGroupFamily};
use std::mem;

/// Describes the layout of grouped component storages in the `World`.
pub struct Layout {
	group_families: Vec<LayoutGroupFamily>,
}

impl Layout {
	/// Create a `LayoutBuilder` to enable creating a `Layout`
	/// using the builder pattern.
	pub fn builder() -> LayoutBuilder {
		LayoutBuilder::default()
	}

	pub(crate) fn group_families(&self) -> &[LayoutGroupFamily] {
		&self.group_families
	}
}

/// Implements the builder pattern to create `Layout`.
#[derive(Default)]
pub struct LayoutBuilder {
	group_families: Vec<Vec<LayoutGroup>>,
}

impl LayoutBuilder {
	/// Add a group to the `Layout`.
	pub fn add_group(&mut self, group: LayoutGroup) -> &mut Self {
		let mut group_family_index = Option::<usize>::None;

		for (i, first_group) in self
			.group_families
			.iter()
			.flat_map(|group_set| group_set.first())
			.enumerate()
		{
			if !group.components().is_disjoint(first_group.components()) {
				group_family_index = Some(i);

				for i in (i + 1)..self.group_families.len() {
					assert!(
						group
							.components()
							.is_disjoint(self.group_families[i].last().unwrap().components()),
						"Groups are not allowed to only partially overlap",
					)
				}

				break;
			}
		}

		match group_family_index {
			Some(i) => {
				let group_family = &mut self.group_families[i];

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
			None => self.group_families.push(vec![group]),
		}

		self
	}

	/// Build the `Layout` using the previously given `LayoutGroups`.
	pub fn build(&mut self) -> Layout {
		let group_families = mem::take(&mut self.group_families)
			.iter()
			.map(|groups| unsafe { LayoutGroupFamily::new_unchecked(&groups) })
			.collect();

		Layout { group_families }
	}
}
