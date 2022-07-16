// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_did::did::CoreDID;
use identity_did::document::CoreDocument;
use identity_did::document::DocumentBuilder;
use identity_did::verification::MethodRef;
use serde::Deserialize;
use serde::Serialize;

use crate::did_or_placeholder::DIDOrPlaceholder;
use crate::did_or_placeholder::PLACEHOLDER_DID;
use crate::StardustDocument;

/// The DID document as it is contained in an alias output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StateMetadataDocument(CoreDocument<DIDOrPlaceholder>);

impl StateMetadataDocument {
  /// Transforms the document into a [`StardustDocument`] by replacing all placeholders with `original_did`.
  pub fn into_stardust_document(mut self, original_did: &CoreDID) -> StardustDocument {
    let mut builder: DocumentBuilder<CoreDID, Object, Object, Object> =
      CoreDocument::builder(self.0.properties().clone()).id(original_did.clone());

    let mut controllers = None;
    std::mem::swap(&mut controllers, self.0.controller_mut());

    let original_did_ctor = || original_did.clone();

    if let Some(controllers) = controllers {
      for controller in controllers.into_vec().into_iter() {
        builder = builder.controller(controller.unwrap_or_else(original_did_ctor));
      }
    }

    for method in self
      .0
      .verification_method_mut()
      .as_vec_mut()
      .drain(..)
      .map(|method| method.map(|did| did.unwrap_or_else(original_did_ctor)))
    {
      builder = builder.verification_method(method);
    }

    let method_ref_transform =
      |method_ref: MethodRef<DIDOrPlaceholder, _>| method_ref.map(|did| did.unwrap_or_else(original_did_ctor));

    for method_ref in self
      .0
      .assertion_method_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.assertion_method(method_ref);
    }
    for method_ref in self
      .0
      .capability_delegation_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.capability_delegation(method_ref);
    }
    for method_ref in self
      .0
      .capability_invocation_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.capability_invocation(method_ref);
    }
    for method_ref in self
      .0
      .key_agreement_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.key_agreement(method_ref);
    }
    for method_ref in self
      .0
      .authentication_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.authentication(method_ref);
    }

    for service in self.0.service_mut().as_vec_mut().drain(..) {
      builder = builder.service(service.map(|did| did.unwrap_or_else(original_did_ctor)));
    }

    let mut core_doc: CoreDocument = builder.build().expect("all required document fields should be set");

    std::mem::swap(core_doc.also_known_as_mut(), self.0.also_known_as_mut());

    StardustDocument(core_doc)
  }
}

impl From<StardustDocument> for StateMetadataDocument {
  /// Transforms a [`StardustDocument`] into its state metadata representation by replacing all
  /// occurrences of its did with a placeholder.
  fn from(mut doc: StardustDocument) -> Self {
    let mut original_did: CoreDID = PLACEHOLDER_DID.to_owned();
    std::mem::swap(&mut original_did, doc.0.id_mut());

    let mut builder: DocumentBuilder<DIDOrPlaceholder, _, _, _> =
      CoreDocument::builder(doc.0.properties().clone()).id(DIDOrPlaceholder::Placeholder);

    let mut controllers = None;
    std::mem::swap(&mut controllers, doc.0.controller_mut());

    if let Some(controllers) = controllers {
      for controller in controllers.into_vec().into_iter() {
        builder = builder.controller(DIDOrPlaceholder::new(controller, &original_did));
      }
    }

    for method in doc
      .0
      .verification_method_mut()
      .as_vec_mut()
      .drain(..)
      .map(|method| method.map(|did| DIDOrPlaceholder::new(did, &original_did)))
    {
      builder = builder.verification_method(method);
    }

    let method_ref_transform = |method_ref: MethodRef| method_ref.map(|did| DIDOrPlaceholder::new(did, &original_did));

    for method_ref in doc
      .0
      .assertion_method_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.assertion_method(method_ref);
    }
    for method_ref in doc
      .0
      .capability_delegation_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.capability_delegation(method_ref);
    }
    for method_ref in doc
      .0
      .capability_invocation_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.capability_invocation(method_ref);
    }
    for method_ref in doc
      .0
      .key_agreement_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.key_agreement(method_ref);
    }
    for method_ref in doc
      .0
      .authentication_mut()
      .as_vec_mut()
      .drain(..)
      .map(method_ref_transform)
    {
      builder = builder.authentication(method_ref);
    }

    for service in doc.0.service_mut().as_vec_mut().drain(..) {
      builder = builder.service(service.map(|did| DIDOrPlaceholder::new(did, &original_did)));
    }

    let mut core_doc: CoreDocument<DIDOrPlaceholder, _, _, _> =
      builder.build().expect("all required document fields should be set");

    std::mem::swap(core_doc.also_known_as_mut(), doc.0.also_known_as_mut());

    StateMetadataDocument(core_doc)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::OneOrSet;
  use identity_core::common::Url;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::CoreDID;
  use identity_did::verification::MethodScope;

  use crate::DIDOrPlaceholder;
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
        document.id().clone(),
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
        document.id().clone(),
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

    println!("document\n{}", document.to_json_pretty().unwrap());

    let state_metadata_doc: StateMetadataDocument = StateMetadataDocument::from(document.clone());

    println!("state_metadata_doc\n{}", state_metadata_doc.to_json_pretty().unwrap());

    assert!(matches!(
      state_metadata_doc
        .0
        .resolve_method("#did-self", None)
        .unwrap()
        .id()
        .did(),
      DIDOrPlaceholder::Placeholder
    ));

    assert!(matches!(
      state_metadata_doc
        .0
        .resolve_method("#did-foreign", None)
        .unwrap()
        .id()
        .did(),
      DIDOrPlaceholder::Core(did) if did == &example_did
    ));

    assert!(matches!(
      state_metadata_doc
        .0
        .service().iter().find(|service| service.id().fragment().unwrap() == "my-foreign-service")
        .unwrap()
        .id()
        .did(),
      DIDOrPlaceholder::Core(did) if did == &key_did
    ));

    assert!(matches!(
      state_metadata_doc
        .0
        .service()
        .iter()
        .find(|service| service.id().fragment().unwrap() == "my-service")
        .unwrap()
        .id()
        .did(),
      DIDOrPlaceholder::Placeholder
    ));

    let controllers = state_metadata_doc.0.controller().unwrap();
    assert!(matches!(controllers.get(0).unwrap(), DIDOrPlaceholder::Core(_)));
    assert!(matches!(controllers.get(1).unwrap(), DIDOrPlaceholder::Placeholder));

    let stardust_document = state_metadata_doc.into_stardust_document(&original_did);

    println!("stardust_document\n{}", stardust_document.to_json_pretty().unwrap());

    assert_eq!(stardust_document, document);
  }
}
