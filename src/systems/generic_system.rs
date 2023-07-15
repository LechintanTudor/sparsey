use crate::systems::{SystemDataDescriptor, SystemDataType};
use crate::utils::impl_generic_0_to_16;

pub trait GenericSystem<TParams, TReturn> {
    fn collect_system_borrows(borrows: &mut Vec<SystemDataType>);
}

macro_rules! impl_system_borrows {
    ($($TParam:ident),*) => {
        impl<TFunc, $($TParam,)* TReturn> GenericSystem<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn,
            $($TParam: SystemDataDescriptor,)*
        {
            #[allow(unused_variables)]
            fn collect_system_borrows(borrows: &mut Vec<SystemDataType>) {
                $(borrows.push($TParam::system_data_type());)*
            }
        }
    };
}

impl_generic_0_to_16!(impl_system_borrows);
