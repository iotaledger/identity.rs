// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use super::Endpoint;

/// Used to represent the synchronicity of a request at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestMode {
  Synchronous,
  Asynchronous,
}
/// A request that can be sent to an actor with the expected response being of type `Response`.
///
/// This request is synchronous, which means to invoke the handler on the remote and wait for
/// the result of that invocation.
pub trait SyncActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn endpoint() -> Endpoint;

  fn request_mode() -> RequestMode {
    RequestMode::Synchronous
  }
}

/// A message that can be sent to an actor without an explicit response.
///
/// This message is sent asynchronously, which means to invoke the handler on the remote without waiting
/// for its completion. An acknowledgment is returned to signal that the handler exists and can be invoked
/// or an error, if the opposite is true.
pub trait AsyncActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  fn endpoint() -> Endpoint;

  fn request_mode() -> RequestMode {
    RequestMode::Asynchronous
  }
}
