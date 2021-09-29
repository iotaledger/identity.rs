// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
  any::{Any, TypeId},
  borrow::Cow,
  fmt::Debug,
  pin::Pin,
};

use futures::Future;
use serde::{de::DeserializeOwned, Serialize};

pub trait RequestHandler: Send + Sync {
  fn invoke<'this>(
    &'this self,
    object: Box<dyn Any + Send + Sync>,
    input: Vec<u8>,
  ) -> Pin<Box<dyn Future<Output = Vec<u8>> + Send + 'this>>;

  fn object_type_id(&self) -> TypeId;

  fn clone_object(&self, object: &Box<dyn Any + Send + Sync>) -> Box<dyn Any + Send + Sync>;
}

pub trait ActorRequest: Debug + Serialize + DeserializeOwned + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn request_name<'cow>(&self) -> Cow<'cow, str>;
}
