// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::Account;

use super::IdentityKey;

/// A struct created by the [`Account::update_identity`] method, that
/// allows executing various updates on the identity it was created on.
#[derive(Debug, Clone)]
pub struct IdentityUpdater<'account, K: IdentityKey + Clone> {
  account: &'account Account,
  key: K,
}

impl<'account, K: IdentityKey + Clone> IdentityUpdater<'account, K> {
  pub(crate) fn new(account: &'account Account, key: K) -> Self {
    Self { account, key }
  }

  pub(crate) fn account(&self) -> &'account Account {
    self.account
  }

  pub(crate) fn key(&self) -> &K {
    &self.key
  }
}
