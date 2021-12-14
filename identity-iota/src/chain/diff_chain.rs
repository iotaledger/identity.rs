// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use core::slice::Iter;

use serde;
use serde::Deserialize;
use serde::Serialize;

use crate::chain::IntegrationChain;
use identity_core::convert::FmtJson;

use crate::chain::milestone::sort_by_milestone;
use crate::did::IotaDID;
use crate::diff::DiffMessage;
use crate::document::IotaDocument;
use crate::document::ResolvedIotaDocument;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::Client;
use crate::tangle::Message;
use crate::tangle::MessageExt;
use crate::tangle::MessageId;
use crate::tangle::MessageIdExt;
use crate::tangle::MessageIndex;
use crate::tangle::PublishType;
use crate::tangle::TangleRef;

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

    Ok(Self::try_from_index(integration_chain, index, client).await?)
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
    while let Some(diffs) = index.remove(
      this
        .current_message_id()
        .unwrap_or_else(|| integration_document.message_id()),
    ) {
      // Extract valid diffs.
      let expected_prev_message_id: &MessageId = this
        .current_message_id()
        .unwrap_or_else(|| integration_document.message_id());
      let valid_diffs: Vec<DiffMessage> = diffs
        .into_iter()
        .filter(|diff| Self::check_valid_addition(diff, integration_document, expected_prev_message_id).is_ok())
        .collect();

      // Sort and push the diff referenced by the oldest milestone.
      if let Some(next) = sort_by_milestone(valid_diffs, client).await?.into_iter().next() {
        this.push_unchecked(next); // checked by check_valid_addition above
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

  /// Adds a new diff to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the diff signature is invalid or the Tangle message
  /// references within the diff are invalid.
  pub fn try_push(&mut self, diff: DiffMessage, integration_chain: &IntegrationChain) -> Result<()> {
    let document: &ResolvedIotaDocument = integration_chain.current();
    let expected_prev_message_id: &MessageId = self.current_message_id().unwrap_or_else(|| document.message_id());
    Self::check_valid_addition(&diff, document, expected_prev_message_id)?;
    self.push_unchecked(diff);

    Ok(())
  }

  /// Adds a new diff to the [`DiffChain`] without performing validation checks on the signature or Tangle references
  /// of the [`DiffMessage`].
  fn push_unchecked(&mut self, diff: DiffMessage) {
    self.inner.push(diff);
  }

  /// Checks if the [`DiffMessage`] can be added to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the [`DiffMessage`] is not a valid addition.
  pub fn check_valid_addition(
    diff: &DiffMessage,
    document: &ResolvedIotaDocument,
    expected_prev_message_id: &MessageId,
  ) -> Result<()> {
    if document.document.id() != &diff.id {
      return Err(Error::ChainError { error: "Invalid DID" });
    }

    if diff.message_id().is_null() {
      return Err(Error::ChainError {
        error: "Invalid Message Id",
      });
    }

    if diff.previous_message_id().is_null() {
      return Err(Error::ChainError {
        error: "Invalid Previous Message Id",
      });
    }

    if diff.previous_message_id() != expected_prev_message_id {
      return Err(Error::ChainError {
        error: "Invalid Previous Message Id",
      });
    }

    if document.document.verify_diff(diff).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
      });
    }

    let updated_doc: IotaDocument = diff.merge(&document.document)?;
    if let Some(PublishType::Integration) = PublishType::new(&document.document, &updated_doc) {
      return Err(Error::ChainError {
        error: "diff cannot alter update signing methods",
      });
    }

    Ok(())
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
