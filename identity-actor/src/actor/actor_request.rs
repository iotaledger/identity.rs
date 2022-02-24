// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestMode {
  Synchronous,
  Asynchronous,
}

// TODO: Use ActorRequest for sync, and ActorMessage for Async, and introduce a supertrait?
/// A request that can be sent to an actor with the expected response being of type `Response`.
///
/// A request can be sync or async. [`RequestMode::Synchronous`] means to invoke the remote handler and wait for
/// the result of that invocation. [`RequestMode::Asynchronous`] means to only wait for an acknowledgement that the
/// request has been received and that a handler exists, but not for the remote handler to finish execution.
///
/// If [`Self::request_mode`] is `Async`, the `Response` field is ignored.
/// Prefer to implement `AsyncActorRequest` for convenience in that case.
pub trait ActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  /// The type of the response that this request returns.
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn request_name<'cow>(&self) -> Cow<'cow, str>;

  fn request_mode(&self) -> RequestMode {
    RequestMode::Synchronous
  }
}

/// An `ActorRequest` whose [`RequestMode`] is `Async`.
pub trait AsyncActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  fn request_name<'cow>(&self) -> Cow<'cow, str>;
}

impl<T: AsyncActorRequest> ActorRequest for T {
  type Response = ();

  fn request_name<'cow>(&self) -> Cow<'cow, str> {
    self.request_name()
  }

  fn request_mode(&self) -> RequestMode {
    RequestMode::Asynchronous
  }
}
