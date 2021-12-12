// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::FmtJson;

use crate::did::IotaDID;
use crate::document::IotaDocument;
use crate::tangle::MessageId;
use crate::tangle::MessageIdExt;
use crate::tangle::TangleRef;

/// An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
/// merged with one or more diff messages.
///
/// NOTE: see [`DocumentChain`](crate::chain::DocumentChain) for how `diff_message_id` is set.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ResolvedIotaDocument {
  #[serde(flatten)]
  pub document: IotaDocument,

  // TODO: combine these fields into `TangleMetadata` / `ResolvedMetadata`?
  /// [`MessageId`] of this integration chain document.
  #[serde(
    rename = "integrationMessageId",
    default = "MessageId::null",
    skip_serializing_if = "MessageIdExt::is_null"
  )]
  pub integration_message_id: MessageId,

  /// [`MessageId`] of the last diff chain message merged into this during resolution, or null.
  ///
  /// See [`DocumentChain`](crate::chain::DocumentChain).
  #[serde(
    rename = "diffMessageId",
    default = "MessageId::null",
    skip_serializing_if = "MessageIdExt::is_null"
  )]
  pub diff_message_id: MessageId,
  // TODO: version_id
}

impl TangleRef for ResolvedIotaDocument {
  fn did(&self) -> &IotaDID {
    self.document.id()
  }

  fn message_id(&self) -> &MessageId {
    &self.integration_message_id
  }

  fn set_message_id(&mut self, message_id: MessageId) {
    self.integration_message_id = message_id;
  }

  fn previous_message_id(&self) -> &MessageId {
    &self.document.metadata.previous_message_id
  }

  fn set_previous_message_id(&mut self, message_id: MessageId) {
    self.document.metadata.previous_message_id = message_id;
  }
}

impl Display for ResolvedIotaDocument {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    self.fmt_json(f)
  }
}

impl From<IotaDocument> for ResolvedIotaDocument {
  fn from(document: IotaDocument) -> Self {
    Self {
      document,
      integration_message_id: MessageId::null(),
      diff_message_id: MessageId::null(),
    }
  }
}
