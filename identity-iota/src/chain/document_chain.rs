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
use crate::diff::DiffMessage;
use crate::document::ResolvedIotaDocument;
use crate::error::Result;
use crate::tangle::MessageId;

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
    // Use the last integration chain document to validate the signature on the diff.
    let integration_document: &ResolvedIotaDocument = self.chain_i.current();
    let expected_prev_message_id: &MessageId = self.diff_message_id();
    DiffChain::check_valid_addition(&diff, integration_document, expected_prev_message_id)?;

    // Merge the diff into the latest state.
    let mut document: ResolvedIotaDocument = self.document.take().unwrap_or_else(|| self.chain_i.current().clone());
    document.merge_diff_message(&diff)?;

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

#[cfg(test)]
mod test {
  use identity_core::common::Timestamp;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::SignatureOptions;
  use identity_core::crypto::TrySignature;
  use identity_did::did::CoreDIDUrl;
  use identity_did::did::DID;
  use identity_did::verification::MethodBuilder;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodRef;
  use identity_did::verification::MethodType;

  use crate::document::IotaDocument;
  use crate::tangle::TangleRef;

  use super::*;

  #[test]
  fn test_document_chain() {
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
      let mut resolved: ResolvedIotaDocument = ResolvedIotaDocument::from(document);
      resolved.set_message_id(MessageId::new([1; 32]));
      chain = DocumentChain::new(IntegrationChain::new(resolved).unwrap());
      keys.push(keypair);

      assert_eq!(
        chain
          .current()
          .document
          .metadata
          .proof
          .as_ref()
          .unwrap()
          .verification_method(),
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
      let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
      let signing_method: MethodRef = MethodBuilder::default()
        .id(CoreDIDUrl::from(chain.id().to_url().join("#key-2").unwrap()))
        .controller(chain.id().clone().into())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_multibase(keypair.public()))
        .build()
        .map(Into::into)
        .unwrap();

      unsafe {
        new.document.core_document_mut().capability_invocation_mut().clear();
        new
          .document
          .core_document_mut()
          .capability_invocation_mut()
          .append(signing_method);
      }

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
}
