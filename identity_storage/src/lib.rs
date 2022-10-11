// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod blob_storage;
mod core_document_ext;
mod create_method;
mod error;
mod identity_suite;
mod identity_updater;
mod jcs_ed25519;
mod key_alias;
mod key_storage;
mod key_types;
mod memstore;
mod method_content;
mod method_suite;
mod method_type1;
mod signature;
mod signature_handler;
mod signature_types;
mod storage;
mod storage_combinator;

pub use blob_storage::*;
pub use core_document_ext::*;
pub use create_method::*;
pub use error::*;
pub use identity_suite::*;
pub use identity_updater::*;
pub use jcs_ed25519::*;
pub use key_alias::*;
pub use key_storage::*;
pub use key_types::*;
pub use memstore::*;
pub use method_content::*;
pub use method_suite::*;
pub use method_type1::*;
pub use signature::*;
pub use signature_handler::*;
pub use signature_types::*;
pub use storage::*;
pub use storage_combinator::*;

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

  use crate::CoreDocumentExt;
  use crate::IdentitySuite;
  use crate::JcsEd25519;
  use crate::MemStore;
  use crate::MethodContent;
  use crate::MethodType1;

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
    let fragment = "my-key";
    let key_storage = MemStore::new();

    let did = CoreDID::parse("did:iota:0x0000").unwrap();
    let mut document: CoreDocument = CoreDocument::builder(Default::default()).id(did).build().unwrap();

    document
      .update_identity()
      .create_method()
      .key_storage(&key_storage)
      .content(MethodContent::Generate(MethodType1::ed25519_verification_key_2018()))
      .fragment(fragment)
      .apply()
      .await;

    assert!(document.resolve_method(fragment, Default::default()).is_some());

    let mut suite = IdentitySuite::new(key_storage);

    // This adds an "Ed25519VerificationKey2018" -> JcsEd25519 handler mapping.
    suite.register(JcsEd25519);

    let mut credential: Credential = test_credential();

    // Since `fragment` resolves to a method of type "Ed25519VerificationKey2018",
    // the JcsEd25519 handler is invoked to sign.
    document
      .sign(&mut credential, fragment, &suite, Default::default())
      .await;

    assert!(credential.signature().is_some());

    println!("{}", credential.to_json_pretty().unwrap());
  }
}

// #[allow(dead_code, unused_variables)]
// mod custom_user_impl {
//   use crate::KeyAlias;
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

//     async fn generate<KT: Send + Into<Self::KeyType>>(&self, key_type: KT) -> StorageResult<KeyAlias> {
//       todo!()
//     }
//     async fn public(&self, private_key: &KeyAlias) -> StorageResult<PublicKey> {
//       todo!()
//     }
//     async fn sign<ST: Send + Into<Self::SigningAlgorithm>>(
//       &self,
//       private_key: &KeyAlias,
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
//       let private_key: KeyAlias = KeyAlias::new("random_string");
//       key_storage
//         .sign(&private_key, MySigningAlgorithm::Secp, data)
//         .await
//         .expect("TODO")
//         .0
//     }
//   }
// }
