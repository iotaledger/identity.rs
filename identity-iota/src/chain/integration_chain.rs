// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::mem;

use identity_core::convert::FmtJson;
use identity_iota_core::did::IotaDID;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::tangle::Message;
use identity_iota_core::tangle::MessageId;
use identity_iota_core::tangle::MessageIdExt;
use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::chain::milestone::sort_by_milestone;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::MessageExt;
use crate::tangle::MessageIndex;
use crate::tangle::TangleRef;

/// Primary chain of full [`ResolvedIotaDocuments`](ResolvedIotaDocument) holding the latest version
/// of a DID document and its history.
///
/// See also [`DocumentChain`](crate::chain::DocumentChain) and [`DiffChain`](crate::chain::DiffChain).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegrationChain {
  #[serde(skip_serializing_if = "Option::is_none")]
  history: Option<Vec<ResolvedIotaDocument>>,
  current: ResolvedIotaDocument,
}

impl IntegrationChain {
  /// Constructs a new [`IntegrationChain`] from a slice of [`Message`]s.
  pub async fn try_from_messages(did: &IotaDID, messages: &[Message], client: &Client) -> Result<Self> {
    let index: MessageIndex<ResolvedIotaDocument> = messages
      .iter()
      .flat_map(|message| message.try_extract_document(did))
      .collect();

    log::debug!("[Int] Valid Messages = {}/{}", messages.len(), index.len());

    Self::try_from_index(index, client).await
  }

  /// Constructs a new [`IntegrationChain`] from the given [`MessageIndex`].
  pub async fn try_from_index(mut index: MessageIndex<ResolvedIotaDocument>, client: &Client) -> Result<Self> {
    log::trace!("[Int] Message Index = {:#?}", index);

    // Extract root document.
    let root_document: ResolvedIotaDocument = {
      let valid_root_documents: Vec<ResolvedIotaDocument> = index
        .remove(&MessageId::null())
        .ok_or_else(|| Error::DIDNotFound("DID not found or pruned".to_owned()))?
        .into_iter()
        .filter(|doc| IotaDocument::verify_root_document(&doc.document).is_ok())
        .collect();

      if valid_root_documents.is_empty() {
        return Err(Error::DIDNotFound("no valid root document found".to_owned()));
      }

      let sorted_root_documents: Vec<ResolvedIotaDocument> = sort_by_milestone(valid_root_documents, client).await?;
      sorted_root_documents
        .into_iter()
        .next()
        .ok_or_else(|| Error::DIDNotFound("no root document confirmed by a milestone found".to_owned()))?
    };

    // Construct the rest of the integration chain.
    let mut this: Self = Self::new(root_document)?;
    while let Some(documents) = index.remove(this.current_message_id()) {
      // Extract valid documents.
      let valid_documents: Vec<ResolvedIotaDocument> = documents
        .into_iter()
        .filter(|document| this.check_valid_addition(document).is_ok())
        .collect();

      // Sort and push the one referenced by the oldest milestone.
      if let Some(next) = sort_by_milestone(valid_documents, client).await?.into_iter().next() {
        this.push_unchecked(next); // checked above
      }
      // If no document is appended, the chain ends.
    }
    Ok(this)
  }

  /// Creates a new [`IntegrationChain`] with `current` as the root [`ResolvedIotaDocument`] and no history.
  pub fn new(current: ResolvedIotaDocument) -> Result<Self> {
    if IotaDocument::verify_root_document(&current.document).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Root Document",
      });
    }

    if current.message_id().is_null() {
      return Err(Error::ChainError {
        error: "Invalid Message Id",
      });
    }

    Ok(Self { current, history: None })
  }

  /// Returns a reference to the latest [`ResolvedIotaDocument`].
  pub fn current(&self) -> &ResolvedIotaDocument {
    &self.current
  }

  /// Returns a mutable reference to the latest [`ResolvedIotaDocument`].
  pub fn current_mut(&mut self) -> &mut ResolvedIotaDocument {
    &mut self.current
  }

  /// Returns the Tangle message id of the latest integration [`ResolvedIotaDocument`].
  pub fn current_message_id(&self) -> &MessageId {
    self.current.message_id()
  }

  /// Returns a slice of [`ResolvedIotaDocuments`](ResolvedIotaDocument) in the integration chain, if present.
  /// This excludes the current document.
  pub fn history(&self) -> Option<&[ResolvedIotaDocument]> {
    self.history.as_deref()
  }

  /// Adds a new [`ResolvedIotaDocument`] to this [`IntegrationChain`].
  ///
  /// # Errors
  ///
  /// Fails if the [`ResolvedIotaDocument`] is not a valid addition.
  /// See [`IntegrationChain::check_valid_addition`].
  pub fn try_push(&mut self, document: ResolvedIotaDocument) -> Result<()> {
    self.check_valid_addition(&document)?;
    self.push_unchecked(document);

    Ok(())
  }

  /// Adds a new [`ResolvedIotaDocument`] to this [`IntegrationChain`] without validating it.
  fn push_unchecked(&mut self, document: ResolvedIotaDocument) {
    self
      .history
      .get_or_insert_with(Vec::new)
      .push(mem::replace(&mut self.current, document));
  }

  /// Returns `true` if the [`ResolvedIotaDocument`] can be added to this [`IntegrationChain`].
  ///
  /// See [`IntegrationChain::check_valid_addition`].
  pub fn is_valid_addition(&self, document: &ResolvedIotaDocument) -> bool {
    self.check_valid_addition(document).is_ok()
  }

  /// Checks if the [`ResolvedIotaDocument`] can be added to this [`IntegrationChain`].
  ///
  /// NOTE: the checks here are not exhaustive (e.g. the document `message_id` is not verified to
  /// have been published and contain the same contents on the Tangle).
  ///
  /// # Errors
  ///
  /// Fails if the document signature is invalid or the Tangle message
  /// references within the [`ResolvedIotaDocument`] are invalid.
  pub fn check_valid_addition(&self, document: &ResolvedIotaDocument) -> Result<()> {
    if document.document.id() != self.current.document.id() {
      return Err(Error::ChainError { error: "Invalid DID" });
    }

    if document.message_id().is_null() {
      return Err(Error::ChainError {
        error: "Missing Message Id",
      });
    }

    if document.previous_message_id().is_null() {
      return Err(Error::ChainError {
        error: "Missing Previous Message Id",
      });
    }

    if self.current_message_id() != document.previous_message_id() {
      return Err(Error::ChainError {
        error: "Invalid Previous Message Id",
      });
    }

    // Verify the next document was signed by a valid method from the previous "current" document.
    if self.current.document.verify_document(&document.document).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
      });
    }

    Ok(())
  }
}

impl Display for IntegrationChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

/// Convert an [`IntegrationChain`] into an ordered list of documents with the current document
/// as the last entry.
impl From<IntegrationChain> for Vec<ResolvedIotaDocument> {
  fn from(integration_chain: IntegrationChain) -> Self {
    let mut documents = integration_chain.history.unwrap_or_default();
    documents.push(integration_chain.current);
    documents
  }
}
