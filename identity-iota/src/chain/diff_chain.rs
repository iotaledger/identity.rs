// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::slice::Iter;

use identity_core::convert::ToJson;

use crate::chain::DocumentChain;
use crate::chain::IntegrationChain;
use crate::did::DocumentDiff;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DiffChain {
  inner: Vec<DocumentDiff>,
}

impl DiffChain {
  /// Constructs a new [`DiffChain`] for the given [`IntegrationChain`] from a slice of [`Messages`][Message].
  pub fn try_from_messages(integration_chain: &IntegrationChain, messages: &[Message]) -> Result<Self> {
    let did: &IotaDID = integration_chain.current().id();

    let index: MessageIndex<DocumentDiff> = messages
      .iter()
      .flat_map(|message| message.try_extract_diff(did))
      .collect();

    debug!("[Diff] Valid Messages = {}/{}", messages.len(), index.len());

    Self::try_from_index(integration_chain, index)
  }

  /// Constructs a new [`DiffChain`] for the given [`IntegrationChain`] from the given [`MessageIndex`].
  pub fn try_from_index(integration_chain: &IntegrationChain, index: MessageIndex<DocumentDiff>) -> Result<Self> {
    trace!("[Diff] Message Index = {:#?}", index);
    Self::try_from_index_with_document(integration_chain.current(), index)
  }

  /// Constructs a new [`DiffChain`] from the given [`MessageIndex`], using an integration document
  /// to validate.
  pub(in crate::chain) fn try_from_index_with_document(
    integration_document: &IotaDocument,
    mut index: MessageIndex<DocumentDiff>,
  ) -> Result<Self> {
    if index.is_empty() {
      return Ok(Self::new());
    }

    let mut this: Self = Self::new();
    while let Some(mut list) = index.remove(
      this
        .current_message_id()
        .unwrap_or_else(|| integration_document.message_id()),
    ) {
      'inner: while let Some(next) = list.pop() {
        if integration_document.verify_data(&next).is_ok() {
          this.inner.push(next);
          break 'inner;
        }
      }
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

  /// Returns an iterator yielding references to [`DocumentDiffs`][DocumentDiff].
  pub fn iter(&self) -> Iter<'_, DocumentDiff> {
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
  pub fn try_push(&mut self, integration_chain: &IntegrationChain, diff: DocumentDiff) -> Result<()> {
    self.check_valid_addition(integration_chain, &diff)?;

    // SAFETY: we performed the necessary validation in `check_validity`.
    unsafe {
      self.push_unchecked(diff);
    }

    Ok(())
  }

  /// Adds a new diff to the [`DiffChain`] with performing validation checks.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check the validity of
  /// the signature or Tangle references of the [`DocumentDiff`].
  pub unsafe fn push_unchecked(&mut self, diff: DocumentDiff) {
    self.inner.push(diff);
  }

  /// Returns `true` if the [`DocumentDiff`] can be added to the [`DiffChain`].
  pub fn is_valid_addition(&self, integration_chain: &IntegrationChain, diff: &DocumentDiff) -> bool {
    self.check_valid_addition(integration_chain, diff).is_ok()
  }

  /// Checks if the [`DocumentDiff`] can be added to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the [`DocumentDiff`] is not a valid addition.
  pub fn check_valid_addition(&self, integration_chain: &IntegrationChain, diff: &DocumentDiff) -> Result<()> {
    let current_document: &IotaDocument = integration_chain.current();
    let expected_prev_message_id: &MessageId = DocumentChain::__diff_message_id(integration_chain, self);
    Self::__check_valid_addition(diff, current_document, expected_prev_message_id)
  }

  /// Validates the [`DocumentDiff`] is signed by the document and may form part of its diff chain.
  pub(in crate::chain) fn __check_valid_addition(
    diff: &DocumentDiff,
    document: &IotaDocument,
    expected_prev_message_id: &MessageId,
  ) -> Result<()> {
    if document.verify_data(diff).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
      });
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

    Ok(())
  }
}

impl Default for DiffChain {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for DiffChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

impl From<DiffChain> for Vec<DocumentDiff> {
  fn from(diff_chain: DiffChain) -> Self {
    diff_chain.inner
  }
}

#[cfg(test)]
mod test {
  use identity_core::common::Timestamp;
  use identity_core::crypto::KeyPair;
  use identity_did::did::CoreDIDUrl;
  use identity_did::did::DID;
  use identity_did::verification::MethodBuilder;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodRef;
  use identity_did::verification::MethodType;

  use crate::chain::DocumentChain;
  use crate::chain::IntegrationChain;
  use crate::did::DocumentDiff;
  use crate::did::IotaDocument;
  use crate::tangle::MessageId;
  use crate::tangle::TangleRef;

  #[test]
  fn test_diff_chain() {
    let mut chain: DocumentChain;
    let mut keys: Vec<KeyPair> = Vec::new();

    // =========================================================================
    // Create Initial Document
    // =========================================================================

    {
      let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
      let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
      document.sign(keypair.private()).unwrap();
      document.set_message_id(MessageId::new([8; 32]));
      chain = DocumentChain::new(IntegrationChain::new(document).unwrap());
      keys.push(keypair);
    }

    assert_eq!(
      chain.current().proof().unwrap().verification_method(),
      "#authentication"
    );

    // =========================================================================
    // Push Integration Chain Update
    // =========================================================================
    {
      let mut new: IotaDocument = chain.current().clone();
      let keypair: KeyPair = KeyPair::new_ed25519().unwrap();

      let authentication: MethodRef = MethodBuilder::default()
        .id(CoreDIDUrl::from(chain.id().to_url().join("#key-2").unwrap()))
        .controller(chain.id().clone().into())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_multibase(keypair.public()))
        .build()
        .map(Into::into)
        .unwrap();

      unsafe {
        new.as_document_mut().authentication_mut().clear();
        new.as_document_mut().authentication_mut().append(authentication.into());
      }

      new.set_updated(Timestamp::now_utc());
      new.set_previous_message_id(*chain.integration_message_id());

      assert!(chain.current().sign_data(&mut new, keys[0].private()).is_ok());
      assert_eq!(
        chain.current().proof().unwrap().verification_method(),
        "#authentication"
      );

      keys.push(keypair);
      assert!(chain.try_push_integration(new).is_ok());
    }

    // =========================================================================
    // Push Diff Chain Update
    // =========================================================================
    {
      let new: IotaDocument = {
        let mut this: IotaDocument = chain.current().clone();
        this.properties_mut().insert("foo".into(), 123.into());
        this.properties_mut().insert("bar".into(), 456.into());
        this.set_updated(Timestamp::now_utc());
        this
      };

      let message_id = *chain.diff_message_id();
      let mut diff: DocumentDiff = chain.current().diff(&new, message_id, keys[1].private()).unwrap();
      diff.set_message_id(message_id);
      assert!(chain.try_push_diff(diff).is_ok());
    }
  }
}
