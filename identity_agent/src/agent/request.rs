// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::agent::Endpoint;

/// Expresses the synchronicity of a request at runtime, i.e. whether a request
/// is handled synchronously or asynchronously.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestMode {
  Synchronous,
  Asynchronous,
}

/// A request sent to a handler with a response of type `Response`.
///
/// This request is sent synchronously, which means waiting for
/// the result of that invocation on the remote agent.
pub trait HandlerRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  /// The response type for this request.
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  /// The unique identifier for this request. See [`Endpoint`] for more details.
  fn endpoint() -> Endpoint;

  /// Whether this request is synchronous or asynchronous.
  fn request_mode() -> RequestMode {
    RequestMode::Synchronous
  }
}
