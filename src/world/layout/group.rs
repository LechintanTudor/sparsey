use crate::data::Component;
use crate::world::LayoutComponent;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct LayoutGroup {
    components: HashSet<LayoutComponent>,
}

impl LayoutGroup {
    pub(crate) fn new(components: HashSet<LayoutComponent>) -> Self {
        assert!(
            components.len() > 1,
            "Groups must have at least 2 component types",
        );

        assert!(
            components.len() <= 16,
            "Groups must have at most 16 component types",
        );

        Self { components }
    }

    pub(crate) fn components(&self) -> &HashSet<LayoutComponent> {
        &self.components
    }
}

pub(crate) struct LayoutGroupSet {
    components: Vec<LayoutComponent>,
    arities: Vec<usize>,
}

impl LayoutGroupSet {
    pub unsafe fn new_unchecked(groups: &[LayoutGroup]) -> Self {
        let mut components = Vec::<LayoutComponent>::new();
        let mut arities = Vec::<usize>::new();

        components.extend(groups[0].components().iter().cloned());
        arities.push(groups[0].components().len());

        for (g1, g2) in groups.windows(2).map(|g| (&g[0], &g[1])) {
            let component_count = components.len();
            components.extend(g2.components().difference(g1.components()).cloned());

            if components.len() > component_count {
                arities.push(components.len());
            }
        }

        Self {
            components,
            arities,
        }
    }

    pub fn components(&self) -> &[LayoutComponent] {
        &self.components
    }

    pub fn arities(&self) -> &[usize] {
        &self.arities
    }
}

pub trait LayoutGroupDescriptor {
    fn group() -> LayoutGroup;
}

macro_rules! impl_layout_group_descriptor {
    ($($comp:ident),+) => {
        impl<$($comp),+> LayoutGroupDescriptor for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            fn group() -> LayoutGroup {
                LayoutGroup::new(HashSet::from_iter(vec![
                    $(LayoutComponent::new::<$comp>()),+
                ]))
            }
        }
    };
}

#[rustfmt::skip]
mod impls {
    use super::*;

    impl_layout_group_descriptor!(A, B);
    impl_layout_group_descriptor!(A, B, C);
    impl_layout_group_descriptor!(A, B, C, D);
    impl_layout_group_descriptor!(A, B, C, D, E);
    impl_layout_group_descriptor!(A, B, C, D, E, F);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_layout_group_descriptor!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
}
