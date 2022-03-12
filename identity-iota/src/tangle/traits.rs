// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use crate::did::IotaDID;
use crate::document::ResolvedIotaDocument;
use crate::error::Result;
use crate::tangle::traits::private::Sealed;
use crate::tangle::MessageId;

pub trait TangleRef {
  fn did(&self) -> &IotaDID;

  fn message_id(&self) -> &MessageId;

  fn set_message_id(&mut self, message_id: MessageId);

  fn previous_message_id(&self) -> &MessageId;

  fn set_previous_message_id(&mut self, message_id: MessageId);
}

// TODO: remove TangleResolve with ClientMap refactor?
#[async_trait::async_trait(?Send)]
pub trait TangleResolve {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument>;
}

// Replace by higher-kinded type when `CoerceUnsized` is stabilized, otherwise we cannot
// support unsized types like dynamic traits.
// See: https://github.com/iotaledger/identity.rs/pull/707
/// Sealed trait to generalize over `Arc<T>` and `Rc<T>` for sized types.
pub trait SharedPtr<T>: Clone + From<T> + Deref<Target = T> + Sealed {}

impl<T> SharedPtr<T> for Rc<T> {}

impl<T> SharedPtr<T> for Arc<T> {}

mod private {
  use std::rc::Rc;
  use std::sync::Arc;

  pub trait Sealed {}

  impl<T> Sealed for Rc<T> {}

  impl<T> Sealed for Arc<T> {}
}
