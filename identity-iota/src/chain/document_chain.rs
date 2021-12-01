// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::FmtJson;

use crate::chain::DiffChain;
use crate::chain::IntegrationChain;
use crate::did::IotaDID;
use crate::document::DiffMessage;
use crate::document::IotaDocument;
use crate::error::Result;
use crate::tangle::MessageId;

/// Holds an [`IntegrationChain`] and its corresponding [`DiffChain`] that can be used to resolve the
/// latest version of an [`IotaDocument`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentChain {
  chain_i: IntegrationChain,
  chain_d: DiffChain,
  #[serde(skip_serializing_if = "Option::is_none")]
  document: Option<IotaDocument>,
}

impl DocumentChain {
  pub(crate) fn __diff_message_id<'a>(chain_i: &'a IntegrationChain, diff: &'a DiffChain) -> &'a MessageId {
    diff
      .current_message_id()
      .unwrap_or_else(|| chain_i.current_message_id())
  }

  pub(crate) fn __fold(chain_i: &IntegrationChain, chain_d: &DiffChain) -> Result<IotaDocument> {
    let mut this: IotaDocument = chain_i.current().clone();

    for diff in chain_d.iter() {
      this.merge(diff)?;
    }

    Ok(this)
  }

  /// Creates a new [`DocumentChain`] from the given [`IntegrationChain`].
  pub fn new(chain_i: IntegrationChain) -> Self {
    Self {
      chain_i,
      chain_d: DiffChain::new(),
      document: None,
    }
  }

  /// Creates a new [`DocumentChain`] from given the [`IntegrationChain`] and [`DiffChain`].
  pub fn new_with_diff_chain(chain_i: IntegrationChain, chain_d: DiffChain) -> Result<Self> {
    let document: Option<IotaDocument> = if chain_d.is_empty() {
      None
    } else {
      Some(Self::__fold(&chain_i, &chain_d)?)
    };

    Ok(Self {
      chain_d,
      chain_i,
      document,
    })
  }

  /// Returns a reference to the [`IotaDID`] identifying this document chain.
  pub fn id(&self) -> &IotaDID {
    self.chain_i.current().id()
  }

  /// Returns a reference to the [`IntegrationChain`].
  pub fn integration_chain(&self) -> &IntegrationChain {
    &self.chain_i
  }

  /// Returns a mutable reference to the [`IntegrationChain`].
  pub fn integration_chain_mut(&mut self) -> &mut IntegrationChain {
    &mut self.chain_i
  }

  /// Returns a reference to the [`DiffChain`].
  pub fn diff(&self) -> &DiffChain {
    &self.chain_d
  }

  /// Returns a mutable reference to the [`DiffChain`].
  pub fn diff_mut(&mut self) -> &mut DiffChain {
    &mut self.chain_d
  }

  /// Merges the changes from the [`DiffChain`] into the current [`IotaDocument`] from
  /// the [`IntegrationChain`].
  pub fn fold(self) -> Result<IotaDocument> {
    Self::__fold(&self.chain_i, &self.chain_d)
  }

  /// Returns a reference to the latest [`IotaDocument`].
  pub fn current(&self) -> &IotaDocument {
    self.document.as_ref().unwrap_or_else(|| self.chain_i.current())
  }

  /// Returns a mutable reference to the latest [`IotaDocument`].
  pub fn current_mut(&mut self) -> &mut IotaDocument {
    if let Some(document) = self.document.as_mut() {
      document
    } else {
      self.chain_i.current_mut()
    }
  }

  /// Returns the Tangle [`MessageId`] of the latest integration [`IotaDocument`].
  pub fn integration_message_id(&self) -> &MessageId {
    self.chain_i.current_message_id()
  }

  /// Returns the Tangle [`MessageId`] of the latest diff or integration [`IotaDocument`].
  pub fn diff_message_id(&self) -> &MessageId {
    Self::__diff_message_id(&self.chain_i, &self.chain_d)
  }

  /// Adds a new integration document to the chain.
  ///
  /// # Errors
  ///
  /// Fails if the document is not a valid integration document.
  pub fn try_push_integration(&mut self, document: IotaDocument) -> Result<()> {
    self.chain_i.try_push(document)?;
    self.chain_d.clear();

    self.document = None;

    Ok(())
  }

  /// Adds a new [`DiffMessage`] to the chain.
  ///
  /// # Errors
  ///
  /// Fails if the diff is invalid.
  pub fn try_push_diff(&mut self, diff: DiffMessage) -> Result<()> {
    // Use the last integration chain document to validate the signature on the diff.
    let integration_document: &IotaDocument = self.chain_i.current();
    let expected_prev_message_id: &MessageId = self.diff_message_id();
    DiffChain::check_valid_addition(&diff, integration_document, expected_prev_message_id)?;

    // Merge the diff into the latest state
    let mut document: IotaDocument = self.document.take().unwrap_or_else(|| self.chain_i.current().clone());
    document.merge(&diff)?;

    // Extend the diff chain and store the merged result.
    self.chain_d.try_push(diff, &self.chain_i)?;
    self.document = Some(document);

    Ok(())
  }
}

impl Display for DocumentChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}
