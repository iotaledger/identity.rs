// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod blob_storage;
mod core_document_ext;
mod create_method;
mod error;
mod identity_updater;
mod key_alias;
mod key_storage;
mod memstore;
mod method_content;
mod signature;
mod signature_handler;
// mod signature_suite;
// mod signing_algorithm;
mod signature_types;
mod storage;
mod storage_combinator;

use std::borrow::Cow;
use std::collections::HashMap;

pub use blob_storage::*;
pub use core_document_ext::*;
pub use create_method::*;
pub use error::*;
pub use identity_updater::*;
pub use key_alias::*;
pub use key_storage::*;
pub use memstore::*;
pub use method_content::*;
pub use signature::*;
pub use signature_handler::*;
// pub use signature_suite::*;
// pub use signing_algorithm::*;
pub use signature_types::*;
pub use storage::*;
pub use storage_combinator::*;

// #[cfg(test)]
// mod tests {
//   use identity_did::{did::CoreDID, document::CoreDocument, verification::MethodType};

//   use crate::{CoreDocumentExt, MemStore, MethodContent};

//   #[tokio::test]
//   async fn test_add_key() -> anyhow::Result<()> {
//     let storage = MemStore::new();

//     let did = CoreDID::parse("did:iota:0x0000").unwrap();
//     let mut doc: CoreDocument = CoreDocument::builder(Default::default()).id(did).build().unwrap();

//     doc
//       .update_identity()
//       .create_method()
//       .content(MethodContent::Generate(NewMethodType::Ed25519VerificationKey2018))
//       .fragment("my-next-key")
//       .apply(&storage)
//       .await;

//     doc.resolve_method("my-next-key", Default::default()).unwrap();

//     Ok(())
//   }
// }

#[derive(PartialEq, Eq, Hash)]
pub struct NewMethodType {
  repr: Cow<'static, str>,
}

impl NewMethodType {
  pub fn as_str(&self) -> &str {
    self.repr.as_ref()
  }

  pub const fn ed25519_verification_key_2018() -> Self {
    Self {
      repr: Cow::Borrowed("Ed25519VerificationKey2018"),
    }
  }

  pub const fn x25519_verification_key_2018() -> Self {
    Self {
      repr: Cow::Borrowed("X25519VerificationKey2018"),
    }
  }
}

pub struct IdentitySuite<K: KeyStorage> {
  key_storage: K,
  signature_handlers: HashMap<String, Box<dyn SignatureHandler<K>>>,
}

impl<K: KeyStorage> IdentitySuite<K> {
  pub fn new(key_storage: K) -> Self {
    Self {
      key_storage,
      signature_handlers: HashMap::new(),
    }
  }

  pub fn register(&mut self, handler: Box<dyn SignatureHandler<K>>) {
    self
      .signature_handlers
      .insert(handler.typ().as_str().to_owned(), handler);
  }

  pub fn register_raw(&mut self, signature_type: String, handler: Box<dyn SignatureHandler<K>>) {
    self.signature_handlers.insert(signature_type, handler);
  }

  pub async fn sign(&self, data: Vec<u8>, method_type: &NewMethodType) -> Vec<u8> {
    match self.signature_handlers.get(method_type.as_str()) {
      Some(handler) => handler.sign(data, &self.key_storage).await,
      None => todo!("return missing handler error"),
    }
  }
}

#[cfg(test)]
mod tests2 {
  use identity_did::did::CoreDID;
  use identity_did::document::CoreDocument;

  use crate::CoreDocumentExt;
  use crate::IdentitySuite;
  use crate::JcsEd25519;
  use crate::MemStore;
  use crate::MethodContent;
  use crate::NewMethodType;

  #[tokio::test]
  async fn test_things() {
    let fragment = "my-key";
    let key_storage = MemStore::new();

    let did = CoreDID::parse("did:iota:0x0000").unwrap();
    let mut document: CoreDocument = CoreDocument::builder(Default::default()).id(did).build().unwrap();

    document
      .update_identity()
      .create_method()
      .content(MethodContent::Generate(NewMethodType::ed25519_verification_key_2018()))
      .fragment(fragment)
      .apply(&key_storage)
      .await;

    assert!(document.resolve_method(fragment, Default::default()).is_some());

    let mut suite = IdentitySuite::new(key_storage);
    // This adds a "Ed25519VerificationKey2018" -> JcsEd25519 handler mapping.
    suite.register(Box::new(JcsEd25519));

    // Since `fragment` resolves to a method of type "Ed25519VerificationKey2018",
    // the JcsEd25519 handler is invoked to sign.
    let signature = document.sign(fragment, b"data to be signed".to_vec(), &suite).await;

    assert_eq!(signature.len(), 64);
  }
}
