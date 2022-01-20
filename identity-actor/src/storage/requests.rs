// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_account::identity::IdentitySetup;
use identity_iota::did::IotaDID;
use identity_iota::document::IotaDocument;
use serde::Deserialize;
use serde::Serialize;

use crate::ActorRequest;

use super::StorageError;

impl ActorRequest for IdentitySetup {
  type Response = Result<IotaDocument, StorageError>;

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("storage/create")
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityList;

impl ActorRequest for IdentityList {
  type Response = Vec<IotaDID>;

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("storage/list")
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityResolve {
  pub did: IotaDID,
}

impl IdentityResolve {
  pub fn new(did: IotaDID) -> Self {
    Self { did }
  }
}

impl ActorRequest for IdentityResolve {
  type Response = Result<IotaDocument, StorageError>;

  fn request_name<'cow>(&self) -> std::borrow::Cow<'cow, str> {
    Cow::Borrowed("storage/resolve")
  }
}
