// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::actor::Endpoint;
use crate::actor::RequestMode;

/// A message that can be sent to a remote actor without an explicit response.
///
/// This message is sent asynchronously, which means to send the message to the peer without waiting
/// for the completion of the peer's handler. However, an acknowledgment is returned to signal that the
/// handler exists and can be invoked, or an error, if the opposite is true.
pub trait DidCommRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  fn endpoint() -> Endpoint;

  fn request_mode() -> RequestMode {
    RequestMode::Asynchronous
  }
}
