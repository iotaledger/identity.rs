// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Timestamp;
use identity::crypto::KeyPair;
use identity::did::MethodBuilder;
use identity::did::MethodData;
use identity::did::MethodRef;
use identity::did::MethodType;
use identity::iota::DocumentChain;
use identity::iota::DocumentDiff;
use identity::iota::IntegrationChain;
use identity::iota::IotaDocument;
use identity::iota::MessageId;
use identity::iota::TangleRef;

pub fn setup_diff_chain_bench() -> (IotaDocument, KeyPair) {
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
  let mut document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();

  document.sign(keypair.secret()).unwrap();
  document.set_message_id(MessageId::new([8; 32]));

  (document, keypair)
}

pub fn create_diff_chain(document: IotaDocument) {
  let _ = DocumentChain::new(IntegrationChain::new(document).unwrap());
}

/// Creates a diff chain and updates it `n` times
pub fn update_diff_chain(n: usize, chain: &mut DocumentChain, keypair: &KeyPair) {
  let current_n = chain.diff().len();

  for i in current_n..(n + current_n) {
    let new: IotaDocument = {
      let mut this: IotaDocument = chain.current().clone();
      this.properties_mut().insert(i.to_string(), 123.into());
      this.set_updated(Timestamp::now());
      this
    };

    let message_id = *chain.diff_message_id();
    let mut diff: DocumentDiff = chain.current().diff(&new, message_id, keypair.secret()).unwrap();

    diff.set_message_id(message_id);
    assert!(chain.try_push_diff(diff).is_ok());
  }
}

/// Creates an integration chain and updates it `n` times
pub fn update_integration_chain(n: usize, chain: &mut DocumentChain, keypair: &KeyPair) {
  let current_n = chain.diff().len();

  for i in current_n..(n + current_n) {
    let mut new: IotaDocument = chain.current().clone();

    let authentication: MethodRef = MethodBuilder::default()
      .id(chain.id().join(&format!("#key-{}", i)).unwrap().into())
      .controller(chain.id().clone().into())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_b58(keypair.public()))
      .build()
      .map(Into::into)
      .unwrap();

    unsafe {
      new.as_document_mut().authentication_mut().clear();
      new.as_document_mut().authentication_mut().append(authentication.into());
    }

    new.set_updated(Timestamp::now());
    new.set_previous_message_id(*chain.integration_message_id());

    chain.current().sign_data(&mut new, keypair.secret()).unwrap();
    chain.try_push_integration(new).unwrap();
  }
}
