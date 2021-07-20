// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::requests::AuthenticationRequest;

#[derive(Clone)]
pub struct DIDCommHandler;

impl DIDCommHandler {
  pub async fn new() -> Self {
    Self
  }

  pub async fn authenticate(self, _input: AuthenticationRequest) {
    todo!()
  }
}
