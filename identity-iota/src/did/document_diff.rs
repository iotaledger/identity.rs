// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::convert::AsJson;
use identity_core::convert::SerdeInto;
use identity_core::crypto::Signature;
use identity_core::diff::Diff;
use identity_did::diff::DiffDocument;
use identity_did::document::Document;
use identity_did::verifiable::SetSignature;
use identity_did::verifiable::TrySignature;
use identity_did::verifiable::TrySignatureMut;

use crate::client::Client;
use crate::client::Network;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use crate::tangle::MessageId;
use crate::tangle::TangleRef;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DocumentDiff {
  pub(crate) did: IotaDID,
  pub(crate) diff: String,
  pub(crate) previous_message_id: MessageId,
  pub(crate) proof: Option<Signature>,
  #[serde(skip)]
  pub(crate) message_id: MessageId,
}

impl DocumentDiff {
  pub fn new(current: &IotaDocument, updated: &IotaDocument, previous_message_id: MessageId) -> Result<Self> {
    let a: Document = current.serde_into()?;
    let b: Document = updated.serde_into()?;
    let diff: String = Diff::diff(&a, &b)?.to_json()?;

    Ok(Self {
      did: current.id().clone(),
      previous_message_id,
      diff,
      proof: None,
      message_id: MessageId::NONE,
    })
  }

  /// Returns the DID of associated document.
  pub fn id(&self) -> &IotaDID {
    &self.did
  }

  /// Returns the raw contents of the DID document diff.
  pub fn diff(&self) -> &str {
    &*self.diff
  }

  /// Returns the Tangle message id of the previous DID document diff.
  pub fn previous_message_id(&self) -> &MessageId {
    &self.previous_message_id
  }

  /// Returns a reference to the `DocumentDiff` proof.
  pub fn proof(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }

  /// Publishes the `DocumentDiff` to the Tangle using a default `Client`.
  pub async fn publish(&mut self, message_id: &MessageId) -> Result<()> {
    let network: Network = Network::from_name(self.did.network());
    let client: Client = Client::from_network(network)?;

    self.publish_with_client(&client, message_id).await
  }

  /// Publishes the `DocumentDiff` to the Tangle using the provided `Client`.
  pub async fn publish_with_client(&mut self, client: &Client, message_id: &MessageId) -> Result<()> {
    let transaction: _ = client.publish_diff(message_id, self).await?;
    let message_id: String = client.transaction_hash(&transaction);

    self.set_message_id(message_id.into());

    Ok(())
  }

  pub(crate) fn merge(&self, document: &IotaDocument) -> Result<IotaDocument> {
    let data: DiffDocument = DiffDocument::from_json(&self.diff)?;

    document
      .serde_into()
      .and_then(|this: Document| Diff::merge(&this, data).map_err(Into::into))
      .and_then(|this: Document| this.serde_into())
      .map_err(Into::into)
  }
}

impl TangleRef for DocumentDiff {
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

impl TrySignature for DocumentDiff {
  fn signature(&self) -> Option<&Signature> {
    self.proof.as_ref()
  }
}

impl TrySignatureMut for DocumentDiff {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.proof.as_mut()
  }
}

impl SetSignature for DocumentDiff {
  fn set_signature(&mut self, value: Signature) {
    self.proof = Some(value);
  }
}
