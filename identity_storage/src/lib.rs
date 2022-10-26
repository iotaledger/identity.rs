// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod blob_storage;
mod core_document_ext;
mod create_method;
mod error;
mod identity_updater;
mod jcs_ed25519;
mod key_alias;
mod key_storage;
mod key_types;
mod mem_blob_store;
mod memstore;
mod method_content;
mod method_hash;
mod method_suite;
mod signature;
mod signature_handler;
mod signature_suite;
mod signature_types;
mod storage;
mod storage_combinator;
mod storage_error;

pub use blob_storage::*;
pub use core_document_ext::*;
pub use create_method::*;
pub use error::*;
pub use identity_updater::*;
pub use jcs_ed25519::*;
pub use key_alias::*;
pub use key_storage::*;
pub use key_types::*;
pub use mem_blob_store::*;
pub use memstore::*;
pub use method_content::*;
pub use method_suite::*;
pub use signature::*;
pub use signature_handler::*;
pub use signature_suite::*;
pub use signature_types::*;
pub use storage::*;
pub use storage_combinator::*;
pub use storage_error::*;

#[cfg(test)]
mod tests2 {

  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_core::crypto::GetSignature;
  use identity_core::json;
  use identity_credential::credential::Credential;
  use identity_credential::credential::Subject;
  use identity_did::did::CoreDID;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;
  use identity_did::verification::MethodType;

  use crate::CoreDocumentExt;
  use crate::Ed25519VerificationKey2018;
  use crate::JcsEd25519;
  use crate::MemBlobStore;
  use crate::MemKeyStore;
  use crate::MethodContent;
  use crate::MethodSuite;
  use crate::SignatureSuite;
  use crate::Storage;

  fn test_credential() -> Credential {
    let issuer_did: CoreDID = "did:iota:0x0001".parse().unwrap();
    let subject_did: CoreDID = "did:iota:0x0002".parse().unwrap();

    let subject: Subject = Subject::from_json_value(json!({
      "id": subject_did.as_str(),
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    }))
    .unwrap();

    Credential::builder(Default::default())
      .issuer(Url::parse(issuer_did.as_str()).unwrap())
      .type_("UniversityDegreeCredential")
      .subject(subject)
      .build()
      .unwrap()
  }

  #[tokio::test]
  async fn test_things() {
    let fragment = "#my-key";
    let key_storage = MemKeyStore::new();
    let blob_storage = MemBlobStore::new();
    let storage = Storage::new(key_storage, blob_storage);

    let did = CoreDID::parse("did:iota:0x0000").unwrap();
    let mut document: CoreDocument = CoreDocument::builder(Default::default()).id(did).build().unwrap();

    let mut method_suite = MethodSuite::new(storage.clone());
    method_suite.register(Ed25519VerificationKey2018);

    document
      .update_identity()
      .create_method()
      .method_suite(&method_suite)
      .type_(MethodType::ED25519_VERIFICATION_KEY_2018)
      .content(MethodContent::Generate)
      .fragment(fragment)
      .apply()
      .await;

    assert!(document.resolve_method(fragment, Default::default()).is_some());

    let mut signature_suite = SignatureSuite::new(storage);
    // This adds an "Ed25519VerificationKey2018" -> JcsEd25519 handler mapping.
    signature_suite.register(JcsEd25519);

    let mut credential: Credential = test_credential();

    // Since `fragment` resolves to a method of type "Ed25519VerificationKey2018",
    // the JcsEd25519 handler is invoked to sign.
    document
      .sign(&mut credential, fragment, &signature_suite, Default::default())
      .await;

    assert!(credential.signature().is_some());

    println!("{}", credential.to_json_pretty().unwrap());
  }
}

// #[allow(dead_code, unused_variables)]
// mod custom_user_impl {
//   use crate::KeyId;
//   use crate::KeyStorage;
//   use crate::Signature;
//   use crate::SignatureHandler;
//   use crate::StorageResult;
//   use async_trait::async_trait;
//   use identity_core::crypto::PublicKey;

//   pub struct SecpSignatureHandler;

//   pub enum MySigningAlgorithm {
//     Secp,
//   }

//   pub struct MyStorage;
//   #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
//   #[cfg_attr(feature = "send-sync-storage", async_trait)]
//   impl KeyStorage for MyStorage {
//     type KeyType = ();
//     type SigningAlgorithm = MySigningAlgorithm;

//     async fn generate<KT: Send + Into<Self::KeyType>>(&self, key_type: KT) -> StorageResult<KeyId> {
//       todo!()
//     }
//     async fn public(&self, private_key: &KeyId) -> StorageResult<PublicKey> {
//       todo!()
//     }
//     async fn sign<ST: Send + Into<Self::SigningAlgorithm>>(
//       &self,
//       private_key: &KeyId,
//       signing_algorithm: ST,
//       data: Vec<u8>,
//     ) -> StorageResult<Signature> {
//       todo!()
//     }
//   }

//   #[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
//   #[cfg_attr(feature = "send-sync-storage", async_trait)]
//   impl SignatureHandler<MyStorage> for SecpSignatureHandler {
//     async fn sign(&self, data: Vec<u8>, key_storage: &MyStorage) -> Vec<u8> {
//       // TODO: Alias needs to be passed in.
//       let private_key: KeyId = KeyId::new("random_string");
//       key_storage
//         .sign(&private_key, MySigningAlgorithm::Secp, data)
//         .await
//         .expect("TODO")
//         .0
//     }
//   }
// }
