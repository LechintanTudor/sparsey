use super::{Group, GroupSet, Subgroup};
use crate::registry::Component;
use std::any::TypeId;

pub trait SubgroupLayout {
    fn components() -> Subgroup;
}

pub trait GroupLayout {
    fn subgroups() -> Group;
}

pub trait WorldLayout {
    fn groups() -> GroupSet;
}

macro_rules! impl_subgroup_layout {
    ($($comp:ident),+) => {
        impl<$($comp,)+> SubgroupLayout for ($($comp,)+)
        where
            $($comp: Component,)+
        {
            fn components() -> Subgroup {
                let mut subgroup = Subgroup::builder();
                $(subgroup.add(TypeId::of::<$comp>());)+
                subgroup.build()
            }
        }
    };
}

macro_rules! impl_group_layout {
    ($($subgroup:ident),+) => {
        impl<$($subgroup,)+> GroupLayout for ($($subgroup,)+)
        where
            $($subgroup: SubgroupLayout,)+
        {
            fn subgroups() -> Group {
                let mut group = Group::builder();
                $(group.add($subgroup::components());)+
                group.build()
            }
        }
    };
}

macro_rules! impl_world_layout {
    ($($group:ident),*) => {
        impl<$($group,)*> WorldLayout for ($($group,)*)
        where
            $($group: GroupLayout,)*
        {
            fn groups() -> GroupSet {
                #[allow(unused_mut)]
                let mut group_set = GroupSet::builder();
                $(group_set.add($group::subgroups());)*
                group_set.build()
            }
        }
    };
}

impl_subgroup_layout!(A, B);
impl_subgroup_layout!(A, B, C);
impl_subgroup_layout!(A, B, C, D);
impl_subgroup_layout!(A, B, C, D, E);
impl_subgroup_layout!(A, B, C, D, E, F);
impl_subgroup_layout!(A, B, C, D, E, F, G);
impl_subgroup_layout!(A, B, C, D, E, F, G, H);

impl_group_layout!(A);
impl_group_layout!(A, B);
impl_group_layout!(A, B, C);
impl_group_layout!(A, B, C, D);
impl_group_layout!(A, B, C, D, E);
impl_group_layout!(A, B, C, D, E, F);
impl_group_layout!(A, B, C, D, E, F, G);
impl_group_layout!(A, B, C, D, E, F, G, H);

impl_world_layout!();
impl_world_layout!(A);
impl_world_layout!(A, B);
impl_world_layout!(A, B, C);
impl_world_layout!(A, B, C, D);
impl_world_layout!(A, B, C, D, E);
impl_world_layout!(A, B, C, D, E, F);
impl_world_layout!(A, B, C, D, E, F, G);
impl_world_layout!(A, B, C, D, E, F, G, H);
