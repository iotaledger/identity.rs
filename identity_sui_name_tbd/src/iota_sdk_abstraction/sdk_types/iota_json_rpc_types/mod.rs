// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod iota_transaction;
pub mod iota_object;
pub mod iota_coin;
pub mod iota_event;
pub mod iota_move;

pub use iota_transaction::*;
pub use iota_object::*;
pub use iota_coin::*;
pub use iota_event::*;

use serde::{Deserialize, Serialize};

/// `next_cursor` points to the last item in the page;
/// Reading with `next_cursor` will start from the next item after `next_cursor`
/// if `next_cursor` is `Some`, otherwise it will start from the first item.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
    pub has_next_page: bool,
}