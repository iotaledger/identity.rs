// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use identity_core::convert::FmtJson;
use identity_iota_core::did::IotaDID;
use identity_iota_core::diff::DiffMessage;
use identity_iota_core::tangle::MessageId;
use serde::Deserialize;
use serde::Serialize;

use crate::chain::DiffChain;
use crate::chain::IntegrationChain;
use crate::document::ResolvedIotaDocument;
use crate::error::Result;

/// Holds an [`IntegrationChain`] and its corresponding [`DiffChain`] that can be used to resolve the
/// latest version of a [`ResolvedIotaDocument`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DocumentChain {
  chain_i: IntegrationChain,
  chain_d: DiffChain,
  #[serde(skip_serializing_if = "Option::is_none")]
  document: Option<ResolvedIotaDocument>,
}

impl DocumentChain {
  pub(crate) fn __diff_message_id<'a>(chain_i: &'a IntegrationChain, diff: &'a DiffChain) -> &'a MessageId {
    diff
      .current_message_id()
      .unwrap_or_else(|| chain_i.current_message_id())
  }

  pub(crate) fn __fold(chain_i: &IntegrationChain, chain_d: &DiffChain) -> Result<ResolvedIotaDocument> {
    let mut document: ResolvedIotaDocument = chain_i.current().clone();

    for diff_message in chain_d.iter() {
      document.merge_diff_message(diff_message)?;
    }

    Ok(document)
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
    let document: Option<ResolvedIotaDocument> = if chain_d.is_empty() {
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
    self.chain_i.current().document.id()
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
  pub fn fold(self) -> Result<ResolvedIotaDocument> {
    Self::__fold(&self.chain_i, &self.chain_d)
  }

  /// Returns a reference to the latest [`ResolvedIotaDocument`].
  pub fn current(&self) -> &ResolvedIotaDocument {
    self.document.as_ref().unwrap_or_else(|| self.chain_i.current())
  }

  /// Returns a mutable reference to the latest [`ResolvedIotaDocument`].
  pub fn current_mut(&mut self) -> &mut ResolvedIotaDocument {
    self.document.as_mut().unwrap_or_else(|| self.chain_i.current_mut())
  }

  /// Returns the Tangle [`MessageId`] of the latest integration [`ResolvedIotaDocument`].
  pub fn integration_message_id(&self) -> &MessageId {
    self.chain_i.current_message_id()
  }

  /// Returns the Tangle [`MessageId`] of the latest diff or integration [`ResolvedIotaDocument`].
  pub fn diff_message_id(&self) -> &MessageId {
    Self::__diff_message_id(&self.chain_i, &self.chain_d)
  }

  /// Adds a new integration document to the chain.
  ///
  /// # Errors
  ///
  /// Fails if the document is not a valid integration document.
  pub fn try_push_integration(&mut self, document: ResolvedIotaDocument) -> Result<()> {
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
    // Use the latest document state to validate the diff.
    let integration_document: &ResolvedIotaDocument = self.document.as_ref().unwrap_or_else(|| self.chain_i.current());

    // Extend the diff chain and store the merged result.
    self.document = Some(self.chain_d.try_push_and_merge(diff, integration_document)?);

    Ok(())
  }
}

impl Display for DocumentChain {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

#[cfg(test)]
mod test {
  use identity_core::common::Timestamp;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::PrivateKey;
  use identity_core::crypto::SignatureOptions;
  use identity_core::crypto::TrySignature;
  use identity_did::did::DID;
  use identity_did::verification::MethodBuilder;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodRef;
  use identity_did::verification::MethodRelationship;
  use identity_did::verification::MethodScope;
  use identity_did::verification::MethodType;
  use identity_iota_core::did::IotaDIDUrl;
  use identity_iota_core::document::IotaDocument;
  use identity_iota_core::document::IotaVerificationMethod;

  use crate::tangle::TangleRef;
  use crate::Error;

  use super::*;

  #[test]
  fn test_document_chain() {
    let mut chain: DocumentChain;
    let mut keys: Vec<KeyPair> = Vec::new();

    // =========================================================================
    // Create Initial Document
    // =========================================================================
    {
      let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
      let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
      document
        .sign_self(
          keypair.private(),
          document.default_signing_method().unwrap().id().clone(),
        )
        .unwrap();
      let mut resolved: ResolvedIotaDocument = ResolvedIotaDocument::from(document);
      resolved.set_message_id(MessageId::new([1; 32]));
      chain = DocumentChain::new(IntegrationChain::new(resolved).unwrap());
      keys.push(keypair);

      assert_eq!(
        chain.current().document.proof.as_ref().unwrap().verification_method(),
        format!("#{}", IotaDocument::DEFAULT_METHOD_FRAGMENT)
      );
      assert_eq!(chain.current().diff_message_id, MessageId::null());
      assert_eq!(chain.current().integration_message_id, MessageId::from([1; 32]));
    }

    // =========================================================================
    // Push Integration Chain Update
    // =========================================================================
    {
      let new_integration_message_id = MessageId::new([2; 32]);
      let mut new: ResolvedIotaDocument = chain.current().clone();
      new.integration_message_id = new_integration_message_id;

      // Replace the capability invocation signing key (one step key rotation).
      let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
      let signing_method: MethodRef<IotaDID> = MethodBuilder::default()
        .id(chain.id().to_url().join("#key-2").unwrap())
        .controller(chain.id().clone())
        .type_(MethodType::Ed25519VerificationKey2018)
        .data(MethodData::new_multibase(keypair.public()))
        .build()
        .map(Into::into)
        .unwrap();
      new.document.core_document_mut().capability_invocation_mut().clear();
      new
        .document
        .core_document_mut()
        .capability_invocation_mut()
        .append(signing_method);

      new.document.metadata.updated = Timestamp::now_utc();
      new.document.metadata.previous_message_id = *chain.integration_message_id();

      // Sign the update using the old document.
      assert!(chain
        .current()
        .document
        .sign_data(
          &mut new.document,
          keys[0].private(),
          chain.current().document.default_signing_method().unwrap().id(),
          SignatureOptions::default(),
        )
        .is_ok());
      assert_eq!(
        chain.current().document.signature().unwrap().verification_method(),
        format!("#{}", IotaDocument::DEFAULT_METHOD_FRAGMENT)
      );

      keys.push(keypair);
      assert!(chain.try_push_integration(new).is_ok());

      assert_eq!(chain.current().diff_message_id, MessageId::null());
      assert_eq!(chain.current().integration_message_id, new_integration_message_id);
    }

    // =========================================================================
    // Push Diff Chain Update
    // =========================================================================
    {
      let new: ResolvedIotaDocument = {
        let mut this: ResolvedIotaDocument = chain.current().clone();
        this.document.properties_mut().insert("foo".into(), 123.into());
        this.document.properties_mut().insert("bar".into(), 456.into());
        this.document.metadata.updated = Timestamp::now_utc();
        this
      };

      // Sign using the new key added in the previous integration chain update.
      let message_id: MessageId = *chain.diff_message_id();
      let mut diff: DiffMessage = chain
        .current()
        .document
        .diff(&new.document, message_id, keys[1].private(), "#key-2")
        .unwrap();

      let new_diff_message_id: MessageId = MessageId::from([3; 32]);
      diff.set_message_id(new_diff_message_id);
      assert!(chain.try_push_diff(diff).is_ok());

      // Ensure diff_message_id is updated on ResolvedIotaDocument.
      assert_eq!(chain.current().diff_message_id, new_diff_message_id);
      assert_eq!(chain.current().integration_message_id, message_id);
    }
  }

  #[test]
  fn test_check_valid_addition_rejects_removing_signing_method() {
    // =========================================================================
    // Create Initial Document
    // =========================================================================
    let (resolved, keypair): (ResolvedIotaDocument, KeyPair) = create_initial_document();
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(resolved.clone()).unwrap());

    // =========================================================================
    // Create DiffMessage Removing the Capability Invocation Method
    // =========================================================================
    let mut new_resolved: ResolvedIotaDocument = resolved.clone();
    new_resolved.document.properties_mut().insert("foo".into(), 123.into());
    new_resolved
      .document
      .core_document_mut()
      .capability_invocation_mut()
      .clear();
    new_resolved.document.metadata.updated = Timestamp::now_utc();
    new_resolved.document.metadata.previous_message_id = *chain.integration_message_id();

    let diff_msg: DiffMessage =
      create_signed_diff_message(&resolved.document, &new_resolved.document, &chain, keypair.private());
    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &resolved, chain.integration_message_id()).unwrap_err();
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
    let (resolved, keypair): (ResolvedIotaDocument, KeyPair) = create_initial_document();
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(resolved.clone()).unwrap());

    // =========================================================================
    // Create DiffMessage Adding a Capability Invocation Method
    // =========================================================================
    let mut new_resolved: ResolvedIotaDocument = resolved.clone();
    let new_signing_method: IotaVerificationMethod = IotaVerificationMethod::new(
      new_resolved.did().clone(),
      keypair.type_(),
      keypair.public(),
      "new-signing-key",
    )
    .unwrap();
    new_resolved
      .document
      .insert_method(new_signing_method, MethodScope::capability_invocation())
      .unwrap();
    new_resolved.document.metadata.updated = Timestamp::now_utc();
    new_resolved.document.metadata.previous_message_id = *chain.integration_message_id();

    let diff_msg: DiffMessage =
      create_signed_diff_message(&resolved.document, &new_resolved.document, &chain, keypair.private());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &resolved, chain.integration_message_id()).unwrap_err();
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
    let (resolved, keypair): (ResolvedIotaDocument, KeyPair) = create_initial_document();
    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(resolved.clone()).unwrap());

    // =========================================================================
    // Create DiffMessage Altering a Capability Invocation Method
    // =========================================================================
    let mut new_resolved: ResolvedIotaDocument = resolved.clone();
    // Replace the public key data.
    match new_resolved
      .document
      .core_document_mut()
      .capability_invocation_mut()
      .head_mut()
      .unwrap()
    {
      MethodRef::Embed(method) => {
        *method.data_mut() = MethodData::new_multibase([3u8; 32]);
      }
      MethodRef::Refer(_) => unreachable!(),
    };
    new_resolved.document.metadata.updated = Timestamp::now_utc();
    new_resolved.document.metadata.previous_message_id = *chain.integration_message_id();

    let diff_msg: DiffMessage =
      create_signed_diff_message(&resolved.document, &new_resolved.document, &chain, keypair.private());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &resolved, chain.integration_message_id()).unwrap_err();
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
    let (mut resolved, keypair): (ResolvedIotaDocument, KeyPair) = create_initial_document();

    let signing_method: IotaVerificationMethod = resolved.document.default_signing_method().unwrap().clone();
    let signing_method_id: IotaDIDUrl = signing_method.id().clone();
    resolved.document.remove_method(signing_method.id()).unwrap();
    resolved
      .document
      .insert_method(signing_method, MethodScope::VerificationMethod)
      .unwrap();
    assert!(resolved
      .document
      .attach_method_relationship(&signing_method_id, MethodRelationship::CapabilityInvocation)
      .unwrap());
    resolved
      .document
      .sign_self(
        keypair.private(),
        resolved.document.default_signing_method().unwrap().id().clone(),
      )
      .unwrap();

    let chain: DocumentChain = DocumentChain::new(IntegrationChain::new(resolved.clone()).unwrap());

    // =======================================================================================================
    // Create DiffMessage Altering the Verification Method that Has an Attached Capability Invocation Relationship
    // =======================================================================================================
    let mut new_resolved: ResolvedIotaDocument = resolved.clone();
    // Replace the public key data.
    let updated_method: &mut IotaVerificationMethod = new_resolved
      .document
      .core_document_mut()
      .verification_method_mut()
      .head_mut()
      .unwrap();
    *updated_method.data_mut() = MethodData::new_multibase([3u8; 32]);
    new_resolved.document.metadata.updated = Timestamp::now_utc();
    new_resolved.document.metadata.previous_message_id = *chain.integration_message_id();

    let diff_msg: DiffMessage =
      create_signed_diff_message(&resolved.document, &new_resolved.document, &chain, keypair.private());

    let valid_addition_error: Error =
      DiffChain::check_valid_addition(&diff_msg, &resolved, chain.integration_message_id()).unwrap_err();
    assert!(matches!(
      valid_addition_error,
      Error::ChainError {
        error: "diff cannot alter update signing methods"
      }
    ));
  }

  fn create_initial_document() -> (ResolvedIotaDocument, KeyPair) {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
    document
      .sign_self(
        keypair.private(),
        document.default_signing_method().unwrap().id().clone(),
      )
      .unwrap();
    let mut resolved = ResolvedIotaDocument::from(document);
    resolved.set_message_id(MessageId::new([1; 32]));
    (resolved, keypair)
  }

  fn create_signed_diff_message(
    current_doc: &IotaDocument,
    updated_doc: &IotaDocument,
    chain: &DocumentChain,
    key: &PrivateKey,
  ) -> DiffMessage {
    let mut diff_msg: DiffMessage =
      DiffMessage::new(current_doc, updated_doc, *chain.integration_message_id()).unwrap();
    current_doc
      .sign_data(
        &mut diff_msg,
        key,
        current_doc.default_signing_method().unwrap().id(),
        SignatureOptions::default(),
      )
      .unwrap();
    diff_msg.set_message_id(*chain.diff_message_id());
    diff_msg
  }
}
