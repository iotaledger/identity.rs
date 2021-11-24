// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::account::Account;

/// A struct created by the [`Account::update_identity`] method, that
/// allows executing various updates on the identity it was created on.
#[derive(Debug)]
pub struct IdentityUpdater<'account> {
  pub(crate) account: &'account mut Account,
}

impl<'account> IdentityUpdater<'account> {
  pub(crate) fn new(account: &'account mut Account) -> Self {
    Self { account }
  }
}
