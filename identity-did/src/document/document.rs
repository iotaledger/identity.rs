// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto as _;
use core::fmt::Display;
use core::fmt::Error as FmtError;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::convert::ToJson;
use serde::Serialize;

use crate::did::DID;
use crate::document::DocumentBuilder;
use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::DIDKey;
use crate::utils::OrderedSet;
use crate::verification::Method;
use crate::verification::MethodQuery;
use crate::verification::MethodRef;
use crate::verification::MethodScope;
use crate::verification::MethodWrap;

/// A DID Document
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[rustfmt::skip]
pub struct Document<T = Object, U = Object, V = Object> {
  pub(crate) id: DID,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) controller: Option<DID>,
  #[serde(default = "Default::default", rename = "alsoKnownAs", skip_serializing_if = "Vec::is_empty")]
  pub(crate) also_known_as: Vec<Url>,
  #[serde(default = "Default::default", rename = "verificationMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) verification_method: OrderedSet<DIDKey<Method<U>>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) authentication: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "assertionMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) assertion_method: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "keyAgreement", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) key_agreement: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "capabilityDelegation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_delegation: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", rename = "capabilityInvocation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_invocation: OrderedSet<DIDKey<MethodRef<U>>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) service: OrderedSet<DIDKey<Service<V>>>,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<T, U, V> Document<T, U, V> {
  /// Creates a `DocumentBuilder` to configure a new `Document`.
  ///
  /// This is the same as `DocumentBuilder::new()`.
  pub fn builder(properties: T) -> DocumentBuilder<T, U, V> {
    DocumentBuilder::new(properties)
  }

  /// Returns a new `Document` based on the `DocumentBuilder` configuration.
  pub fn from_builder(builder: DocumentBuilder<T, U, V>) -> Result<Self> {
    Ok(Self {
      id: builder.id.ok_or(Error::BuilderInvalidDocumentId)?,
      controller: builder.controller,
      also_known_as: builder.also_known_as,
      verification_method: builder.verification_method.try_into()?,
      authentication: builder.authentication.try_into()?,
      assertion_method: builder.assertion_method.try_into()?,
      key_agreement: builder.key_agreement.try_into()?,
      capability_delegation: builder.capability_delegation.try_into()?,
      capability_invocation: builder.capability_invocation.try_into()?,
      service: builder.service.try_into()?,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `Document` id.
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Returns a mutable reference to the `Document` id.
  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  /// Returns a reference to the `Document` controller.
  pub fn controller(&self) -> Option<&DID> {
    self.controller.as_ref()
  }

  /// Returns a mutable reference to the `Document` controller.
  pub fn controller_mut(&mut self) -> Option<&mut DID> {
    self.controller.as_mut()
  }

  /// Returns a reference to the `Document` alsoKnownAs set.
  pub fn also_known_as(&self) -> &[Url] {
    &self.also_known_as
  }

  /// Returns a mutable reference to the `Document` alsoKnownAs set.
  pub fn also_known_as_mut(&mut self) -> &mut Vec<Url> {
    &mut self.also_known_as
  }

  /// Returns a reference to the `Document` verificationMethod set.
  pub fn verification_method(&self) -> &OrderedSet<DIDKey<Method<U>>> {
    &self.verification_method
  }

  /// Returns a mutable reference to the `Document` verificationMethod set.
  pub fn verification_method_mut(&mut self) -> &mut OrderedSet<DIDKey<Method<U>>> {
    &mut self.verification_method
  }

  /// Returns a reference to the `Document` authentication set.
  pub fn authentication(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.authentication
  }

  /// Returns a mutable reference to the `Document` authentication set.
  pub fn authentication_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.authentication
  }

  /// Returns a reference to the `Document` assertionMethod set.
  pub fn assertion_method(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.assertion_method
  }

  /// Returns a mutable reference to the `Document` assertionMethod set.
  pub fn assertion_method_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.assertion_method
  }

  /// Returns a reference to the `Document` keyAgreement set.
  pub fn key_agreement(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.key_agreement
  }

  /// Returns a mutable reference to the `Document` keyAgreement set.
  pub fn key_agreement_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.key_agreement
  }

  /// Returns a reference to the `Document` capabilityDelegation set.
  pub fn capability_delegation(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.capability_delegation
  }

  /// Returns a mutable reference to the `Document` capabilityDelegation set.
  pub fn capability_delegation_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.capability_delegation
  }

  /// Returns a reference to the `Document` capabilityInvocation set.
  pub fn capability_invocation(&self) -> &OrderedSet<DIDKey<MethodRef<U>>> {
    &self.capability_invocation
  }

  /// Returns a mutable reference to the `Document` capabilityInvocation set.
  pub fn capability_invocation_mut(&mut self) -> &mut OrderedSet<DIDKey<MethodRef<U>>> {
    &mut self.capability_invocation
  }

  /// Returns a reference to the `Document` service set.
  pub fn service(&self) -> &OrderedSet<DIDKey<Service<V>>> {
    &self.service
  }

  /// Returns a mutable reference to the `Document` service set.
  pub fn service_mut(&mut self) -> &mut OrderedSet<DIDKey<Service<V>>> {
    &mut self.service
  }

  /// Returns a reference to the custom `Document` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom `Document` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  /// Maps `Document<T>` to `Document<U>` by applying a function to the custom
  /// properties.
  pub fn map<A, F>(self, f: F) -> Document<A, U, V>
  where
    F: FnOnce(T) -> A,
  {
    Document {
      id: self.id,
      controller: self.controller,
      also_known_as: self.also_known_as,
      verification_method: self.verification_method,
      authentication: self.authentication,
      assertion_method: self.assertion_method,
      key_agreement: self.key_agreement,
      capability_delegation: self.capability_delegation,
      capability_invocation: self.capability_invocation,
      service: self.service,
      properties: f(self.properties),
    }
  }

  /// A fallible version of `Document::map(..)`.
  ///
  /// # Errors
  ///
  /// `try_map` can fail if the provided function fails.
  pub fn try_map<A, F, E>(self, f: F) -> Result<Document<A, U, V>, E>
  where
    F: FnOnce(T) -> Result<A, E>,
  {
    Ok(Document {
      id: self.id,
      controller: self.controller,
      also_known_as: self.also_known_as,
      verification_method: self.verification_method,
      authentication: self.authentication,
      assertion_method: self.assertion_method,
      key_agreement: self.key_agreement,
      capability_delegation: self.capability_delegation,
      capability_invocation: self.capability_invocation,
      service: self.service,
      properties: f(self.properties)?,
    })
  }

  /// Adds a new [`Method<U>`][`Method`] to the Document.
  pub fn insert_method(&mut self, scope: MethodScope, method: Method<U>) -> Result<bool> {
    macro_rules! mref {
      ($method:expr) => {
        MethodRef::Refer($method.id().clone()).into()
      };
    }

    let mut inserted: bool = false;

    match scope {
      MethodScope::None => {
        // Ensure the method doesn't already exist
        if self.resolve(method.id().as_str()).is_some() {
          return Err(Error::InvalidMethodDuplicate);
        }

        // Add everywhere
        inserted &= self.authentication.append(mref!(method));
        inserted &= self.assertion_method.append(mref!(method));
        inserted &= self.key_agreement.append(mref!(method));
        inserted &= self.capability_delegation.append(mref!(method));
        inserted &= self.capability_invocation.append(mref!(method));
        inserted &= self.verification_method.append(method.into());
      }
      MethodScope::VerificationMethod => {
        inserted |= self.verification_method.append(method.into());
      }
      MethodScope::Authentication => {
        inserted |= self.authentication.append(MethodRef::Embed(method).into());
      }
      MethodScope::AssertionMethod => {
        inserted |= self.assertion_method.append(MethodRef::Embed(method).into());
      }
      MethodScope::KeyAgreement => {
        inserted |= self.key_agreement.append(MethodRef::Embed(method).into());
      }
      MethodScope::CapabilityDelegation => {
        inserted |= self.capability_delegation.append(MethodRef::Embed(method).into());
      }
      MethodScope::CapabilityInvocation => {
        inserted |= self.capability_invocation.append(MethodRef::Embed(method).into());
      }
    }

    Ok(inserted)
  }

  /// Removes all references to the specified [`Method<U>`][`Method`].
  pub fn remove_method(&mut self, did: &DID) {
    self.authentication.remove(did);
    self.assertion_method.remove(did);
    self.key_agreement.remove(did);
    self.capability_delegation.remove(did);
    self.capability_invocation.remove(did);
    self.verification_method.remove(did);
  }

  /// Finds and returns the first verification [`method`][`Method`]
  /// matching the provided [`query`][`MethodQuery`].
  pub fn resolve<'a, Q>(&self, query: Q) -> Option<MethodWrap<'_, U>>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self.resolve_method(query.into())
  }

  /// Finds and returns the first verification [`method`][`Method`]
  /// matching the provided [`query`][`MethodQuery`].
  ///
  /// # Errors
  ///
  /// Fails if no matching verification `Method` is found.
  pub fn try_resolve<'a, Q>(&self, query: Q) -> Result<MethodWrap<'_, U>>
  where
    Q: Into<MethodQuery<'a>>,
  {
    self.resolve(query).ok_or(Error::QueryMethodNotFound)
  }

  fn resolve_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, U>> {
    let iter: _ = match query.scope {
      MethodScope::None => return self.resolve_any_method(query),
      MethodScope::VerificationMethod => return self.resolve_verification_method(query),
      MethodScope::Authentication => self.authentication.iter(),
      MethodScope::AssertionMethod => self.assertion_method.iter(),
      MethodScope::KeyAgreement => self.key_agreement.iter(),
      MethodScope::CapabilityDelegation => self.capability_delegation.iter(),
      MethodScope::CapabilityInvocation => self.capability_invocation.iter(),
    };

    self.resolve_method_iter(query, iter)
  }

  fn resolve_method_iter<'a>(
    &'a self,
    query: MethodQuery<'_>,
    iter: impl Iterator<Item = &'a DIDKey<MethodRef<U>>>,
  ) -> Option<MethodWrap<'a, U>> {
    iter
      .enumerate()
      .find(|(index, method)| query.ident == *index || query.ident.matches(method.id()))
      .and_then(|(index, method)| match method.as_ref() {
        MethodRef::Refer(did) => self.resolve(MethodQuery::with_scope(
          did.fragment()?,
          MethodScope::VerificationMethod,
        )),
        MethodRef::Embed(method) => Some(MethodWrap::new(method, index, query.scope)),
      })
  }

  fn resolve_verification_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, U>> {
    self
      .verification_method
      .iter()
      .enumerate()
      .find(|(index, method)| query.ident == *index || query.ident.matches(method.id()))
      .map(|(index, method)| MethodWrap::new(method, index, MethodScope::VerificationMethod))
  }

  fn resolve_any_method(&self, query: MethodQuery<'_>) -> Option<MethodWrap<'_, U>> {
    macro_rules! try_iter {
      ($this:expr, $query:expr, $scope:ident, $iter:ident) => {{
        let query: MethodQuery<'_> = $query.scoped(MethodScope::$scope);

        if let Some(method) = $this.resolve_method_iter(query, $this.$iter.iter()) {
          return Some(method);
        }
      }};
    }

    if let Some(method) = self.resolve_verification_method(query) {
      return Some(method);
    }

    try_iter!(self, query, Authentication, authentication);
    try_iter!(self, query, AssertionMethod, assertion_method);
    try_iter!(self, query, KeyAgreement, key_agreement);
    try_iter!(self, query, CapabilityDelegation, capability_delegation);
    try_iter!(self, query, CapabilityInvocation, capability_invocation);

    None
  }
}

impl<T, U, V> Display for Document<T, U, V>
where
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    if f.alternate() {
      f.write_str(&self.to_json_pretty().map_err(|_| FmtError)?)
    } else {
      f.write_str(&self.to_json().map_err(|_| FmtError)?)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::did::DID;
  use crate::document::Document;
  use crate::document::DocumentBuilder;
  use crate::verification::Method;
  use crate::verification::MethodBuilder;
  use crate::verification::MethodData;
  use crate::verification::MethodScope;
  use crate::verification::MethodType;

  fn controller() -> DID {
    "did:example:1234".parse().unwrap()
  }

  fn method(controller: &DID, fragment: &str) -> Method {
    MethodBuilder::default()
      .id(controller.join(fragment).unwrap())
      .controller(controller.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_b58(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn document() -> Document {
    let controller: DID = controller();

    DocumentBuilder::default()
      .id(controller.clone())
      .verification_method(method(&controller, "#key-1"))
      .verification_method(method(&controller, "#key-2"))
      .verification_method(method(&controller, "#key-3"))
      .authentication(method(&controller, "#auth-key"))
      .authentication(controller.join("#key-3").unwrap())
      .key_agreement(controller.join("#key-4").unwrap())
      .build()
      .unwrap()
  }

  #[test]
  #[rustfmt::skip]
  fn test_resolve_fragment_identifier() {
    let document: Document = document();

    // Resolve methods by fragment using the default scope (VerificationMethod)
    assert_eq!(document.resolve("#key-1").unwrap().id(), "did:example:1234#key-1");
    assert_eq!(document.resolve("#key-2").unwrap().id(), "did:example:1234#key-2");
    assert_eq!(document.resolve("#key-3").unwrap().id(), "did:example:1234#key-3");

    // Perfect fine to omit the octothorpe
    assert_eq!(document.resolve("key-1").unwrap().id(), "did:example:1234#key-1");
    assert_eq!(document.resolve("key-2").unwrap().id(), "did:example:1234#key-2");
    assert_eq!(document.resolve("key-3").unwrap().id(), "did:example:1234#key-3");
  }

  #[test]
  #[rustfmt::skip]
  fn test_resolve_index_identifier() {
    let document: Document = document();

    // Resolve methods by index using the default scope once again
    assert_eq!(document.resolve(0).unwrap().id(), "did:example:1234#key-1");
    assert_eq!(document.resolve(2).unwrap().id(), "did:example:1234#key-3");
  }

  #[test]
  #[rustfmt::skip]
  fn test_resolve_explicit_scope() {
    let document: Document = document();

    // Resolve methods by fragment using explicit scopes
    assert_eq!(document.resolve(("#key-1", MethodScope::KeyAgreement)), None);
    assert_eq!(document.resolve(("#key-2", MethodScope::VerificationMethod)).unwrap().id(), "did:example:1234#key-2");
  }

  #[test]
  #[rustfmt::skip]
  fn test_resolve_reference_found() {
    let document: Document = document();

    // Resolving a method reference returns the method object
    let resolved_ref = document.resolve(("#key-3", MethodScope::Authentication)).unwrap();
    let resolved_obj = document.resolve(("#key-3", MethodScope::VerificationMethod)).unwrap();

    assert_eq!(resolved_ref.index(), 2);
    assert_eq!(resolved_ref.scope(), MethodScope::VerificationMethod);

    // The resolved methods should be identical
    assert_eq!(&*resolved_ref, &*resolved_obj);
  }

  #[test]
  #[rustfmt::skip]
  fn test_resolve_reference_missing() {
    let document: Document = document();

    // Resolving an existing reference to a missing method returns None
    assert_eq!(document.resolve(("#key-4", MethodScope::KeyAgreement)), None);
  }
}
