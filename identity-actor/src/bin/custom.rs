// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use communication_refactored::firewall::{
  PermissionValue, RequestPermissions, ToPermissionVariants, VariantPermission,
};
use identity_actor::{IdentityCommunicator, IdentityRequestHandler};
use serde::{Deserialize, Serialize};

pub struct CustomIdentityHandler {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestPermissions)]
pub enum CustomRequest {
  MyCustomRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RequestPermissions)]
pub enum CustomResponse {
  MyCustomResponse,
}

impl IdentityRequestHandler for CustomIdentityHandler {
  type Request = CustomRequest;
  type Response = CustomResponse;
  // type RequestPermission = CustomRequestPermission;

  fn handle(&mut self, request: Self::Request) -> Self::Response {
    match request {
      CustomRequest::MyCustomRequest => CustomResponse::MyCustomResponse,
    }
  }
}

pub type CustomIdentityCommunicator =
  IdentityCommunicator<CustomRequest, CustomResponse, CustomRequestPermission, CustomIdentityHandler>;

#[async_std::main]
async fn main() {
  let handler = CustomIdentityHandler {};
  let mut comm = CustomIdentityCommunicator::new(handler).await;
  comm.start_listening(None).await;
}
