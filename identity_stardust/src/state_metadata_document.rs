// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_did::did::CoreDID;
use identity_did::did::DIDUrl;
use identity_did::document::CoreDocument;
use identity_did::document::DocumentBuilder;
use identity_did::verification::MethodRef;
use identity_did::verification::VerificationMethod;
use serde::Deserialize;
use serde::Serialize;

use crate::did_or_placeholder::DidOrPlaceholder;
use crate::StardustDocument;

/// The DID document as it is serialized in an alias output.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StateMetadataDocument(CoreDocument<DidOrPlaceholder>);

impl StateMetadataDocument {
  /// Transforms the document into a [`StardustDocument`] by replacing all placeholders with `original_did`.
  pub fn to_stardust_document(&self, original_did: &CoreDID) -> StardustDocument {
    let mut builder: DocumentBuilder<CoreDID, Object, Object, Object> =
      CoreDocument::builder(self.0.properties().clone()).id(original_did.clone());

    if let Some(controllers) = self.0.controller() {
      for controller in controllers.iter() {
        builder = builder.controller(controller.clone().unwrap_or_else(|| original_did.clone()));
      }
    }

    for aka in self.0.also_known_as().iter() {
      builder = builder.also_known_as(aka.clone());
    }

    for method in self.0.verification_method().iter() {
      let verification_method: VerificationMethod<CoreDID, Object> =
        reverse_transform_method(method.clone(), original_did);
      builder = builder.verification_method(verification_method);
    }
    for method_ref in self.0.assertion_method().iter() {
      builder = builder.assertion_method(reverse_transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in self.0.capability_delegation().iter() {
      builder = builder.capability_delegation(reverse_transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in self.0.capability_invocation().iter() {
      builder = builder.capability_invocation(reverse_transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in self.0.key_agreement().iter() {
      builder = builder.key_agreement(reverse_transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in self.0.authentication().iter() {
      builder = builder.authentication(reverse_transform_method_ref(method_ref.clone(), original_did));
    }

    for service in self.0.service().iter() {
      builder = builder.service(service.clone().map(|did| did.unwrap_or_else(|| original_did.clone())));
    }

    let core_doc: CoreDocument = builder.build().expect("all required document fields should be set");

    StardustDocument(core_doc)
  }
}

impl From<StardustDocument> for StateMetadataDocument {
  /// Transforms a [`StardustDocument`] into its state metadata representation by replacing all
  /// occurences of its did with a placeholder.
  fn from(doc: StardustDocument) -> Self {
    let original_did: &CoreDID = doc.0.id();
    // TODO: Can we avoid all the clones? Not really unless we make the CoreDocument fields pub.
    // We should at least use mem swap in cases where we can get access to a &mut ref.
    let mut builder: DocumentBuilder<DidOrPlaceholder, _, _, _> =
      CoreDocument::builder(doc.0.properties().clone()).id(DidOrPlaceholder::Placeholder);

    if let Some(controllers) = doc.0.controller() {
      for controller in controllers.iter() {
        builder = builder.controller(DidOrPlaceholder::from(controller.clone()).replace(controller == original_did));
      }
    }

    for aka in doc.0.also_known_as().iter() {
      builder = builder.also_known_as(aka.clone());
    }

    for method in doc.0.verification_method().iter() {
      let verification_method: VerificationMethod<DidOrPlaceholder, Object> =
        transform_method(method.clone(), original_did);
      builder = builder.verification_method(verification_method);
    }
    for method_ref in doc.0.assertion_method().iter() {
      builder = builder.assertion_method(transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in doc.0.capability_delegation().iter() {
      builder = builder.capability_delegation(transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in doc.0.capability_invocation().iter() {
      builder = builder.capability_invocation(transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in doc.0.key_agreement().iter() {
      builder = builder.key_agreement(transform_method_ref(method_ref.clone(), original_did));
    }
    for method_ref in doc.0.authentication().iter() {
      builder = builder.authentication(transform_method_ref(method_ref.clone(), original_did));
    }

    for service in doc.0.service().iter() {
      builder = builder.service(
        service
          .clone()
          .map(|did| DidOrPlaceholder::from(did).replace(service.id().did() == original_did)),
      );
    }

    let core_doc: CoreDocument<DidOrPlaceholder, _, _, _> =
      builder.build().expect("all required document fields should be set");

    StateMetadataDocument(core_doc)
  }
}

fn transform_method(
  method: VerificationMethod<CoreDID, Object>,
  original_did: &CoreDID,
) -> VerificationMethod<DidOrPlaceholder, Object> {
  let mut builder = VerificationMethod::<DidOrPlaceholder, Object>::builder(method.properties().clone())
    .type_(method.type_())
    .data(method.data().clone());

  builder = builder
    .controller(DidOrPlaceholder::from(method.controller().clone()).replace(method.controller() == original_did));
  let id: DIDUrl<DidOrPlaceholder> = method.id().clone().map(|did| {
    let replace = &did == original_did;
    DidOrPlaceholder::from(did).replace(replace)
  });
  builder = builder.id(id);

  builder.build().expect("all required method fields should be set")
}

fn reverse_transform_method(
  method: VerificationMethod<DidOrPlaceholder, Object>,
  did: &CoreDID,
) -> VerificationMethod<CoreDID, Object> {
  let mut builder = VerificationMethod::<CoreDID, Object>::builder(method.properties().clone())
    .type_(method.type_())
    .data(method.data().clone());

  builder = builder.controller(method.controller().clone().unwrap_or_else(|| did.clone()));
  let id: DIDUrl<CoreDID> = method
    .id()
    .clone()
    .map(|did_or_placeholder| did_or_placeholder.unwrap_or_else(|| did.clone()));
  builder = builder.id(id);

  builder.build().expect("all required method fields should be set")
}

fn transform_method_ref(
  method_ref: MethodRef<CoreDID, Object>,
  original_did: &CoreDID,
) -> MethodRef<DidOrPlaceholder, Object> {
  match method_ref {
    MethodRef::Embed(method) => MethodRef::Embed(transform_method(method, original_did)),
    MethodRef::Refer(reference) => MethodRef::Refer(reference.map(|did| {
      let replace = &did == original_did;
      DidOrPlaceholder::from(did).replace(replace)
    })),
  }
}

fn reverse_transform_method_ref(
  method_ref: MethodRef<DidOrPlaceholder, Object>,
  did: &CoreDID,
) -> MethodRef<CoreDID, Object> {
  match method_ref {
    MethodRef::Embed(method) => MethodRef::Embed(reverse_transform_method(method, did)),
    MethodRef::Refer(reference) => MethodRef::Refer(
      reference
        .clone()
        .map(|did_or_placeholder| did_or_placeholder.unwrap_or_else(|| did.clone())),
    ),
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Url;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::CoreDID;
  use identity_did::verification::MethodScope;

  use crate::DidOrPlaceholder;
  use crate::StardustDocument;
  use crate::StateMetadataDocument;

  #[test]
  fn test_transformation_roundtrip() {
    let did = CoreDID::parse("did:stardust:0xffffffffffffffffffffffffffffffff").unwrap();
    let example_did = CoreDID::parse("did:example:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    let key_did = CoreDID::parse("did:key:0xf43958394883939484848484").unwrap();

    let mut document: StardustDocument = StardustDocument::new();
    document.tmp_set_id(did.clone());

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
        identity_did::service::ServiceEndpoint::One(Url::parse("https://example.com/0xf4c42e9da").unwrap()),
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
      DidOrPlaceholder::Placeholder
    ));

    assert!(matches!(
      state_metadata_doc
        .0
        .resolve_method("#did-foreign", None)
        .unwrap()
        .id()
        .did(),
      DidOrPlaceholder::Core(did) if did == &example_did
    ));

    assert!(matches!(
      state_metadata_doc
        .0
        .service().iter().find(|service| service.id().fragment().unwrap() == "my-foreign-service")
        .unwrap()
        .id()
        .did(),
      DidOrPlaceholder::Core(did) if did == &key_did
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
      DidOrPlaceholder::Placeholder
    ));

    let stardust_document = state_metadata_doc.to_stardust_document(&did);

    println!("stardust_document\n{}", stardust_document.to_json_pretty().unwrap());

    assert_eq!(stardust_document, document);
  }
}
