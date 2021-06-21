use crate::components::Component;
use crate::layout::LayoutComponent;
use std::collections::HashSet;
use std::iter::FromIterator;

pub const MIN_GROUP_ARITY: usize = 2;
pub const MAX_GROUP_ARITY: usize = 16;

/// Describes a set of component storages to be grouped together.
pub struct LayoutGroup {
	components: HashSet<LayoutComponent>,
}

impl LayoutGroup {
	pub(crate) fn new(components: HashSet<LayoutComponent>) -> Self {
		assert!(
			components.len() >= MIN_GROUP_ARITY,
			"Groups must have at least {} component types",
			MIN_GROUP_ARITY,
		);

		assert!(
			components.len() <= MAX_GROUP_ARITY,
			"Groups must have at most {} component types",
			MAX_GROUP_ARITY,
		);

		Self { components }
	}

	pub(crate) fn components(&self) -> &HashSet<LayoutComponent> {
		&self.components
	}
}

pub(crate) struct LayoutGroupFamily {
	components: Vec<LayoutComponent>,
	group_arities: Vec<usize>,
}

impl LayoutGroupFamily {
	pub unsafe fn new_unchecked(groups: &[LayoutGroup]) -> Self {
		let mut components = Vec::<LayoutComponent>::new();
		let mut group_arities = Vec::<usize>::new();

		components.extend(groups[0].components().iter().cloned());
		group_arities.push(groups[0].components().len());

		for (g1, g2) in groups.windows(2).map(|g| (&g[0], &g[1])) {
			let component_count = components.len();
			components.extend(g2.components().difference(g1.components()).cloned());

			if components.len() > component_count {
				group_arities.push(components.len());
			}
		}

		Self {
			components,
			group_arities,
		}
	}

	pub fn components(&self) -> &[LayoutComponent] {
		&self.components
	}

	pub fn group_arities(&self) -> &[usize] {
		&self.group_arities
	}
}

/// Trait used for creating a `LayoutGroup`.
/// Implemented for tuples up to arity 16.
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
                LayoutGroup::new(HashSet::from_iter([
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
