// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use communication_refactored::RqRsMessage;
use identity_account::{events::Command, identity::IdentityCreate, types::Signature};
use identity_iota::did::{IotaDID, IotaDocument};
use serde::{Deserialize, Serialize};
use std::fmt;

#[async_trait::async_trait]
pub trait IdentityRequestHandler {
  type Request: fmt::Debug + RqRsMessage;
  type Response: fmt::Debug + RqRsMessage;

  async fn handle(&mut self, request: Self::Request) -> identity_account::Result<Self::Response>;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityStorageResponse {
  Create(IotaDocument),
  Read(Option<IotaDocument>),
  Update,
  Delete,
  Sign(Signature),
  List(Vec<IotaDocument>),
}
