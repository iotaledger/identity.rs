// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::Actor;
use crate::IdentityList;
use crate::RequestContext;
use identity_account::account::Account;
use identity_account::account::AccountBuilder;
use identity_account::identity::IdentitySetup;
use identity_iota::did::IotaDID;
use identity_iota::document::IotaDocument;
use identity_iota::document::ResolvedIotaDocument;
use identity_iota::tangle::Client;
use identity_iota::tangle::ClientBuilder;
use identity_iota::tangle::Network;

use super::requests::IdentityResolve;
use super::StorageError;

#[derive(Clone)]
pub struct StorageHandler {
  client: Arc<Client>,
}

impl StorageHandler {
  pub async fn new() -> identity_account::Result<Self> {
    let client = ClientBuilder::new().network(Network::Mainnet).build().await?;

    Ok(Self {
      client: Arc::new(client),
    })
  }

  pub async fn create(
    self,
    _actor: Actor,
    request: RequestContext<IdentitySetup>,
  ) -> Result<IotaDocument, StorageError> {
    let mut account_builder: AccountBuilder = AccountBuilder::new();
    let account: Account = account_builder.create_identity(request.input).await?;
    let doc = account.document().to_owned();
    Ok(doc)
  }

  pub async fn list(self, _actor: Actor, _request: RequestContext<IdentityList>) -> Vec<IotaDID> {
    vec![]
  }

  pub async fn resolve(
    self,
    _actor: Actor,
    request: RequestContext<IdentityResolve>,
  ) -> Result<ResolvedIotaDocument, StorageError> {
    log::info!("Resolving {:?}", request.input.did);

    let res = self.client.read_document(&request.input.did).await?;

    log::info!("Resolved into: {:?}", res);

    Ok(res)
  }
}
