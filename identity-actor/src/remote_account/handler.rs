// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::actor::Actor;
use crate::actor::RequestContext;
use crate::remote_account::IdentityList;
use dashmap::DashMap;
use identity_account::account::Account;
use identity_account::account::AccountBuilder;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use tokio::sync::Mutex;

use super::requests::IdentityCreate;
use super::requests::IdentityGet;
use super::RemoteAccountError;

/// A proof-of-concept implementation of a remote `Account` with very basic operations
/// and disabled tangle interaction.
#[derive(Clone)]
pub struct RemoteAccount {
  builder: Arc<Mutex<AccountBuilder>>,
  accounts: Arc<DashMap<IotaDID, Account>>,
}

impl RemoteAccount {
  pub fn new() -> identity_account::Result<Self> {
    let builder: AccountBuilder = Account::builder().autopublish(false);

    Ok(Self {
      builder: Arc::new(Mutex::new(builder)),
      accounts: Arc::new(DashMap::new()),
    })
  }

  /// Creates a new identity using the `Account` API and returns the DID document.
  pub async fn create(
    self,
    // The request handler interface takes an Actor, but we don't use it here.
    _actor: Actor,
    request: RequestContext<IdentityCreate>,
  ) -> Result<IotaDocument, RemoteAccountError> {
    let account: Account = self.builder.lock().await.create_identity(request.input.into()).await?;
    let doc = account.document().to_owned();
    self.accounts.insert(account.did().to_owned(), account);
    Ok(doc)
  }

  /// List all the stored identities.
  pub async fn list(
    self,
    // The request handler interface takes an Actor, but we don't use it here.
    _actor: Actor,
    _request: RequestContext<IdentityList>,
  ) -> Vec<IotaDID> {
    self.accounts.iter().map(|entry| entry.key().to_owned()).collect()
  }

  /// Get the DID document of a given DID.
  pub async fn get(
    self,
    // The request handler interface takes an Actor, but we don't use it here.
    _actor: Actor,
    request: RequestContext<IdentityGet>,
  ) -> Result<IotaDocument, RemoteAccountError> {
    self
      .accounts
      .get(&request.input.0)
      .map(|account| account.document().to_owned())
      .ok_or(RemoteAccountError::IdentityNotFound)
  }
}
