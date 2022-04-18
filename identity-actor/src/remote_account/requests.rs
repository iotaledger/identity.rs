// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::types::IdentitySetup;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::actor::ActorRequest;
use crate::actor::Synchronous;

use super::RemoteAccountError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityCreate(pub IdentitySetup);

impl ActorRequest<Synchronous> for IdentityCreate {
  type Response = Result<IotaDocument, RemoteAccountError>;

  fn endpoint() -> &'static str {
    "remote_account/create"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityList;

impl ActorRequest<Synchronous> for IdentityList {
  type Response = Vec<IotaDID>;

  fn endpoint() -> &'static str {
    "remote_account/list"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityGet(pub IotaDID);

impl ActorRequest<Synchronous> for IdentityGet {
  type Response = Result<IotaDocument, RemoteAccountError>;

  fn endpoint() -> &'static str {
    "remote_account/get"
  }
}
