use crate::layout::ComponentInfo;
use crate::storage::Component;

pub const MIN_GROUP_ARITY: usize = 2;
pub const MAX_GROUP_ARITY: usize = 16;

pub struct LayoutGroup {
    components: Vec<ComponentInfo>,
}

impl LayoutGroup {
    pub fn new(mut components: Vec<ComponentInfo>) -> Self {
        let initial_len = components.len();

        components.sort();
        components.dedup();

        let current_len = components.len();

        if current_len != initial_len {
            panic!("Groups cannot contain duplicate components");
        } else if current_len < MIN_GROUP_ARITY {
            panic!("Groups must contain at least {} components", MIN_GROUP_ARITY);
        } else if current_len > MAX_GROUP_ARITY {
            panic!("Groups must contain at most {} components", MAX_GROUP_ARITY);
        }

        Self { components }
    }

    pub fn arity(&self) -> usize {
        self.components.len()
    }

    pub fn components(&self) -> &[ComponentInfo] {
        &self.components
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
                LayoutGroup::new(vec![$(ComponentInfo::new::<$comp>()),+])
            }
        }
    };
}

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
