// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use super::Endpoint;

/// Expresses the synchronicity of a request at runtime, i.e. whether a request
/// is handled synchronously or asynchronously.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestMode {
  Synchronous,
  Asynchronous,
}

/// A request sent to a remote actor with a response of type `Response`.
///
/// This request is synchronous, which means to send the request to the peer and wait for
/// the result of that invocation.
pub trait ActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn endpoint() -> Endpoint;

  fn request_mode() -> RequestMode {
    RequestMode::Synchronous
  }
}
