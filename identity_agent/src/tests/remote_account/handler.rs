// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use dashmap::DashMap;
use identity_account::account::Account;
use identity_account::account::AccountBuilder;
use identity_iota_core_legacy::did::IotaDID;
use identity_iota_core_legacy::document::IotaDocument;
use tokio::sync::Mutex;

use crate::agent::Handler;
use crate::agent::RequestContext;
use crate::tests::remote_account::IdentityCreate;
use crate::tests::remote_account::IdentityGet;
use crate::tests::remote_account::IdentityList;
use crate::tests::remote_account::RemoteAccountError;

/// A proof-of-concept implementation of a remote `Account` with very basic operations
/// and disabled tangle interaction.
#[derive(Debug, Clone)]
pub(crate) struct RemoteAccount {
  builder: Arc<Mutex<AccountBuilder>>,
  accounts: Arc<DashMap<IotaDID, Account>>,
}

#[async_trait::async_trait]
impl Handler<IdentityList> for RemoteAccount {
  async fn handle(&self, _: RequestContext<IdentityList>) -> Vec<IotaDID> {
    self.accounts.iter().map(|entry| entry.key().to_owned()).collect()
  }
}

#[async_trait::async_trait]
impl Handler<IdentityCreate> for RemoteAccount {
  async fn handle(&self, request: RequestContext<IdentityCreate>) -> Result<IotaDocument, RemoteAccountError> {
    let account: Account = self.builder.lock().await.create_identity(request.input.into()).await?;
    let doc = account.document().to_owned();
    self.accounts.insert(account.did().to_owned(), account);
    Ok(doc)
  }
}

#[async_trait::async_trait]
impl Handler<IdentityGet> for RemoteAccount {
  async fn handle(&self, request: RequestContext<IdentityGet>) -> Result<IotaDocument, RemoteAccountError> {
    self
      .accounts
      .get(&request.input.0)
      .map(|account| account.document().to_owned())
      .ok_or(RemoteAccountError::IdentityNotFound)
  }
}

impl RemoteAccount {
  pub(crate) fn new() -> identity_account::Result<Self> {
    let builder: AccountBuilder = Account::builder().autopublish(false);

    Ok(Self {
      builder: Arc::new(Mutex::new(builder)),
      accounts: Arc::new(DashMap::new()),
    })
  }
}
