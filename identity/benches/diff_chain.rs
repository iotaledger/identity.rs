// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use identity::{
  did::{MethodBuilder, MethodData, MethodRef, MethodType},
  iota::MessageId,
};
use identity_core::common::Timestamp;
use identity_core::crypto::KeyPair;
use identity_iota::chain::AuthChain;
use identity_iota::chain::DocumentChain;
use identity_iota::did::Document;
use identity_iota::did::DocumentDiff;
use identity_iota::tangle::TangleRef;

pub fn setup_diff_chain_bench() -> (Document, KeyPair) {
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
  let mut document: Document = Document::from_keypair(&keypair).unwrap();
  document.sign(keypair.secret()).unwrap();
  document.set_message_id(MessageId::new([8; 32]));

  (document, keypair)
}

pub fn create_diff_chain(document: Document) {
  let _ = DocumentChain::new(AuthChain::new(document).unwrap());
}
/// Creates a diff chain and updates it `n` times
pub fn update_diff_chain(n: usize, chain: &mut DocumentChain, keypair: &KeyPair) {
  let current_n = chain.diff().len();
  for i in current_n..(n + current_n) {
    let new: Document = {
      let mut this: Document = chain.current().clone();
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
/// Creates a diff chain and updates it `n` times
pub fn update_auth_chain(n: usize, chain: &mut DocumentChain, keypair: &KeyPair) {
  let current_n = chain.diff().len();
  for i in current_n..(n + current_n) {
    let mut new: Document = chain.current().clone();

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
    new.set_previous_message_id(*chain.auth_message_id());

    chain.current().sign_data(&mut new, keypair.secret()).unwrap();

    chain.try_push_auth(new).unwrap();
  }
}
