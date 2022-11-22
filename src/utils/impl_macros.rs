macro_rules! impl_generic_0_to_16 {
    ($impl_macro:tt) => {
        $impl_macro!();
        $impl_macro!(A);
        $impl_macro!(A, B);
        $impl_macro!(A, B, C);
        $impl_macro!(A, B, C, D);
        $impl_macro!(A, B, C, D, E);
        $impl_macro!(A, B, C, D, E, F);
        $impl_macro!(A, B, C, D, E, F, G);
        $impl_macro!(A, B, C, D, E, F, G, H);
        $impl_macro!(A, B, C, D, E, F, G, H, I);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J, K);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J, K, L);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J, K, L, M);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
        $impl_macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    };
}

pub(crate) use impl_generic_0_to_16;
