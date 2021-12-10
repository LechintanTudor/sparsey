#[rustfmt::skip]
macro_rules! impl_generic_0_16 {
    ($macro:ident) => {
        $macro!();
        $macro!(A);
        $macro!(A, B);
        $macro!(A, B, C);
        $macro!(A, B, C, D);
        $macro!(A, B, C, D, E);
        $macro!(A, B, C, D, E, F);
        $macro!(A, B, C, D, E, F, G);
        $macro!(A, B, C, D, E, F, G, H);
        $macro!(A, B, C, D, E, F, G, H, I);
        $macro!(A, B, C, D, E, F, G, H, I, J);
        $macro!(A, B, C, D, E, F, G, H, I, J, K);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    };
}

#[rustfmt::skip]
macro_rules! impl_generic_1_16 {
    ($macro:ident) => {
        $macro!(A);
        $macro!(A, B);
        $macro!(A, B, C);
        $macro!(A, B, C, D);
        $macro!(A, B, C, D, E);
        $macro!(A, B, C, D, E, F);
        $macro!(A, B, C, D, E, F, G);
        $macro!(A, B, C, D, E, F, G, H);
        $macro!(A, B, C, D, E, F, G, H, I);
        $macro!(A, B, C, D, E, F, G, H, I, J);
        $macro!(A, B, C, D, E, F, G, H, I, J, K);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    };
}

#[rustfmt::skip]
macro_rules! impl_generic_2_16 {
    ($macro:ident) => {
        $macro!(A, B);
        $macro!(A, B, C);
        $macro!(A, B, C, D);
        $macro!(A, B, C, D, E);
        $macro!(A, B, C, D, E, F);
        $macro!(A, B, C, D, E, F, G);
        $macro!(A, B, C, D, E, F, G, H);
        $macro!(A, B, C, D, E, F, G, H, I);
        $macro!(A, B, C, D, E, F, G, H, I, J);
        $macro!(A, B, C, D, E, F, G, H, I, J, K);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
        $macro!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    };
}

#[rustfmt::skip]
macro_rules! impl_generic_tuple_1_16 {
    ($macro:ident) => {
        $macro!((A, 0));
        $macro!((A, 0), (B, 1));
        $macro!((A, 0), (B, 1), (C, 2));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
        $macro!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
    };
}

pub(crate) use {impl_generic_0_16, impl_generic_1_16, impl_generic_2_16, impl_generic_tuple_1_16};
