// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

use crate::did::CoreDIDUrl;
use crate::document::CoreDocument;
use crate::service::Service;
use crate::verification::MethodRef;
use crate::verification::VerificationMethod;

/// A resource returned from a [DID URL dereferencing][SPEC] process.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#dfn-did-url-dereferencing
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Resource {
  /// A dereferenced primary resource.
  Primary(PrimaryResource),
  /// A dereferenced secondary resource.
  Secondary(SecondaryResource),
}

impl From<PrimaryResource> for Resource {
  fn from(other: PrimaryResource) -> Self {
    Self::Primary(other)
  }
}

impl From<SecondaryResource> for Resource {
  fn from(other: SecondaryResource) -> Self {
    Self::Secondary(other)
  }
}

/// A primary resource returned from a [DID URL dereferencing][SPEC] process.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#dfn-did-url-dereferencing
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)] // temporary fix until the resolver gets refactored
#[serde(untagged)]
pub enum PrimaryResource {
  /// A dereferenced DID Document.
  Document(CoreDocument),
  /// A dereferenced DID Document service endpoint.
  Service(Url),
}

impl From<CoreDocument> for PrimaryResource {
  fn from(other: CoreDocument) -> Self {
    Self::Document(other)
  }
}

impl From<Url> for PrimaryResource {
  fn from(other: Url) -> Self {
    Self::Service(other)
  }
}

/// A secondary resource returned from a [DID URL dereferencing][SPEC] process.
///
/// [SPEC]: https://www.w3.org/TR/did-core/#dfn-did-url-dereferencing
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SecondaryResource {
  /// A DID Document Method Id.
  VerificationDID(CoreDIDUrl),
  /// A DID Document Verification Method.
  VerificationKey(VerificationMethod),
  /// A DID Document Service.
  Service(Service),
}

impl From<CoreDIDUrl> for SecondaryResource {
  fn from(other: CoreDIDUrl) -> Self {
    Self::VerificationDID(other)
  }
}

impl From<VerificationMethod> for SecondaryResource {
  fn from(other: VerificationMethod) -> Self {
    Self::VerificationKey(other)
  }
}

impl From<MethodRef> for SecondaryResource {
  fn from(other: MethodRef) -> Self {
    match other {
      MethodRef::Refer(inner) => inner.into(),
      MethodRef::Embed(inner) => inner.into(),
    }
  }
}

impl From<Service> for SecondaryResource {
  fn from(other: Service) -> Self {
    Self::Service(other)
  }
}
