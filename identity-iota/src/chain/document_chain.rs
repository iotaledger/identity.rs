// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::convert::ToJson;

use crate::chain::DiffChain;
use crate::chain::IntegrationChain;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaDocument;
use crate::error::Result;
use iota_client::bee_message::MessageId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentChain {
  chain_d: DiffChain,
  chain_i: IntegrationChain,
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
    let mut this: IotaDocument = chain_i.current.clone();

    for diff in chain_d.iter() {
      this.merge(diff)?;
    }

    Ok(this)
  }

  /// Creates a new `DocumentChain` from given the `IntegrationChain`.
  pub fn new(chain_i: IntegrationChain) -> Self {
    Self {
      chain_i,
      chain_d: DiffChain::new(),
      document: None,
    }
  }

  /// Creates a new `DocumentChain` from given the `IntegrationChain` and `DiffChain`.
  pub fn with_diff_chain(chain_i: IntegrationChain, chain_d: DiffChain) -> Result<Self> {
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

  /// Returns a reference to the DID identifying the document chain.
  pub fn id(&self) -> &IotaDID {
    self.chain_i.current.id()
  }

  /// Returns a reference to the `IntegrationChain`.
  pub fn integration_chain(&self) -> &IntegrationChain {
    &self.chain_i
  }

  /// Returns a mutable reference to the `IntegrationChain`.
  pub fn integration_chain_mut(&mut self) -> &mut IntegrationChain {
    &mut self.chain_i
  }

  /// Returns a reference to the `DiffChain`.
  pub fn diff(&self) -> &DiffChain {
    &self.chain_d
  }

  /// Returns a mutable reference to the `DiffChain`.
  pub fn diff_mut(&mut self) -> &mut DiffChain {
    &mut self.chain_d
  }

  pub fn fold(mut self) -> Result<IotaDocument> {
    for diff in self.chain_d.iter() {
      self.chain_i.current.merge(diff)?;
    }

    Ok(self.chain_i.current)
  }

  /// Returns a reference to the latest document.
  pub fn current(&self) -> &IotaDocument {
    self.document.as_ref().unwrap_or_else(|| self.chain_i.current())
  }

  /// Returns a mutable reference to the latest document.
  pub fn current_mut(&mut self) -> &mut IotaDocument {
    if let Some(document) = self.document.as_mut() {
      document
    } else {
      self.chain_i.current_mut()
    }
  }

  /// Returns the Tangle message Id of the latest integration document.
  pub fn integration_message_id(&self) -> &MessageId {
    self.chain_i.current_message_id()
  }

  /// Returns the Tangle message Id of the latest diff or integration document.
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

  /// Adds a new diff to the chain.
  ///
  /// # Errors
  ///
  /// Fails if the document diff is invalid.
  pub fn try_push_diff(&mut self, diff: DocumentDiff) -> Result<()> {
    self.chain_d.check_validity(&self.chain_i, &diff)?;

    let mut document: IotaDocument = self.document.take().unwrap_or_else(|| self.chain_i.current().clone());

    document.merge(&diff)?;

    self.document = Some(document);

    // SAFETY: we performed the necessary validation in `DiffChain::check_validity`.
    unsafe {
      self.chain_d.push_unchecked(diff);
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
