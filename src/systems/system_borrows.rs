use crate::systems::{SystemDataDescriptor, SystemDataType};
use crate::utils::impl_generic_0_to_16;

/// Trait for collecting the data types borrowed by systems during execution.
pub trait SystemBorrows<TParams, TReturn> {
    /// Collects the system data borrowed by system during execution into the provided vector.
    fn collect_system_borrows(&self, borrows: &mut Vec<SystemDataType>);
}

macro_rules! impl_system_borrows {
    ($($TParam:ident),*) => {
        impl<TFunc, $($TParam,)* TReturn> SystemBorrows<($($TParam,)*), TReturn> for TFunc
        where
            TFunc: FnOnce($($TParam),*) -> TReturn,
            $($TParam: SystemDataDescriptor,)*
        {
            #[allow(unused_variables)]
            fn collect_system_borrows(&self, borrows: &mut Vec<SystemDataType>) {
                $(borrows.push($TParam::system_data_type());)*
            }
        }
    };
}

impl_generic_0_to_16!(impl_system_borrows);
