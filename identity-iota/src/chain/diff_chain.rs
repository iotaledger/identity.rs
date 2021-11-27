// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::slice::Iter;

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::ToJson;

use crate::chain::IntegrationChain;
use crate::did::IotaDID;
use crate::document::DocumentDiff;
use crate::document::IotaDocument;
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

    log::debug!("[Diff] Valid Messages = {}/{}", messages.len(), index.len());

    Self::try_from_index(integration_chain, index)
  }

  /// Constructs a new [`DiffChain`] for the given [`IntegrationChain`] from the given [`MessageIndex`].
  pub fn try_from_index(integration_chain: &IntegrationChain, index: MessageIndex<DocumentDiff>) -> Result<Self> {
    log::trace!("[Diff] Message Index = {:#?}", index);
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
      'inner: while let Some(next_diff) = list.pop() {
        if this.try_push_inner(next_diff, integration_document).is_ok() {
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
  pub fn try_push(&mut self, diff: DocumentDiff, integration_chain: &IntegrationChain) -> Result<()> {
    let document: &IotaDocument = integration_chain.current();
    self.try_push_inner(diff, document)
  }

  /// Adds a new diff to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the diff signature is invalid or the Tangle message
  /// references within the diff are invalid.
  fn try_push_inner(&mut self, diff: DocumentDiff, document: &IotaDocument) -> Result<()> {
    let expected_prev_message_id: &MessageId = self.current_message_id().unwrap_or_else(|| document.message_id());
    Self::check_valid_addition(&diff, document, expected_prev_message_id)?;

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

  /// Checks if the [`DocumentDiff`] can be added to the [`DiffChain`].
  ///
  /// # Errors
  ///
  /// Fails if the [`DocumentDiff`] is not a valid addition.
  pub fn check_valid_addition(
    diff: &DocumentDiff,
    document: &IotaDocument,
    expected_prev_message_id: &MessageId,
  ) -> Result<()> {
    if document.id() != &diff.did {
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

    if document.verify_diff(diff).is_err() {
      return Err(Error::ChainError {
        error: "Invalid Signature",
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
  use crate::document::DocumentDiff;
  use crate::document::IotaDocument;
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
      document
        .sign_self(keypair.private(), &document.default_signing_method().unwrap().id())
        .unwrap();
      document.set_message_id(MessageId::new([8; 32]));
      chain = DocumentChain::new(IntegrationChain::new(document).unwrap());
      keys.push(keypair);
    }

    assert_eq!(
      chain.current().proof().unwrap().verification_method(),
      format!("#{}", IotaDocument::DEFAULT_METHOD_FRAGMENT)
    );

    // =========================================================================
    // Push Integration Chain Update
    // =========================================================================
    {
      let mut new: IotaDocument = chain.current().clone();
      let keypair: KeyPair = KeyPair::new_ed25519().unwrap();

      // Replace the capability invocation signing key (one step key rotation).
      let signing_method: MethodRef = MethodBuilder::default()
        .id(CoreDIDUrl::from(chain.id().to_url().join("#key-2").unwrap()))
        .controller(chain.id().clone().into())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_multibase(keypair.public()))
        .build()
        .map(Into::into)
        .unwrap();

      unsafe {
        new.as_document_mut().capability_invocation_mut().clear();
        new.as_document_mut().capability_invocation_mut().append(signing_method);
      }

      new.set_updated(Timestamp::now_utc());
      new.set_previous_message_id(*chain.integration_message_id());

      // Sign the update using the old document.
      assert!(chain
        .current()
        .sign_data(
          &mut new,
          keys[0].private(),
          chain.current().default_signing_method().unwrap().id()
        )
        .is_ok());
      assert_eq!(
        chain.current().proof().unwrap().verification_method(),
        format!("#{}", IotaDocument::DEFAULT_METHOD_FRAGMENT)
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

      // Sign using the new key added in the previous integration chain update.
      let message_id = *chain.diff_message_id();
      let mut diff: DocumentDiff = chain
        .current()
        .diff(&new, message_id, keys[1].private(), "#key-2")
        .unwrap();
      diff.set_message_id(message_id);
      assert!(chain.try_push_diff(diff).is_ok());
    }
  }
}
