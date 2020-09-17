mod enums;
mod structs;

pub use crate::impls::{
    enums::{derive_diff_enum, impl_debug_enum, impl_diff_enum},
    structs::{debug_impl, derive_diff_struct, diff_impl, impl_from_into},
};
