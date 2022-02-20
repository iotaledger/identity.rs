// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;
use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait ActorRequest: Debug + Serialize + DeserializeOwned + Send + 'static {
  type Response: Debug + Serialize + DeserializeOwned + 'static;

  fn request_name<'cow>(&self) -> Cow<'cow, str>;
}
