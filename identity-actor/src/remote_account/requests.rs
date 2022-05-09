// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::types::IdentitySetup;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::actor::Endpoint;
use crate::actor::SyncActorRequest;

use super::RemoteAccountError;

/// Can be sent to a `RemoteAccount` to instruct it to create an identity.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IdentityCreate;

impl From<IdentityCreate> for IdentitySetup {
  fn from(_: IdentityCreate) -> Self {
    IdentitySetup::default()
  }
}

impl SyncActorRequest for IdentityCreate {
  type Response = Result<IotaDocument, RemoteAccountError>;

  fn endpoint() -> Endpoint {
    "remote_account/create".try_into().unwrap()
  }
}

/// Can be sent to a `RemoteAccount` to instruct it to return the identities it contains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityList;

impl SyncActorRequest for IdentityList {
  type Response = Vec<IotaDID>;

  fn endpoint() -> Endpoint {
    "remote_account/list".try_into().unwrap()
  }
}

/// Can be sent to a `RemoteAccount` to instruct it to return the given identities' DID document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityGet(pub IotaDID);

impl SyncActorRequest for IdentityGet {
  type Response = Result<IotaDocument, RemoteAccountError>;

  fn endpoint() -> Endpoint {
    "remote_account/get".try_into().unwrap()
  }
}
