// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_client_legacy::tangle::Client;
use identity_iota_client_legacy::tangle::SharedPtr;

use crate::account::Account;

/// A struct created by the [`Account::update_identity`] method, that
/// allows executing various updates on the identity it was created on.
#[derive(Debug)]
pub struct IdentityUpdater<'account, C>
where
  C: SharedPtr<Client>,
{
  pub(crate) account: &'account mut Account<C>,
}

impl<'account, C> IdentityUpdater<'account, C>
where
  C: SharedPtr<Client>,
{
  pub(crate) fn new(account: &'account mut Account<C>) -> Self {
    Self { account }
  }
}
