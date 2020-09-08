mod enums;
mod structs;

pub use crate::impls::{
    enums::{derive_diff_enum, impl_debug_enum},
    structs::{debug_impl, derive_diff_struct, diff_impl},
};
