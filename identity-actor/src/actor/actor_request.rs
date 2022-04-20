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

/// A marker to indicate that an [`ActorRequest`] type is handled synchronously.
pub struct Synchronous;
/// A marker to indicate that an [`ActorRequest`] type is handled asynchronously.
pub struct Asynchronous;

/// Used to represent the synchronicity of a request at compile-time.
pub trait SyncMode: private::Sealed {
  fn request_mode() -> RequestMode;
}

impl SyncMode for Synchronous {
  fn request_mode() -> RequestMode {
    RequestMode::Synchronous
  }
}

impl SyncMode for Asynchronous {
  fn request_mode() -> RequestMode {
    RequestMode::Asynchronous
  }
}

mod private {
  pub trait Sealed {}

  impl Sealed for super::Asynchronous {}
  impl Sealed for super::Synchronous {}
}

// A request that can be sent to an actor with the expected response being of type `Response`.
//
// A request can be sync or async. [`Synchronous`] means to invoke the remote handler and wait for
// the result of that invocation. [`Asynchronous`] means to only wait for an acknowledgement that the
// request has been received and that a handler exists, but not for the remote handler to finish execution.
pub trait ActorRequest<T: SyncMode>: Debug + Serialize + DeserializeOwned + Send + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn endpoint() -> Endpoint;

  fn request_mode(&self) -> RequestMode {
    T::request_mode()
  }
}
