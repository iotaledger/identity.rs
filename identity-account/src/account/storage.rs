// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use crate::storage::VaultAdapter;

pub enum AccountStorage {
  Stronghold,
  Custom(Box<dyn VaultAdapter>),
}

impl Debug for AccountStorage {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Stronghold => f.write_str("Stronghold"),
      Self::Custom(_) => f.write_str("Custom"),
    }
  }
}
