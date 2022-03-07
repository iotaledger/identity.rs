// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota_core::did::IotaDID;
use identity_iota_core::message::MessageId;

use crate::document::ResolvedIotaDocument;
use crate::error::Result;

// TODO: remove TangleResolve with ClientMap refactor?
#[async_trait::async_trait(?Send)]
pub trait TangleResolve {
  async fn resolve(&self, did: &IotaDID) -> Result<ResolvedIotaDocument>;
}

pub trait TangleRef {
  fn did(&self) -> &IotaDID;

  fn message_id(&self) -> &MessageId;

  fn set_message_id(&mut self, message_id: MessageId);

  fn previous_message_id(&self) -> &MessageId;

  fn set_previous_message_id(&mut self, message_id: MessageId);
}
