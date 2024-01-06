use crate::entity::{Component, ComponentData};

#[derive(Clone, Default, Debug)]
pub struct GroupLayout {
    components: Vec<ComponentData>,
}

impl GroupLayout {
    #[inline]
    pub fn builder() -> GroupLayoutBuilder {
        GroupLayoutBuilder::default()
    }
}

#[must_use]
#[derive(Clone, Default, Debug)]
pub struct GroupLayoutBuilder {
    //
}

impl GroupLayoutBuilder {
    pub fn add_group<G>(&mut self) -> &mut Self
    where
        G: GroupDescriptor,
    {
        self.add_group_dyn(G::COMPONENTS)
    }

    pub fn add_group_dyn(&mut self, _components: &[ComponentData]) -> &mut Self {
        todo!()
    }

    pub fn build(&mut self) -> GroupLayout {
        todo!()
    }
}

pub trait GroupDescriptor {
    const COMPONENTS: &'static [ComponentData];
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
