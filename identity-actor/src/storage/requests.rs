// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account::identity::IdentityCreate;
use identity_iota::did::{IotaDID, IotaDocument};
use serde::{Deserialize, Serialize};

use crate::traits::ActorRequest;

impl ActorRequest for IdentityCreate {
  type Response = IotaDocument;

  fn request_name() -> &'static str {
    "storage/create"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityList;

impl ActorRequest for IdentityList {
  type Response = Vec<IotaDID>;

  fn request_name() -> &'static str {
    "storage/list"
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityResolve(pub IotaDID);

impl ActorRequest for IdentityResolve {
  type Response = Option<IotaDocument>;

  fn request_name() -> &'static str {
    "storage/resolve"
  }
}
