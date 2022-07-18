// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::did::CoreDID;
use identity_did::document::CoreDocument;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;

use crate::StardustDocument;

pub(crate) static PLACEHOLDER_DID: Lazy<CoreDID> = Lazy::new(|| CoreDID::parse("did:0:0").unwrap());

/// The DID document as it is contained in the state metadata of an alias output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub(crate) struct StateMetadataDocument(CoreDocument);

impl StateMetadataDocument {
  /// Transforms the document into a [`StardustDocument`] by replacing all placeholders with `original_did`.
  pub fn into_stardust_document(self, original_did: &CoreDID) -> StardustDocument {
    let core_doc: CoreDocument = self.0;
    let core_document: CoreDocument = core_doc.map(
      // Replace placeholder identifiers.
      |did| {
        if did == PLACEHOLDER_DID.as_ref() {
          original_did.clone()
        } else {
          did
        }
      },
      // Do not modify properties.
      |o| o,
    );
    StardustDocument(core_document)
  }
}

impl From<StardustDocument> for StateMetadataDocument {
  /// Transforms a [`StardustDocument`] into its state metadata representation by replacing all
  /// occurrences of its did with a placeholder.
  fn from(document: StardustDocument) -> Self {
    let core_document: CoreDocument = CoreDocument::from(document);
    let id: CoreDID = core_document.id().clone();
    let core_doc: CoreDocument = core_document.map(
      // Replace self-referential identifiers with a placeholder, but not others.
      |did| {
        if did == id {
          PLACEHOLDER_DID.clone()
        } else {
          did
        }
      },
      // Do not modify properties.
      |o| o,
    );
    StateMetadataDocument(core_doc)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::OneOrSet;
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::CoreDID;
  use identity_did::verification::MethodScope;

  use crate::state_metadata_document::PLACEHOLDER_DID;
  use crate::StardustDocument;
  use crate::StateMetadataDocument;

  #[test]
  fn test_transformation_roundtrip() {
    let original_did =
      CoreDID::parse("did:stardust:8036235b6b5939435a45d68bcea7890eef399209a669c8c263fac7f5089b2ec6").unwrap();
    let example_did = CoreDID::parse("did:example:1zdksdnrjq0ru092sdfsd491cxvs03e0").unwrap();
    let key_did = CoreDID::parse("did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK").unwrap();

    let mut document: StardustDocument = StardustDocument::new();
    document.tmp_set_id(original_did.clone());

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    document
      .tmp_add_verification_method(
        document.tmp_id().clone(),
        &keypair,
        "#did-self",
        MethodScope::VerificationMethod,
      )
      .unwrap();

    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    document
      .tmp_add_verification_method(
        example_did.clone(),
        &keypair,
        "#did-foreign",
        MethodScope::authentication(),
      )
      .unwrap();

    document
      .tmp_add_service(
        document.tmp_id().clone(),
        "#my-service",
        "RevocationList2030",
        identity_did::service::ServiceEndpoint::One(Url::parse("https://example.com/xyzabc").unwrap()),
      )
      .unwrap();

    document
      .tmp_add_service(
        key_did.clone(),
        "#my-foreign-service",
        "RevocationList2030",
        identity_did::service::ServiceEndpoint::One(Url::parse("https://example.com/0xf4c42e9da").unwrap()),
      )
      .unwrap();

    document
      .0
      .also_known_as_mut()
      .append(Url::parse("did:example:abc").unwrap());

    document
      .0
      .also_known_as_mut()
      .append(Url::parse("did:example:xyz").unwrap());

    let mut controllers = OneOrSet::new_one(example_did.clone());
    (&mut controllers).append(original_did.clone());
    let mut controllers = Some(controllers);
    std::mem::swap(&mut controllers, document.0.controller_mut());

    let state_metadata_doc: StateMetadataDocument = StateMetadataDocument::from(document.clone());

    assert_eq!(
      state_metadata_doc
        .0
        .resolve_method("#did-self", None)
        .unwrap()
        .id()
        .did(),
      PLACEHOLDER_DID.as_ref()
    );

    assert_eq!(
      state_metadata_doc
        .0
        .resolve_method("#did-foreign", None)
        .unwrap()
        .id()
        .did(),
      &example_did
    );

    assert_eq!(
      state_metadata_doc
        .0
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-foreign-service")
        .unwrap()
        .id()
        .did(),
      &key_did
    );

    assert_eq!(
      state_metadata_doc
        .0
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-service")
        .unwrap()
        .id()
        .did(),
      PLACEHOLDER_DID.as_ref()
    );

    let controllers = state_metadata_doc.0.controller().unwrap();
    assert_eq!(controllers.get(0).unwrap(), &example_did);
    assert_eq!(controllers.get(1).unwrap(), PLACEHOLDER_DID.as_ref());

    let stardust_document = state_metadata_doc.into_stardust_document(&original_did);

    assert_eq!(stardust_document, document);
  }
}
