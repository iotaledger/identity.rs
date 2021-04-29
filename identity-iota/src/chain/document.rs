// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::convert::ToJson;

use crate::chain::DiffChain;
use crate::chain::IntChain;
use crate::did::Document;
use crate::did::DocumentDiff;
use crate::did::DID;
use crate::error::Result;
use iota::MessageId;

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentChain {
  #[serde(rename = "diff")]
  diff_chain: DiffChain,
  #[serde(rename = "int")]
  int_chain: IntChain,
  #[serde(rename = "latest", skip_serializing_if = "Option::is_none")]
  document: Option<Document>,
}

impl DocumentChain {
  pub(crate) fn __diff_message_id<'a>(int: &'a IntChain, diff: &'a DiffChain) -> &'a MessageId {
    diff.current_message_id().unwrap_or_else(|| int.current_message_id())
  }

  pub(crate) fn __fold(int_chain: &IntChain, diff_chain: &DiffChain) -> Result<Document> {
    let mut this: Document = int_chain.current.clone();

    for diff in diff_chain.iter() {
      this.merge(diff)?;
    }

    Ok(this)
  }

  /// Creates a new `DocumentChain` from given the `IntChain`.
  pub fn new(int_chain: IntChain) -> Self {
    Self {
      int_chain,
      diff_chain: DiffChain::new(),
      document: None,
    }
  }

  /// Creates a new `DocumentChain` from given the `IntChain` and `DiffChain`.
  pub fn with_diff_chain(int_chain: IntChain, diff_chain: DiffChain) -> Result<Self> {
    let document: Option<Document> = if diff_chain.is_empty() {
      None
    } else {
      Some(Self::__fold(&int_chain, &diff_chain)?)
    };

    Ok(Self {
      int_chain,
      diff_chain,
      document,
    })
  }

  /// Returns a reference to the DID identifying the document chain.
  pub fn id(&self) -> &DID {
    self.int_chain.current.id()
  }

  /// Returns a reference to the `IntChain`.
  pub fn int_chain(&self) -> &IntChain {
    &self.int_chain
  }

  /// Returns a mutable reference to the `IntChain`.
  pub fn int_chain_mut(&mut self) -> &mut IntChain {
    &mut self.int_chain
  }

  /// Returns a reference to the `DiffChain`.
  pub fn diff(&self) -> &DiffChain {
    &self.diff_chain
  }

  /// Returns a mutable reference to the `DiffChain`.
  pub fn diff_mut(&mut self) -> &mut DiffChain {
    &mut self.diff_chain
  }

  pub fn fold(mut self) -> Result<Document> {
    for diff in self.diff_chain.iter() {
      self.int_chain.current.merge(diff)?;
    }

    Ok(self.int_chain.current)
  }

  /// Returns a reference to the latest document.
  pub fn current(&self) -> &Document {
    self.document.as_ref().unwrap_or_else(|| self.int_chain.current())
  }

  /// Returns a mutable reference to the latest document.
  pub fn current_mut(&mut self) -> &mut Document {
    if let Some(document) = self.document.as_mut() {
      document
    } else {
      self.int_chain.current_mut()
    }
  }

  /// Returns the Tangle message Id of the latest auth document.
  pub fn auth_message_id(&self) -> &MessageId {
    self.int_chain.current_message_id()
  }

  /// Returns the Tangle message Id of the latest diff or int document.
  pub fn diff_message_id(&self) -> &MessageId {
    Self::__diff_message_id(&self.int_chain, &self.diff_chain)
  }

  /// Adds a new auth document to the chain.
  ///
  /// # Errors
  ///
  /// Fails if the document is not a valid auth document.
  pub fn try_push_auth(&mut self, document: Document) -> Result<()> {
    self.int_chain.try_push(document)?;
    self.diff_chain.clear();

    self.document = None;

    Ok(())
  }

  /// Adds a new diff to the chain.
  ///
  /// # Errors
  ///
  /// Fails if the document diff is invalid.
  pub fn try_push_diff(&mut self, diff: DocumentDiff) -> Result<()> {
    self.diff_chain.check_validity(&self.int_chain, &diff)?;

    let mut document: Document = self.document.take().unwrap_or_else(|| self.int_chain.current().clone());

    document.merge(&diff)?;

    self.document = Some(document);

    // SAFETY: we performed the necessary validation in `DiffChain::check_validity`.
    unsafe {
      self.diff_chain.push_unchecked(diff);
    }

    Ok(())
  }
}

impl Display for DocumentChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}
