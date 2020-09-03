use crate::traits::Diff;

use std::fmt::{Debug, Formatter, Result as FmtResult};

macro_rules! impl_diff_on_primitives {
    ($($type:ty | $diff:ident $(: $($traits:ident),+)?);* $(;)?) => {
        $(
            $( #[derive($($traits),+ )] )?
            #[derive(serde::Deserialize, serde::Serialize, Default)]
            pub struct $diff(#[doc(hidden)] pub Option<$type>);


            impl Diff for $type {
                type Type = $diff;

                #[inline(always)]
                fn diff(&self, other: &Self) -> Self::Type {
                    other.clone().into_diff()
                }

                #[inline(always)]
                fn merge(&self, diff: Self::Type) -> Self {
                    Self::from_diff(diff)
                }

                #[inline(always)]
                fn from_diff(diff: Self::Type) -> Self {
                    match diff.0 {
                        Some(val) => val,
                        None => Self::default(),
                    }


                }

                #[inline(always)]
                fn into_diff(self) -> Self::Type {
                    $diff(Some(self))
                }
            }


            impl Debug for $diff {
                fn fmt(&self, f: &mut  Formatter) -> FmtResult {
                    match self.0 {
                        None => write!(f, "{} None", stringify!($diff)),
                        Some(val) => write!(f, "{} => {:#?}", stringify!($diff), val),
                    }
                }
            }

        )*
    };
}

impl_diff_on_primitives! {
    i8 | I8Diff: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i16 | I16Diff: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i32 | I32Diff:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i64 | I64Diff:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i128 | I128Diff:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    isize | IsizeDiff: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;

    u8 | U8Diff:    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u16 | U16Diff:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u32 | U32Diff:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u64 | U64Diff:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u128 | U128Diff:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    usize | UsizeDiff: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;

    f32 | F32Diff:   Clone, Copy, PartialEq, PartialOrd;
    f64  | F64Diff:   Clone, Copy, PartialEq, PartialOrd;
    bool | BoolDiff:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    char | CharDiff:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    () | UnitDiff:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
}
