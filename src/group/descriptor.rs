use super::{GroupLayout, SubgroupLayout, WorldLayout};
use crate::registry::{Component, World};
use std::any::TypeId;

pub trait SubgroupLayoutDescriptor {
    fn subgroup_layout() -> SubgroupLayout;

    fn register_components(world: &mut World);
}

pub trait GroupLayoutDescriptor {
    fn group_layout() -> GroupLayout;

    fn register_components(world: &mut World);
}

pub trait WorldLayoutDescriptor {
    fn world_layout() -> WorldLayout;

    fn register_components(world: &mut World);
}

macro_rules! impl_subgroup_layout_descriptor {
    ($($comp:ident),+) => {
        impl<$($comp,)+> SubgroupLayoutDescriptor for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            fn subgroup_layout() -> SubgroupLayout {
                let mut subgroup_layout = SubgroupLayout::builder();
                $(subgroup_layout.add(TypeId::of::<$comp>());)+
                subgroup_layout.build()
            }

            fn register_components(world: &mut World) {
                $(world.register::<$comp>();)+
            }
        }
    };
}

macro_rules! impl_group_layout_descriptor {
    ($($subgroup_layout:ident),+) => {
        impl<$($subgroup_layout,)+> GroupLayoutDescriptor for ($($subgroup_layout,)+)
        where
            $($subgroup_layout: SubgroupLayoutDescriptor,)+
        {
            fn group_layout() -> GroupLayout {
                let mut group_layout = GroupLayout::builder();
                $(group_layout.add($subgroup_layout::subgroup_layout());)+
                group_layout.build()
            }

            fn register_components(world: &mut World) {
                $($subgroup_layout::register_components(world);)+
            }
        }
    };
}

macro_rules! impl_world_layout_descriptor {
    ($($group_layout:ident),*) => {
        impl<$($group_layout,)*> WorldLayoutDescriptor for ($($group_layout,)*)
        where
            $($group_layout: GroupLayoutDescriptor,)*
        {
            #[allow(unused_mut)]
            fn world_layout() -> WorldLayout {
                let mut world_layout = WorldLayout::builder();
                $(world_layout.add($group_layout::group_layout());)*
                world_layout.build()
            }

            #[allow(unused_variables)]
            fn register_components(world: &mut World) {
                $($group_layout::register_components(world);)*
            }
        }
    };
}

impl_subgroup_layout_descriptor!(A, B);
impl_subgroup_layout_descriptor!(A, B, C);
impl_subgroup_layout_descriptor!(A, B, C, D);
impl_subgroup_layout_descriptor!(A, B, C, D, E);
impl_subgroup_layout_descriptor!(A, B, C, D, E, F);
impl_subgroup_layout_descriptor!(A, B, C, D, E, F, G);
impl_subgroup_layout_descriptor!(A, B, C, D, E, F, G, H);

impl_group_layout_descriptor!(A);
impl_group_layout_descriptor!(A, B);
impl_group_layout_descriptor!(A, B, C);
impl_group_layout_descriptor!(A, B, C, D);
impl_group_layout_descriptor!(A, B, C, D, E);
impl_group_layout_descriptor!(A, B, C, D, E, F);
impl_group_layout_descriptor!(A, B, C, D, E, F, G);
impl_group_layout_descriptor!(A, B, C, D, E, F, G, H);

impl_world_layout_descriptor!();
impl_world_layout_descriptor!(A);
impl_world_layout_descriptor!(A, B);
impl_world_layout_descriptor!(A, B, C);
impl_world_layout_descriptor!(A, B, C, D);
impl_world_layout_descriptor!(A, B, C, D, E);
impl_world_layout_descriptor!(A, B, C, D, E, F);
impl_world_layout_descriptor!(A, B, C, D, E, F, G);
impl_world_layout_descriptor!(A, B, C, D, E, F, G, H);
