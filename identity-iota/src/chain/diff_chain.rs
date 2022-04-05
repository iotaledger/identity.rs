// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::slice::Iter;

use identity_core::convert::FmtJson;
use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::tangle::Message;
use identity_iota_core::tangle::MessageId;
use identity_iota_core::tangle::MessageIdExt;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::chain::milestone::sort_by_milestone;
use crate::chain::IntegrationChain;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::MessageExt;
use crate::tangle::MessageIndex;
use crate::tangle::PublishType;
use crate::tangle::TangleRef;

#[deprecated(since = "0.5.0", note = "diff chain features are slated for removal")]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DiffChain {
  inner: Vec<DiffMessage>,
}

impl DiffChain {
  /// Constructs a new [`DiffChain`] for the given [`IntegrationChain`] from a slice of [`Messages`][Message].
  pub async fn try_from_messages(
    integration_chain: &IntegrationChain,
    messages: &[Message],
    client: &Client,
  ) -> Result<Self> {
    let did: &IotaDID = integration_chain.current().document.id();

    let index: MessageIndex<DiffMessage> = messages
      .iter()
      .flat_map(|message| message.try_extract_diff(did))
      .collect();

    log::debug!("[Diff] Valid Messages = {}/{}", messages.len(), index.len());

    Self::try_from_index(integration_chain, index, client).await
  }

  /// Constructs a new [`DiffChain`] for the given [`IntegrationChain`] from the given [`MessageIndex`].
  pub async fn try_from_index(
    integration_chain: &IntegrationChain,
    index: MessageIndex<DiffMessage>,
    client: &Client,
  ) -> Result<Self> {
    log::trace!("[Diff] Message Index = {:#?}", index);
    Self::try_from_index_with_document(integration_chain.current(), index, client).await
  }

  /// Constructs a new [`DiffChain`] from the given [`MessageIndex`], using an integration document
  /// to validate.
  pub(in crate::chain) async fn try_from_index_with_document(
    integration_document: &ResolvedIotaDocument,
    mut index: MessageIndex<DiffMessage>,
    client: &Client,
  ) -> Result<Self> {
    if index.is_empty() {
      return Ok(Self::new());
    }

    let mut this: Self = Self::new();
    let mut current_document: ResolvedIotaDocument = integration_document.clone();
    while let Some(diffs) = index.remove(
      this
        .current_message_id()
        .unwrap_or_else(|| current_document.message_id()),
    ) {
      // Extract diffs that reference the last message (either the integration message or the
      // diff message from the previous iteration). If more than one references the
      // same message, they are conflicting.
      let expected_prev_message_id: &MessageId = this
        .current_message_id()
        .unwrap_or_else(|| current_document.message_id());
      // Filter out diffs with invalid signatures.
      let valid_diffs: Vec<DiffMessage> = diffs
        .into_iter()
        .filter(|diff| Self::verify_diff(diff, integration_document, expected_prev_message_id).is_ok())
        .collect();

      // Sort and apply the diff referenced by the oldest milestone.
      if let Some((diff, merged_document)) = sort_by_milestone(valid_diffs, client)
        .await?
        .into_iter()
        .filter_map(|diff|
          // Try merge the diff changes into the document. Important to prevent changes to the
          // signing verification methods and reject invalid changes to non-existent sections.
          match Self::try_merge(&diff, &current_document) {
            Ok(merged_document) => Some((diff, merged_document)),
            _ => None,
          })
        .next()
      {
        // Update the document for the next diff to allow updating sections added by previous diffs.
        current_document = merged_document;
        // Checked by verify_diff and try_merge above.
        this.push_unchecked(diff);
      }
      // If no diff is appended, the chain ends.
    }

    Ok(this)
  }

  /// Creates a new [`DiffChain`].
  pub fn new() -> Self {
    Self { inner: Vec::new() }
  }

  /// Returns the total number of diffs.
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  /// Returns `true` if the [`DiffChain`] is empty.
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// Empties the [`DiffChain`], removing all diffs.
  pub fn clear(&mut self) {
    self.inner.clear();
  }

  /// Returns an iterator yielding references to [`DiffMessages`][DiffMessage].
  pub fn iter(&self) -> Iter<'_, DiffMessage> {
    self.inner.iter()
  }

  /// Returns the [`MessageId`] of the latest diff in the chain, if any.
  pub fn current_message_id(&self) -> Option<&MessageId> {
    self.inner.last().map(|diff| diff.message_id())
  }

  /// Adds a new diff to the [`DiffChain`] while merging it with the given integration document.
  /// Returns the merged document if valid.
  ///
  /// # Errors
  ///
  /// Fails if the diff signature is invalid, the Tangle message
  /// references within the diff are invalid, or the merge fails.
  pub fn try_push_and_merge(
    &mut self,
    diff: DiffMessage,
    integration_document: &ResolvedIotaDocument,
  ) -> Result<ResolvedIotaDocument> {
    let expected_prev_message_id: &MessageId = self
      .current_message_id()
      .unwrap_or_else(|| integration_document.message_id());
    let updated_document: ResolvedIotaDocument =
      Self::check_valid_addition(&diff, integration_document, expected_prev_message_id)?;
    self.push_unchecked(diff);

    Ok(updated_document)
  }

  /// Adds a new diff to the [`DiffChain`] without performing any validation checks on
  /// the [`DiffMessage`].
  fn push_unchecked(&mut self, diff: DiffMessage) {
    self.inner.push(diff);
  }

  /// Checks whether the [`DiffMessage`] attributes and signature are valid.
  ///
  /// NOTE: does not verify the changes contained in the diff are valid.
  /// See [`DiffChain::try_merge`].
  pub fn verify_diff(
    diff: &DiffMessage,
    document: &ResolvedIotaDocument,
    expected_prev_message_id: &MessageId,
  ) -> Result<()> {
    if document.document.id() != diff.id() {
      return Err(Error::ChainError { error: "invalid DID" });
    }

    if diff.message_id().is_null() {
      return Err(Error::ChainError {
        error: "invalid message id",
      });
    }

    if diff.previous_message_id().is_null() {
      return Err(Error::ChainError {
        error: "invalid previous message id",
      });
    }

    if diff.previous_message_id() != expected_prev_message_id {
      return Err(Error::ChainError {
        error: "invalid previous message id",
      });
    }

    if document.document.verify_diff(diff).is_err() {
      return Err(Error::ChainError {
        error: "invalid diff signature",
      });
    }

    Ok(())
  }

  /// Attempts to merge the [`DiffMessage`] changes into the given document and returns the
  /// resulting [`ResolvedIotaDocument`] if valid.
  ///
  /// NOTE: does not verify the signature and attributes.
  /// See [`DiffChain::verify_diff`].
  pub fn try_merge(diff: &DiffMessage, document: &ResolvedIotaDocument) -> Result<ResolvedIotaDocument> {
    let mut updated_document: ResolvedIotaDocument = document.clone();
    updated_document.merge_diff_message(diff)?;
    if let Some(PublishType::Integration) = PublishType::new(&document.document, &updated_document.document) {
      return Err(Error::ChainError {
        error: "diff cannot alter update signing methods",
      });
    }

    Ok(updated_document)
  }

  /// Checks if the [`DiffMessage`] can be added to the [`DiffChain`]. Returns the merged
  /// document if valid.
  ///
  /// Equivalent to calling [`DiffChain::verify_diff`] then [`DiffChain::verify_merge`].
  ///
  /// # Errors
  ///
  /// Fails if the [`DiffMessage`] is not a valid addition.
  pub fn check_valid_addition(
    diff: &DiffMessage,
    document: &ResolvedIotaDocument,
    expected_prev_message_id: &MessageId,
  ) -> Result<ResolvedIotaDocument> {
    Self::verify_diff(diff, document, expected_prev_message_id)?;
    Self::try_merge(diff, document)
  }
}

impl Default for DiffChain {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for DiffChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

impl From<DiffChain> for Vec<DiffMessage> {
  fn from(diff_chain: DiffChain) -> Self {
    diff_chain.inner
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_core::json;
  use identity_did::did::DID;
  use identity_did::service::Service;
  use identity_iota_core::diff::DiffMessage;
  use identity_iota_core::document::IotaDocument;
  use identity_iota_core::document::IotaService;
  use identity_iota_core::tangle::MessageId;

  use crate::document::ResolvedIotaDocument;
  use crate::tangle::ClientBuilder;
  use crate::tangle::MessageIndex;
  use crate::tangle::TangleRef;

  use super::*;

  fn create_document() -> (ResolvedIotaDocument, KeyPair) {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document
      .sign_self(
        keypair.private(),
        document.default_signing_method().unwrap().id().clone(),
      )
      .unwrap();
    let mut resolved: ResolvedIotaDocument = ResolvedIotaDocument::from(document);
    resolved.set_message_id(MessageId::new([1; 32]));
    (resolved, keypair)
  }

  #[tokio::test]
  async fn test_diff_chain_add_remove_service() {
    let (original, keypair) = create_document();

    // Add a new service in a diff update.
    let mut updated1: IotaDocument = original.document.clone();
    let service: IotaService = Service::from_json_value(json!({
      "id": updated1.id().to_url().join("#linked-domain").unwrap(),
      "type": "LinkedDomains",
      "serviceEndpoint": "https://iota.org"
    }))
    .unwrap();
    assert!(updated1.insert_service(service));
    let mut diff_add: DiffMessage = original
      .document
      .diff(
        &updated1,
        original.integration_message_id,
        keypair.private(),
        original.document.default_signing_method().unwrap().id(),
      )
      .unwrap();
    diff_add.set_message_id(MessageId::new([2; 32]));

    // Remove the same service in a diff update.
    let mut updated2: IotaDocument = updated1.clone();
    assert!(updated2
      .remove_service(&updated1.id().to_url().join("#linked-domain").unwrap())
      .is_ok());
    let mut diff_delete: DiffMessage = updated1
      .diff(
        &updated2,
        *diff_add.message_id(),
        keypair.private(),
        updated1.default_signing_method().unwrap().id(),
      )
      .unwrap();
    diff_delete.set_message_id(MessageId::new([3; 32]));

    // Ensure both diffs are resolved by the DiffChain.
    let mut message_index: MessageIndex<DiffMessage> = MessageIndex::new();
    message_index.insert(diff_add.clone());
    message_index.insert(diff_delete.clone());
    let client: Client = ClientBuilder::new().node_sync_disabled().build().await.unwrap();
    let diff_chain: DiffChain = DiffChain::try_from_index_with_document(&original, message_index, &client)
      .await
      .unwrap();
    assert_eq!(diff_chain.len(), 2);
    assert_eq!(diff_chain.inner.get(0), Some(&diff_add));
    assert_eq!(diff_chain.inner.get(1), Some(&diff_delete));

    // Ensure the merged result does not have the service.
    let mut merged: ResolvedIotaDocument = original.clone();
    for diff in diff_chain.iter() {
      merged.merge_diff_message(diff).unwrap();
    }
    assert!(merged.document.service().is_empty());
  }

  #[tokio::test]
  async fn test_diff_chain_add_edit_service() {
    let (original, keypair) = create_document();

    // Add a new service in a diff update.
    let mut updated1: IotaDocument = original.document.clone();
    let service: IotaService = Service::from_json_value(json!({
      "id": updated1.id().to_url().join("#linked-domain").unwrap(),
      "type": "LinkedDomains",
      "serviceEndpoint": "https://iota.org"
    }))
    .unwrap();
    assert!(updated1.insert_service(service.clone()));
    let mut diff_add: DiffMessage = original
      .document
      .diff(
        &updated1,
        original.integration_message_id,
        keypair.private(),
        original.document.default_signing_method().unwrap().id(),
      )
      .unwrap();
    diff_add.set_message_id(MessageId::new([2; 32]));

    // Edit the same service in a diff update.
    let mut updated2: IotaDocument = updated1.clone();
    let service_updated: IotaService = Service::from_json_value(json!({
      "id": updated1.id().to_url().join("#linked-domain").unwrap(),
      "type": "LinkedDomains",
      "serviceEndpoint": ["https://example.com", "https://example.org"],
    }))
    .unwrap();
    updated2
      .core_document_mut()
      .service_mut()
      .replace(&service, service_updated.clone());
    let mut diff_edit: DiffMessage = updated1
      .diff(
        &updated2,
        *diff_add.message_id(),
        keypair.private(),
        updated1.default_signing_method().unwrap().id(),
      )
      .unwrap();
    diff_edit.set_message_id(MessageId::new([3; 32]));

    // Ensure both diffs are resolved by the DiffChain.
    let mut message_index: MessageIndex<DiffMessage> = MessageIndex::new();
    message_index.insert(diff_add.clone());
    message_index.insert(diff_edit.clone());
    let client: Client = ClientBuilder::new().node_sync_disabled().build().await.unwrap();
    let diff_chain: DiffChain = DiffChain::try_from_index_with_document(&original, message_index, &client)
      .await
      .unwrap();
    assert_eq!(diff_chain.len(), 2);
    assert_eq!(diff_chain.inner.get(0), Some(&diff_add));
    assert_eq!(diff_chain.inner.get(1), Some(&diff_edit));

    // Ensure the merged result has the updated service.
    let mut merged: ResolvedIotaDocument = original.clone();
    for diff in diff_chain.iter() {
      merged.merge_diff_message(diff).unwrap();
    }
    assert_ne!(merged.document.service().first().unwrap(), &service);
    assert_eq!(merged.document.service().first().unwrap(), &service_updated);
  }
}
