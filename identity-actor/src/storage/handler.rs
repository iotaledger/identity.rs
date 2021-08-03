// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::IdentityList;
use identity_account::identity::IdentityCreate;
use identity_iota::{did::{IotaDID, IotaDocument}, tangle::{ClientBuilder, ClientMap, Network, TangleResolve}};

use super::requests::IdentityResolve;

#[derive(Clone)]
pub struct StorageHandler {
  client: Arc<ClientMap>,
}

impl StorageHandler {
  pub async fn new() -> identity_account::Result<Self> {
    let builder = ClientBuilder::new().network(Network::Mainnet);

    Ok(Self {
      client: Arc::new(ClientMap::from_builder(builder).await?),
    })
  }

  pub async fn create(self, _input: IdentityCreate) -> IotaDocument {
    todo!()
  }

  pub async fn list(self, _input: IdentityList) -> Vec<IotaDID> {
    vec![]
  }

  pub async fn resolve(self, input: IdentityResolve) -> Option<IotaDocument> {
    log::info!("Resolving {:?}", input.did);

    let res = self.client.resolve(&input.did).await;

    log::info!("Resolved into: {:?}", res);

    res.ok()
  }
}
