// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::{IdentityStorageRequest, IdentityStorageResponse};
use crate::IdentityRequestHandler;

pub struct IdentityStorageHandler {}

impl IdentityStorageHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl IdentityRequestHandler for IdentityStorageHandler {
  type Request = IdentityStorageRequest;
  type Response = IdentityStorageResponse;

  fn handle(&mut self, request: Self::Request) -> Self::Response {
    println!("Received {:?}", request);

    // TODO: PreProcessingHook

    let response = match request {
      IdentityStorageRequest::KeyNew { .. } => IdentityStorageResponse::KeyNew {
        public_key: Box::new([0]),
      },
      _ => todo!()
    };

    println!("Returning: {:?}", response);

    response

    // TODO: PostProcessingHook
  }
}
