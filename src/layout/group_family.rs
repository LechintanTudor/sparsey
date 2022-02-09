use crate::layout::{ComponentInfo, LayoutGroup};

#[derive(Debug)]
pub(crate) struct LayoutGroupFamily {
    components: Vec<ComponentInfo>,
    group_arities: Vec<usize>,
}

impl LayoutGroupFamily {
    pub fn new(group: &LayoutGroup) -> Self {
        Self { components: group.components().to_vec(), group_arities: vec![group.arity()] }
    }

    pub fn try_add_group(&mut self, group: &LayoutGroup) -> bool {
        let family_components = self.components.as_slice();
        let group_components = group.components();

        assert!(
            group_components.len() >= family_components.len(),
            "Groups must be added from shortest to longest",
        );

        fn is_disjoint(group: &[ComponentInfo], family: &[ComponentInfo]) -> bool {
            !group.iter().any(|c| family.contains(c))
        }

        fn is_superset(group: &[ComponentInfo], family: &[ComponentInfo]) -> bool {
            family.iter().all(|c| group.contains(c))
        }

        if is_disjoint(group_components, family_components) {
            false
        } else {
            assert!(
                is_superset(group_components, family_components),
                "Groups must not partially overlap",
            );

            if group_components.len() > family_components.len() {
                let mut new_components = group_components
                    .iter()
                    .filter(|c| !family_components.contains(c))
                    .cloned()
                    .collect::<Vec<_>>();

                self.components.append(&mut new_components);
                self.group_arities.push(self.components.len());
            }

            true
        }
    }

    pub fn arity(&self) -> usize {
        self.components.len()
    }

    pub fn components(&self) -> &[ComponentInfo] {
        &self.components
    }

    pub fn group_arities(&self) -> &[usize] {
        &self.group_arities
    }
}
