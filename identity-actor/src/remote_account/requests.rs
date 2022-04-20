// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::types::IdentitySetup;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::actor::ActorRequest;
use crate::actor::Endpoint;
use crate::actor::Synchronous;

use super::RemoteAccountError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCreate(pub IdentitySetup);

impl ActorRequest<Synchronous> for IdentityCreate {
  type Response = Result<IotaDocument, RemoteAccountError>;

  fn endpoint() -> Endpoint {
    "remote_account/create".parse().unwrap()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityList;

impl ActorRequest<Synchronous> for IdentityList {
  type Response = Vec<IotaDID>;

  fn endpoint() -> Endpoint {
    "remote_account/list".parse().unwrap()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityGet(pub IotaDID);

impl ActorRequest<Synchronous> for IdentityGet {
  type Response = Result<IotaDocument, RemoteAccountError>;

  fn endpoint() -> Endpoint {
    "remote_account/get".parse().unwrap()
  }
}
