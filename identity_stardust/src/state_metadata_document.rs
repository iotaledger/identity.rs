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

impl From<StardustDocument> for StateMetadataDocument {
  fn from(doc: StardustDocument) -> Self {
    // TODO: Can we avoid all the clones? Not really unless we make the CoreDocument fields pub.
    // We could at least sometimes use mem swap in cases where we can get access to a &mut ref.
    let mut builder: DocumentBuilder<DidOrPlaceholder, _, _, _> =
      CoreDocument::builder(doc.0.properties().clone()).id(DidOrPlaceholder::Placeholder);

    if let Some(controllers) = doc.0.controller() {
      for controller in controllers.iter() {
        builder = builder.controller(DidOrPlaceholder::from(controller.clone()));
      }
    }

    for aka in doc.0.also_known_as().iter() {
      builder = builder.also_known_as(aka.clone());
    }

    for method in doc.0.verification_method().iter() {
      let verification_method: VerificationMethod<DidOrPlaceholder, Object> = transform_method(method.clone());
      builder = builder.verification_method(verification_method);
    }
    for method_ref in doc.0.assertion_method().iter() {
      builder = builder.assertion_method(transfrom_method_ref(method_ref.clone()));
    }
    for method_ref in doc.0.capability_delegation().iter() {
      builder = builder.capability_delegation(transfrom_method_ref(method_ref.clone()));
    }
    for method_ref in doc.0.capability_invocation().iter() {
      builder = builder.capability_invocation(transfrom_method_ref(method_ref.clone()));
    }
    for method_ref in doc.0.key_agreement().iter() {
      builder = builder.key_agreement(transfrom_method_ref(method_ref.clone()));
    }
    for method_ref in doc.0.authentication().iter() {
      builder = builder.authentication(transfrom_method_ref(method_ref.clone()));
    }

    for service in doc.0.service().iter() {
      builder = builder.service(service.clone().map(DidOrPlaceholder::from));
    }

    let core_doc: CoreDocument<DidOrPlaceholder, _, _, _> =
      builder.build().expect("all required properties should be set");

    StateMetadataDocument(core_doc)
  }
}

fn transform_method(method: VerificationMethod<CoreDID, Object>) -> VerificationMethod<DidOrPlaceholder, Object> {
  let mut builder = VerificationMethod::<DidOrPlaceholder, Object>::builder(method.properties().clone())
    .type_(method.type_())
    .data(method.data().clone());

  builder = builder.controller(DidOrPlaceholder::from(method.controller().clone()));
  let id: DIDUrl<DidOrPlaceholder> = DIDUrl::from(method.id().clone());
  builder = builder.id(id);

  builder.build().expect("all properties should be set")
}

fn transfrom_method_ref(method_ref: MethodRef<CoreDID, Object>) -> MethodRef<DidOrPlaceholder, Object> {
  match method_ref {
    MethodRef::Embed(method) => MethodRef::Embed(transform_method(method)),
    MethodRef::Refer(reference) => MethodRef::Refer(DIDUrl::from(reference)),
  }
}
