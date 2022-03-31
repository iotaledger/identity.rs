// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::traits::Diff;

use std::fmt::Debug;
use std::fmt::Formatter;

/// A macro to implement the `Diff` traits on primitive values and the `Debug` trait on the resulting `Diff*` types.
/// Follows this syntax, impl_diff_on_primitives { type | DiffTypeName + TraitBounds}`
macro_rules! impl_diff_on_primitives {
    ($($type:ty | $diff:ident $(: $($traits:ident),+)?);* $(;)?) => {
        $(
            $( #[derive($($traits),+ )] )?
            #[derive(serde::Deserialize, serde::Serialize, Default)]
            pub struct $diff(#[doc(hidden)] pub Option<$type>);


            impl Diff for $type {
                type Type = $diff;

                #[inline(always)]
                fn diff(&self, other: &Self) -> crate::Result<Self::Type> {
                    other.clone().into_diff()
                }

                #[inline(always)]
                fn merge(&self, diff: Self::Type) -> crate::Result<Self> {
                    Self::from_diff(diff)
                }

                #[inline(always)]
                fn from_diff(diff: Self::Type) -> crate::Result<Self> {
                    match diff.0 {
                        Some(val) => Ok(val),
                        None => Ok(Self::default()),
                    }


                }

                #[inline(always)]
                fn into_diff(self) -> crate::Result<Self::Type> {
                    Ok($diff(Some(self)))
                }
            }


            impl Debug for $diff {
                fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
    i8 | Diffi8: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i16 | Diffi16: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i32 | Diffi32:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i64 | Diffi64:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    i128 | Diffi128:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    isize | DiffiSize: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;

    u8 | Diffu8:    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u16 | Diffu16:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u32 | Diffu32:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u64 | Diffu64:   Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    u128 | Diffu128:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    usize | DiffuSize: Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;

    f32 | DiffF32:   Clone, Copy, PartialEq, PartialOrd;
    f64  | DiffF64:   Clone, Copy, PartialEq, PartialOrd;
    bool | Diffbool:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    char | Diffchar:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
    () | Diffunit:  Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash;
}
