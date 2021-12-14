// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use core::slice::Iter;

use serde;
use serde::Deserialize;
use serde::Serialize;

use identity_core::convert::FmtJson;

use crate::chain::milestone::sort_by_milestone;
use crate::chain::IntegrationChain;
use crate::did::IotaDID;
use crate::document::DiffMessage;
use crate::document::IotaDocument;
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
    let did: &IotaDID = integration_chain.current().id();

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
    integration_document: &IotaDocument,
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
    let document: &IotaDocument = integration_chain.current();
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

    let updated_doc: IotaDocument = diff.merge(document)?;
    if let Some(PublishType::Integration) = PublishType::new(document, &updated_doc) {
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

#[cfg(test)]
mod test {
  use identity_core::common::Timestamp;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::PrivateKey;
  use identity_did::did::CoreDIDUrl;
  use identity_did::did::DID;
  use identity_did::verification::MethodBuilder;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodRef;
  use identity_did::verification::MethodRelationship;
  use identity_did::verification::MethodScope;
  use identity_did::verification::MethodType;
  use identity_did::verification::VerificationMethod;

  use crate::chain::DiffChain;
  use crate::chain::DocumentChain;
  use crate::chain::IntegrationChain;
  use crate::did::IotaDIDUrl;
  use crate::document::DiffMessage;
  use crate::document::IotaDocument;
  use crate::document::IotaVerificationMethod;
  use crate::tangle::MessageId;
  use crate::tangle::TangleRef;
  use crate::Error;

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
          chain.current().default_signing_method().unwrap().id(),
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
      let mut diff: DiffMessage = chain
        .current()
        .diff(&new, message_id, keys[1].private(), "#key-2")
        .unwrap();
      diff.set_message_id(message_id);
      assert!(chain.try_push_diff(diff).is_ok());
    }
  }

  #[test]
  fn test_check_valid_addition_rejects_removing_signing_method() {
    // =========================================================================
    // Create Initial Document
    // =========================================================================
    let (document, keypair): (IotaDocument, KeyPair) = create_initial_document();
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(document.clone()).unwrap());

    // =========================================================================
    // Create DiffMessage Removing the Capability Invocation Method
    // =========================================================================
    let mut new_document: IotaDocument = document.clone();
    new_document.properties_mut().insert("foo".into(), 123.into());
    unsafe {
      new_document.as_document_mut().capability_invocation_mut().clear();
    }
    new_document.set_previous_message_id(*chain.integration_message_id());
    new_document.set_updated(Timestamp::now_utc());

    let diff_msg: DiffMessage = create_signed_diff_message(&document, &new_document, &chain, keypair.private());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &document, chain.integration_message_id()).unwrap_err();
    assert!(matches!(
      valid_addition_error,
      Error::ChainError {
        error: "diff cannot alter update signing methods"
      }
    ));
  }

  #[test]
  fn test_check_valid_addition_rejects_adding_signing_method() {
    // =========================================================================
    // Create Initial Document
    // =========================================================================
    let (document, keypair): (IotaDocument, KeyPair) = create_initial_document();
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(document.clone()).unwrap());

    // =========================================================================
    // Create DiffMessage Adding a Capability Invocation Method
    // =========================================================================
    let mut new_document: IotaDocument = document.clone();
    let new_signing_method: IotaVerificationMethod = IotaVerificationMethod::from_did(
      new_document.id().clone(),
      keypair.type_(),
      keypair.public(),
      "new-signing-key",
    )
    .unwrap();
    new_document
      .insert_method(new_signing_method, MethodScope::capability_invocation())
      .unwrap();
    new_document.set_previous_message_id(*chain.integration_message_id());
    new_document.set_updated(Timestamp::now_utc());

    let mut diff_msg: DiffMessage =
      DiffMessage::new(&document, &new_document, *chain.integration_message_id()).unwrap();
    document
      .sign_data(
        &mut diff_msg,
        keypair.private(),
        document.default_signing_method().unwrap().id(),
      )
      .unwrap();
    diff_msg.set_message_id(*chain.diff_message_id());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &document, chain.integration_message_id()).unwrap_err();
    assert!(matches!(
      valid_addition_error,
      Error::ChainError {
        error: "diff cannot alter update signing methods"
      }
    ));
  }

  #[test]
  fn test_check_valid_addition_rejects_altering_signing_method() {
    // =========================================================================
    // Create Initial Document
    // =========================================================================
    let (document, keypair): (IotaDocument, KeyPair) = create_initial_document();
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(document.clone()).unwrap());

    // =========================================================================
    // Create DiffMessage Altering a Capability Invocation Method
    // =========================================================================
    let mut new_document: IotaDocument = document.clone();
    // Replace the public key data.
    unsafe {
      match new_document
        .as_document_mut()
        .capability_invocation_mut()
        .head_mut()
        .unwrap()
      {
        MethodRef::Embed(method) => {
          *method.key_data_mut() = MethodData::new_multibase([3u8; 32]);
        }
        MethodRef::Refer(_) => unreachable!(),
      };
    }
    new_document.set_previous_message_id(*chain.integration_message_id());
    new_document.set_updated(Timestamp::now_utc());

    let diff_msg: DiffMessage = create_signed_diff_message(&document, &new_document, &chain, keypair.private());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &document, chain.integration_message_id()).unwrap_err();
    assert!(matches!(
      valid_addition_error,
      Error::ChainError {
        error: "diff cannot alter update signing methods"
      }
    ));
  }

  #[test]
  fn test_check_valid_addition_rejects_altering_referenced_signing_method() {
    // =========================================================================
    // Create Initial Document
    // =========================================================================
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();

    let signing_method: IotaVerificationMethod = document.default_signing_method().unwrap().clone();
    let signing_method_id: IotaDIDUrl = signing_method.id();
    document.remove_method(signing_method.id()).unwrap();
    document
      .insert_method(signing_method, MethodScope::VerificationMethod)
      .unwrap();
    assert!(document
      .attach_method_relationship(signing_method_id, MethodRelationship::CapabilityInvocation)
      .unwrap());
    document
      .sign_self(keypair.private(), &document.default_signing_method().unwrap().id())
      .unwrap();
    document.set_message_id(MessageId::new([8; 32]));
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(document.clone()).unwrap());

    // =======================================================================================================
    // Create DiffMessage Altering the Verification Method that Has an Attached Capability Invocation Relationship
    // =======================================================================================================
    let mut new_document: IotaDocument = document.clone();
    // Replace the public key data.
    unsafe {
      let updated_method: &mut VerificationMethod = new_document
        .as_document_mut()
        .verification_method_mut()
        .head_mut()
        .unwrap();
      *updated_method.key_data_mut() = MethodData::new_multibase([3u8; 32]);
    }
    new_document.set_previous_message_id(*chain.integration_message_id());
    new_document.set_updated(Timestamp::now_utc());

    let diff_msg: DiffMessage = create_signed_diff_message(&document, &new_document, &chain, keypair.private());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &document, chain.integration_message_id()).unwrap_err();
    assert!(matches!(
      valid_addition_error,
      Error::ChainError {
        error: "diff cannot alter update signing methods"
      }
    ));
  }

  fn create_initial_document() -> (IotaDocument, KeyPair) {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document
      .sign_self(keypair.private(), &document.default_signing_method().unwrap().id())
      .unwrap();
    document.set_message_id(MessageId::new([8; 32]));
    (document, keypair)
  }

  fn create_signed_diff_message(
    current_doc: &IotaDocument,
    updated_doc: &IotaDocument,
    chain: &DocumentChain,
    key: &PrivateKey,
  ) -> DiffMessage {
    let mut diff_msg: DiffMessage =
      DiffMessage::new(&current_doc, &updated_doc, *chain.integration_message_id()).unwrap();
    current_doc
      .sign_data(&mut diff_msg, key, current_doc.default_signing_method().unwrap().id())
      .unwrap();
    diff_msg.set_message_id(*chain.diff_message_id());
    diff_msg
  }
}
