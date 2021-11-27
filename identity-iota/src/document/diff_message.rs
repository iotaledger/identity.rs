// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::FromJson;
use identity_core::convert::SerdeInto;
use identity_core::convert::ToJson;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_core::diff::Diff;
use identity_did::diff::DiffDocument;
use identity_did::document::CoreDocument;
use identity_did::verification::MethodUriType;
use identity_did::verification::TryMethod;

use crate::did::IotaDID;
use crate::document::IotaDocument;
use crate::error::Result;
use crate::tangle::MessageId;
use crate::tangle::MessageIdExt;
use crate::tangle::TangleRef;

/// Defines the difference between two DID [`Document`]s' JSON representations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffMessage {
  pub(crate) did: IotaDID,
  pub(crate) diff: String,
  #[serde(
    rename = "previousMessageId",
    default = "MessageId::null",
    skip_serializing_if = "MessageIdExt::is_null"
  )]
  pub(crate) previous_message_id: MessageId,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Signature>,
  #[serde(default = "MessageId::null", skip)]
  pub(crate) message_id: MessageId,
}

impl DiffMessage {
  /// Construct a new `DiffMessage` by diffing the JSON representations of `current` and `updated`.
  ///
  /// The `previous_message_id` is included verbatim in the output, and the `proof` is `None`. To
  /// set a proof, use the `set_signature()` method.
  pub fn new(current: &IotaDocument, updated: &IotaDocument, previous_message_id: MessageId) -> Result<Self> {
    let a: CoreDocument = current.serde_into()?;
    let b: CoreDocument = updated.serde_into()?;
    let diff: String = Diff::diff(&a, &b)?.to_json()?;

    Ok(Self {
      did: current.id().clone(),
      previous_message_id,
      diff,
      proof: None,
      message_id: MessageId::null(),
    })
  }

  /// Returns the DID of associated DID Document.
  pub fn id(&self) -> &IotaDID {
    &self.did
  }

  /// Returns the raw contents of the DID Document diff.
  pub fn diff(&self) -> &str {
    &*self.diff
  }

  /// Returns the Tangle message id of the previous DID Document diff.
  pub fn previous_message_id(&self) -> &MessageId {
    &self.previous_message_id
  }

  /// Returns a reference to the DID Document proof.
  pub fn proof(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }

  /// Returns a new DID Document which is the result of merging `self`
  /// with the given Document.
  pub fn merge(&self, document: &IotaDocument) -> Result<IotaDocument> {
    let data: DiffDocument = DiffDocument::from_json(&self.diff)?;
    let core: CoreDocument = document.serde_into()?;
    let this: CoreDocument = Diff::merge(&core, data)?;

    Ok(this.serde_into()?)
  }
}

impl TangleRef for DiffMessage {
  fn did(&self) -> &IotaDID {
    self.id()
  }

  fn message_id(&self) -> &MessageId {
    &self.message_id
  }

  fn set_message_id(&mut self, message_id: MessageId) {
    self.message_id = message_id;
  }

  fn previous_message_id(&self) -> &MessageId {
    &self.previous_message_id
  }

  fn set_previous_message_id(&mut self, message_id: MessageId) {
    self.previous_message_id = message_id;
  }
}

impl TrySignature for DiffMessage {
  fn signature(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }
}

impl TrySignatureMut for DiffMessage {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }
}

impl SetSignature for DiffMessage {
  fn set_signature(&mut self, value: Signature) {
    self.proof = Some(value);
  }
}

impl TryMethod for DiffMessage {
  const TYPE: MethodUriType = MethodUriType::Relative;
}
