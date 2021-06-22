// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use communication_refactored::{
  firewall::{PermissionValue, RequestPermissions, ToPermissionVariants, VariantPermission},
  RqRsMessage,
};
use identity_account::{identity::IdentityId, types::KeyLocation};
use serde::{Deserialize, Serialize};

use std::fmt;

pub trait IdentityRequestHandler {
  type Request: fmt::Debug + RqRsMessage;
  type Response: fmt::Debug + RqRsMessage;

  fn handle(&mut self, request: Self::Request) -> Self::Response;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestPermissions)]
pub enum IdentityStorageRequest {
  KeyNew { id: IdentityId, location: KeyLocation },
  KeyGet { id: IdentityId, location: KeyLocation },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestPermissions)]
pub enum IdentityStorageResponse {
  KeyNew { public_key: Box<[u8]> },
  KeyGet { public_key: Box<[u8]> },
}

pub struct ForeignLanguageHandler {}

impl IdentityRequestHandler for ForeignLanguageHandler {
  type Request = Vec<u8>;
  type Response = Vec<u8>;

  fn handle(&mut self, _request: Self::Request) -> Self::Response {
    // Pass bytes off to foreign language
    todo!()
  }
}
