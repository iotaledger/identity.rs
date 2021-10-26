// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;
use std::fmt::Debug;
use std::pin::Pin;

use futures::Future;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Actor;
use crate::RemoteSendError;
use crate::RequestContext;

/// A future whose output is an `Any` trait object.
pub type AnyFuture<'me> = Pin<Box<dyn Future<Output = Box<dyn Any>> + Send + 'me>>;

pub trait RequestHandler: Send + Sync {
  fn invoke(
    &self,
    actor: Actor,
    context: RequestContext<()>,
    object: Box<dyn Any + Send + Sync>,
    request: Box<dyn Any + Send>,
  ) -> Result<AnyFuture<'_>, RemoteSendError>;

  fn object_type_id(&self) -> TypeId;

  fn deserialize_request(&self, input: Vec<u8>) -> Result<Box<dyn Any + Send>, RemoteSendError>;

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Box<dyn Any + Send + Sync>;
}

pub trait ActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn request_name<'cow>(&self) -> Cow<'cow, str>;
}
