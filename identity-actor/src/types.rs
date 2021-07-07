// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use communication_refactored::firewall::{PermissionValue, RequestPermissions, VariantPermission};
use identity_account::{events::Command, identity::IdentityCreate, types::Signature};
use identity_iota::did::{IotaDID, IotaDocument};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait IdentityRequestHandler: Send + Sync {
  type Request: ActorRequest;

  async fn handle(
    &mut self,
    request: Self::Request,
  ) -> identity_account::Result<<Self::Request as ActorRequest>::Response>;
}

pub trait ActorRequest: Debug + Serialize + DeserializeOwned {
  type Response: Debug + Serialize + DeserializeOwned;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityStorageRequest {
  Create(IdentityCreate),
  Read(IotaDID),
  Update(IotaDID, Command),
  Delete(IotaDID),
  Sign(IotaDID, Vec<u8>),
  List,
}

impl ActorRequest for IdentityStorageRequest {
  type Response = IdentityStorageResponse;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityStorageResponse {
  Create(IotaDocument),
  Read(Option<IotaDocument>),
  Update,
  Delete,
  Sign(Signature),
  List(Vec<IotaDocument>),
}

#[derive(Debug, Clone, Serialize, Deserialize, RequestPermissions)]
pub struct NamedMessage {
  pub name: String,
  pub data: Vec<u8>,
}

impl NamedMessage {
  pub fn new<S: Into<String>>(name: S, data: Vec<u8>) -> Self {
    Self {
      name: name.into(),
      data,
    }
  }
}
