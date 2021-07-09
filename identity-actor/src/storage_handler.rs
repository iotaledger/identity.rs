// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::account::Account;
use identity_account::identity::IdentityCreate;
use identity_iota::did::{IotaDID, IotaDocument};
use serde::{Deserialize, Serialize};

use crate::types::ActorRequest;

pub struct IdentityStorageHandler {
  account: Account,
}

impl ActorRequest for IdentityCreate {
  type Response = IotaDocument;

  fn request_name() -> &'static str {
    "storage/create"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List;

impl ActorRequest for List {
  type Response = Vec<IotaDocument>;

  fn request_name() -> &'static str {
    "storage/list"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolve(IotaDID);

impl ActorRequest for Resolve {
  type Response = Option<IotaDocument>;

  fn request_name() -> &'static str {
    "storage/resolve"
  }
}

impl IdentityStorageHandler {
  pub async fn new() -> identity_account::Result<Self> {
    Ok(Self {
      account: Account::builder().build().await?,
    })
  }

  pub fn list(&self, _input: List) -> Vec<IotaDocument> {
    vec![]
  }

  pub fn resolve(&self, _input: Resolve) -> Option<IotaDocument> {
    None
  }
}
