#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub(crate) struct GroupMask {
    include: u16,
    exclude: u16,
}

impl GroupMask {
    pub const EMPTY: Self = Self::new(0, 0);

    pub const fn new(include: u16, exclude: u16) -> Self {
        Self { include, exclude }
    }

    pub const fn new_include_group(arity: usize) -> Self {
        Self {
            include: (1 << arity) - 1,
            exclude: 0,
        }
    }

    pub const fn new_exclude_group(prev_arity: usize, arity: usize) -> Self {
        if prev_arity != 0 {
            let exclude_count = arity - prev_arity;

            Self {
                include: (1 << prev_arity) - 1,
                exclude: ((1 << exclude_count) - 1) << prev_arity,
            }
        } else {
            Self::EMPTY
        }
    }
}
