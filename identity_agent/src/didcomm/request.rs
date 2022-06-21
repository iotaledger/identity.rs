// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::agent::Endpoint;
use crate::agent::RequestMode;

/// A message that can be sent to a remote actor without an explicit response.
///
/// This request is sent asynchronously, which means sending the request without waiting for
/// the result of that invocation on the remote agent.
/// However, an acknowledgment is returned to signal that an
/// appropriate actor exists that can handle the request, or an error, if the opposite is true.
pub trait DidCommRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  /// The unique identifier for this request. See [`Endpoint`] for more details.
  fn endpoint() -> Endpoint;

  /// Whether this request is synchronous or asynchronous.
  fn request_mode() -> RequestMode {
    RequestMode::Asynchronous
  }
}
