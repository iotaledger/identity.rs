// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::traits::ActorRequest;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthenticationRequest;

impl ActorRequest for AuthenticationRequest {
  type Response = ();

  fn request_name() -> &'static str {
    "didcomm/authenticate"
  }
}
