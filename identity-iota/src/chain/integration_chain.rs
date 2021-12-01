// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use core::mem;

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::FmtJson;

use crate::chain::milestone::sort_by_milestone;
use crate::did::IotaDID;
use crate::document::IotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::Message;
use crate::tangle::MessageExt;
use crate::tangle::MessageId;
use crate::tangle::MessageIdExt;
use crate::tangle::MessageIndex;
use crate::tangle::TangleRef;

/// Primary chain of full [`IotaDocuments`](IotaDocument) holding the latest document and its
/// history.
///
/// See also [`DiffChain`](crate::chain::DiffChain)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegrationChain {
  #[serde(skip_serializing_if = "Option::is_none")]
  history: Option<Vec<IotaDocument>>,
  current: IotaDocument,
}

impl IntegrationChain {
  /// Constructs a new [`IntegrationChain`] from a slice of [`Message`]s.
  pub async fn try_from_messages(did: &IotaDID, messages: &[Message], client: &Client) -> Result<Self> {
    let index: MessageIndex<IotaDocument> = messages
      .iter()
      .flat_map(|message| message.try_extract_document(did))
      .collect();

    log::debug!("[Int] Valid Messages = {}/{}", messages.len(), index.len());

    Self::try_from_index(index, client).await
  }

  /// Constructs a new [`IntegrationChain`] from the given [`MessageIndex`].
  pub async fn try_from_index(mut index: MessageIndex<IotaDocument>, client: &Client) -> Result<Self> {
    log::trace!("[Int] Message Index = {:#?}", index);

    // Extract root document.
    let root_document: IotaDocument = {
      let valid_root_documents: Vec<IotaDocument> = index
        .remove(&MessageId::null())
        .ok_or_else(|| Error::DIDNotFound("DID not found or pruned".to_owned()))?
        .into_iter()
        .filter(|doc| IotaDocument::verify_root_document(doc).is_ok())
        .collect();

      if valid_root_documents.is_empty() {
        return Err(Error::DIDNotFound("no valid root document found".to_owned()));
      }

      let sorted_root_documents: Vec<IotaDocument> = sort_by_milestone(valid_root_documents, client).await?;
      sorted_root_documents
        .into_iter()
        .next()
        .ok_or_else(|| Error::DIDNotFound("no root document confirmed by a milestone found".to_owned()))?
    };

    // Construct the rest of the integration chain.
    let mut this: Self = Self::new(root_document)?;
    while let Some(documents) = index.remove(this.current_message_id()) {
      // Extract valid documents.
      let valid_documents: Vec<IotaDocument> = documents
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

  /// Creates a new [`IntegrationChain`] with `current` as the root [`IotaDocument`] and no history.
  pub fn new(current: IotaDocument) -> Result<Self> {
    if IotaDocument::verify_root_document(&current).is_err() {
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

  /// Returns a reference to the latest [`IotaDocument`].
  pub fn current(&self) -> &IotaDocument {
    &self.current
  }

  /// Returns a mutable reference to the latest [`IotaDocument`].
  pub fn current_mut(&mut self) -> &mut IotaDocument {
    &mut self.current
  }

  /// Returns the Tangle message id of the latest integration [`IotaDocument`].
  pub fn current_message_id(&self) -> &MessageId {
    self.current.message_id()
  }

  /// Returns a slice of [`IotaDocuments`](IotaDocument) in the integration chain, if present.
  /// This excludes the current document.
  pub fn history(&self) -> Option<&[IotaDocument]> {
    self.history.as_deref()
  }

  /// Adds a new [`IotaDocument`] to this [`IntegrationChain`].
  ///
  /// # Errors
  ///
  /// Fails if the [`IotaDocument`] is not a valid addition.
  /// See [`IntegrationChain::check_valid_addition`].
  pub fn try_push(&mut self, document: IotaDocument) -> Result<()> {
    self.check_valid_addition(&document)?;
    self.push_unchecked(document);

    Ok(())
  }

  /// Adds a new [`IotaDocument`] to this [`IntegrationChain`] without validating it.
  fn push_unchecked(&mut self, document: IotaDocument) {
    self
      .history
      .get_or_insert_with(Vec::new)
      .push(mem::replace(&mut self.current, document));
  }

  /// Returns `true` if the [`IotaDocument`] can be added to this [`IntegrationChain`].
  ///
  /// See [`IntegrationChain::check_valid_addition`].
  pub fn is_valid_addition(&self, document: &IotaDocument) -> bool {
    self.check_valid_addition(document).is_ok()
  }

  /// Checks if the [`IotaDocument`] can be added to this [`IntegrationChain`].
  ///
  /// NOTE: the checks here are not exhaustive (e.g. the document `message_id` is not verified to
  /// have been published and contain the same contents on the Tangle).
  ///
  /// # Errors
  ///
  /// Fails if the document signature is invalid or the Tangle message
  /// references within the [`IotaDocument`] are invalid.
  pub fn check_valid_addition(&self, document: &IotaDocument) -> Result<()> {
    if document.id() != self.current.id() {
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

    // Verify the next document was signed by a valid method from the previous document.
    if IotaDocument::verify_document(document, &self.current).is_err() {
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
impl From<IntegrationChain> for Vec<IotaDocument> {
  fn from(integration_chain: IntegrationChain) -> Self {
    let mut documents = integration_chain.history.unwrap_or_default();
    documents.push(integration_chain.current);
    documents
  }
}
