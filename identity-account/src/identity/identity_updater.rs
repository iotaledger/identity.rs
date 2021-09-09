// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::Account;

use super::IdentityKey;

/// A struct created by the [`Account::update_identity`] method, that
/// allows executing various updates on the identity it was created on.
#[derive(Debug, Clone)]
pub struct IdentityUpdater<'account, 'key, K: IdentityKey> {
  pub(crate) account: &'account Account,
  pub(crate) key: &'key K,
}

impl<'account, 'key, K: IdentityKey> IdentityUpdater<'account, 'key, K> {
  pub(crate) fn new(account: &'account Account, key: &'key K) -> Self {
    Self { account, key }
  }
}
