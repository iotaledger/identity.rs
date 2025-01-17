// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod account_address;
pub mod annotated_value;
pub mod annotated_visitor;
pub mod identifier;
pub mod language_storage;
pub mod parser;
pub mod runtime_value;
pub mod u256;

use std::fmt;

pub(crate) fn fmt_list<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    begin: &str,
    items: impl IntoIterator<Item = T>,
    end: &str,
) -> fmt::Result {
    write!(f, "{}", begin)?;
    let mut items = items.into_iter();
    if let Some(x) = items.next() {
        write!(f, "{}", x)?;
        for x in items {
            write!(f, ", {}", x)?;
        }
    }
    write!(f, "{}", end)?;
    Ok(())
}