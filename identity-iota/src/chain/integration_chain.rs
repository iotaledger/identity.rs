// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::mem;

use identity_core::convert::ToJson;

use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Error;
use crate::error::Result;
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
  pub fn try_from_messages(did: &IotaDID, messages: &[Message]) -> Result<Self> {
    let index: MessageIndex<IotaDocument> = messages
      .iter()
      .flat_map(|message| message.try_extract_document(did))
      .collect();

    debug!("[Int] Valid Messages = {}/{}", messages.len(), index.len());

    Self::try_from_index(index)
  }

  /// Constructs a new [`IntegrationChain`] from the given [`MessageIndex`].
  pub fn try_from_index(mut index: MessageIndex<IotaDocument>) -> Result<Self> {
    trace!("[Int] Message Index = {:#?}", index);

    let current: IotaDocument = index
      .remove_where(&MessageId::null(), |doc| doc.verify().is_ok())
      .ok_or(Error::ChainError {
        error: "Invalid Root Document",
      })?;

    let mut this: Self = Self::new(current)?;

    while let Some(mut list) = index.remove(this.current_message_id()) {
      'inner: while let Some(document) = list.pop() {
        if this.try_push(document).is_ok() {
          break 'inner;
        }
      }
    }

    Ok(this)
  }

  /// Creates a new [`IntegrationChain`] with the given [`IotaDocument`] as the latest and no
  /// history.
  pub fn new(current: IotaDocument) -> Result<Self> {
    if current.verify().is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
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

    self
      .history
      .get_or_insert_with(Vec::new)
      .push(mem::replace(&mut self.current, document));

    Ok(())
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
    if self.current.verify_data(document).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
      });
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

    Ok(())
  }
}

impl Display for IntegrationChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
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
