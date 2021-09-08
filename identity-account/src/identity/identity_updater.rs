// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::Account;

use super::IdentityKey;

pub struct IdentityUpdater<'account, K: IdentityKey + Clone> {
  account: &'account Account,
  key: K,
}

impl<'account, K: IdentityKey + Clone> IdentityUpdater<'account, K> {
  pub fn new(account: &'account Account, key: K) -> Self {
    Self { account, key }
  }

  pub fn account(&self) -> &'account Account {
    self.account
  }

  pub fn key(&self) -> &K {
    &self.key
  }
}
