// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(deprecated)]

use identity::core::Timestamp;
use identity::crypto::KeyPair;
use identity::did::MethodBuilder;
use identity::did::MethodData;
use identity::did::MethodRef;
use identity::did::MethodType;
use identity::did::DID;
use identity::iota::DocumentChain;
use identity::iota::TangleRef;
use identity::iota_core::DiffMessage;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaDocument;
use identity::iota_core::MessageId;
use identity_core::crypto::KeyType;
use identity_core::crypto::ProofOptions;
use identity_iota_client::document::ResolvedIotaDocument;

pub fn setup_diff_chain_bench() -> (ResolvedIotaDocument, KeyPair) {
  let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
  let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();

  document
    .sign_self(
      keypair.private(),
      document.default_signing_method().unwrap().id().clone(),
    )
    .unwrap();

  let mut resolved: ResolvedIotaDocument = ResolvedIotaDocument::from(document);
  resolved.set_message_id(MessageId::new([8; 32]));

  (resolved, keypair)
}

/// Creates a diff chain and updates it `n` times
pub fn update_diff_chain(n: usize, chain: &mut DocumentChain, keypair: &KeyPair) {
  let current_n = chain.diff().len();

  for i in current_n..(n + current_n) {
    let new: IotaDocument = {
      let mut this: IotaDocument = chain.current().clone().document;
      this.properties_mut().insert(i.to_string(), 123.into());
      this.metadata.updated = Some(Timestamp::now_utc());
      this
    };

    let message_id = *chain.diff_message_id();
    let mut diff: DiffMessage = chain
      .current()
      .document
      .diff(
        &new,
        message_id,
        keypair.private(),
        chain.current().document.default_signing_method().unwrap().id(),
      )
      .unwrap();

    diff.set_message_id(message_id);
    assert!(chain.try_push_diff(diff).is_ok());
  }
}

/// Creates an integration chain and updates it `n` times
pub fn update_integration_chain(n: usize, chain: &mut DocumentChain, keypair: &KeyPair) {
  let current_n = chain.diff().len();

  for i in current_n..(n + current_n) {
    let mut new: ResolvedIotaDocument = chain.current().clone();

    let authentication: MethodRef<IotaDID> = MethodBuilder::default()
      .id(chain.id().to_url().join(&format!("#key-{}", i)).unwrap())
      .controller(chain.id().clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(keypair.public()))
      .build()
      .map(Into::into)
      .unwrap();

    new.document.core_document_mut().authentication_mut().clear();
    new
      .document
      .core_document_mut()
      .authentication_mut()
      .append(authentication);

    new.document.metadata.updated = Some(Timestamp::now_utc());
    new.document.metadata.previous_message_id = *chain.integration_message_id();

    chain
      .current()
      .document
      .sign_data(
        &mut new.document,
        keypair.private(),
        chain.current().document.default_signing_method().unwrap().id(),
        ProofOptions::default(),
      )
      .unwrap();
    chain.try_push_integration(new).unwrap();
  }
}
