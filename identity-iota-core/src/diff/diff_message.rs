// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::crypto::GetSignature;
use identity_core::crypto::GetSignatureMut;
use identity_core::crypto::Proof;
use identity_core::crypto::SetSignature;
use identity_core::diff::Diff;
use identity_did::verification::MethodUriType;
use identity_did::verification::TryMethod;

use crate::did::IotaDID;
use crate::diff::DiffIotaDocument;
use crate::document::IotaDocument;
use crate::error::Result;
use crate::tangle::MessageId;
use crate::tangle::MessageIdExt;

/// Defines the difference between two [`IotaDocument`] JSON representations, published to
/// the Tangle on a differentiation chain index.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DiffMessage {
  pub(crate) id: IotaDID,
  pub(crate) diff: DiffIotaDocument,
  #[serde(
    rename = "previousMessageId",
    default = "MessageId::null",
    skip_serializing_if = "MessageId::is_null"
  )]
  pub(crate) previous_message_id: MessageId,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) proof: Option<Proof>,
  #[serde(default = "MessageId::null", skip)]
  pub(crate) message_id: MessageId,
}

impl DiffMessage {
  /// Construct a new `DiffMessage` by diffing the JSON representations of `current` and `updated`.
  ///
  /// The `previous_message_id` is included verbatim in the output, and the `proof` is `None`. To
  /// set a proof, use the `set_signature()` method.
  pub fn new(current: &IotaDocument, updated: &IotaDocument, previous_message_id: MessageId) -> Result<Self> {
    let diff: DiffIotaDocument = <IotaDocument as Diff>::diff(current, updated)?;

    Ok(Self {
      id: current.id().clone(),
      previous_message_id,
      diff,
      proof: None,
      message_id: MessageId::null(),
    })
  }

  /// Returns the DID of associated DID Document.
  pub fn id(&self) -> &IotaDID {
    &self.id
  }

  /// Returns the raw contents of the DID Document diff.
  pub fn diff(&self) -> &DiffIotaDocument {
    &self.diff
  }

  /// Returns the DID of associated DID Document.
  pub fn message_id(&self) -> &MessageId {
    &self.message_id
  }

  /// Sets the DID of the associated DID Document.
  pub fn set_message_id(&mut self, message_id: MessageId) {
    self.message_id = message_id;
  }

  /// Returns the Tangle message id of the previous DID Document diff.
  pub fn previous_message_id(&self) -> &MessageId {
    &self.previous_message_id
  }

  /// Sets the Tangle message id of the previous DID Document diff.
  pub fn set_previous_message_id(&mut self, message_id: MessageId) {
    self.previous_message_id = message_id;
  }

  /// Returns a reference to the proof.
  pub fn proof(&self) -> Option<&Proof> {
    self.proof.as_ref()
  }

  /// Returns a new DID Document which is the result of merging `self`
  /// with the given Document.
  pub fn merge(&self, document: &IotaDocument) -> Result<IotaDocument> {
    let merged: IotaDocument = Diff::merge(document, self.diff.clone())?;
    Ok(merged)
  }
}

impl GetSignature for DiffMessage {
  fn signature(&self) -> Option<&Proof> {
    self.proof.as_ref()
  }
}

impl GetSignatureMut for DiffMessage {
  fn signature_mut(&mut self) -> Option<&mut Proof> {
    self.proof.as_mut()
  }
}

impl SetSignature for DiffMessage {
  fn set_signature(&mut self, value: Proof) {
    self.proof = Some(value);
  }
}

impl TryMethod for DiffMessage {
  const TYPE: MethodUriType = MethodUriType::Relative;
}
