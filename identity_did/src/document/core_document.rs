// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto as _;
use core::fmt::Display;
use core::fmt::Formatter;
use std::collections::HashMap;

use serde::Serialize;

use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_core::crypto::Ed25519;
use identity_core::crypto::GetSignature;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::Proof;
use identity_core::crypto::ProofPurpose;
use identity_core::crypto::Verifier;
use serde::Serializer;

use crate::did::CoreDID;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::document::Document;
use crate::document::DocumentBuilder;
use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::DIDUrlQuery;
use crate::utils::Queryable;
use crate::verifiable::DocumentSigner;
use crate::verifiable::VerifierOptions;
use crate::verification::MethodRef;
use crate::verification::MethodRelationship;
use crate::verification::MethodScope;
use crate::verification::MethodType;
use crate::verification::MethodUriType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[rustfmt::skip]
pub(crate) struct CoreDocumentData<D = CoreDID, T = Object, U = Object, V = Object>
  where
    D: DID + KeyComparable
{
  pub(crate) id: D,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) controller: Option<OneOrSet<D>>,
  #[serde(default = "Default::default", rename = "alsoKnownAs", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) also_known_as: OrderedSet<Url>,
  #[serde(default = "Default::default", rename = "verificationMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) verification_method: OrderedSet<VerificationMethod<D, U>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) authentication: OrderedSet<MethodRef<D, U>>,
  #[serde(default = "Default::default", rename = "assertionMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) assertion_method: OrderedSet<MethodRef<D, U>>,
  #[serde(default = "Default::default", rename = "keyAgreement", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) key_agreement: OrderedSet<MethodRef<D, U>>,
  #[serde(default = "Default::default", rename = "capabilityDelegation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_delegation: OrderedSet<MethodRef<D, U>>,
  #[serde(default = "Default::default", rename = "capabilityInvocation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_invocation: OrderedSet<MethodRef<D, U>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) service: OrderedSet<Service<D, V>>,
  #[serde(flatten)]
  pub(crate) properties: T,
}

impl<D: DID + KeyComparable, T, U, V> CoreDocumentData<D, T, U, V> {
  /// Checks the following:
  /// - There are no scoped method references to an embedded method in the document
  /// - The ids of verification methods (scoped/embedded or general purpose) and services are unique across the
  ///   document.
  fn check_id_constraints(&self) -> Result<()> {
    let max_unique_method_ids = self.verification_method.len()
      + self.authentication.len()
      + self.assertion_method.len()
      + self.key_agreement.len()
      + self.capability_delegation.len()
      + self.capability_invocation.len();

    // Value = true => the identifier belongs to an embedded method, false means it belongs to a method reference or a
    // general purpose verification method
    let mut method_identifiers: HashMap<&DIDUrl<D>, bool> = HashMap::with_capacity(max_unique_method_ids);

    for (id, is_embedded) in self
      .authentication
      .iter()
      .chain(self.assertion_method.iter())
      .chain(self.key_agreement.iter())
      .chain(self.capability_delegation.iter())
      .chain(self.capability_invocation.iter())
      .map(|method_ref| match method_ref {
        MethodRef::Embed(_) => (method_ref.id(), true),
        MethodRef::Refer(_) => (method_ref.id(), false),
      })
    {
      if let Some(previous) = method_identifiers.insert(id, is_embedded) {
        match previous {
          // An embedded method with the same id has previously been encountered
          true => {
            return Err(Error::InvalidDocument(
              "attempted to construct document with a duplicated or aliased embedded method",
              None,
            ));
          }
          // A method reference to the identifier has previously been encountered
          false => {
            if is_embedded {
              return Err(Error::InvalidDocument(
                "attempted to construct document with an aliased embedded method",
                None,
              ));
            }
          }
        }
      }
    }

    for method_id in self.verification_method.iter().map(|method| method.id()) {
      if method_identifiers
        .insert(method_id, false)
        .filter(|value| *value)
        .is_some()
      {
        return Err(Error::InvalidDocument(
          "attempted to construct document with a duplicated embedded method",
          None,
        ));
      }
    }

    for service_id in self.service.iter().map(|service| service.id()) {
      if method_identifiers.contains_key(service_id) {
        return Err(Error::InvalidDocument(
          "attempted to construct document with a service identifier shared with a verification method",
          None,
        ));
      }
    }

    Ok(())
  }
}

/// A DID Document.
///
/// [Specification](https://www.w3.org/TR/did-core/#did-document-properties)
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[rustfmt::skip]
#[serde(try_from = "CoreDocumentData<D,T,U,V>")]
pub struct CoreDocument<D = CoreDID, T = Object, U = Object, V = Object>
  where
    D: DID + KeyComparable
{
  pub(crate) data: CoreDocumentData<D,T,U,V>, 
}

//Forward serialization to inner
impl<D, T, U, V> Serialize for CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable + Serialize,
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.data.serialize(serializer)
  }
}

// Workaround for lifetime issues with a mutable reference to self preventing closures from being used.
macro_rules! method_ref_mut_helper {
  ($doc:ident, $method: ident, $query: ident) => {
    match $doc.data.$method.query_mut($query.into())? {
      MethodRef::Embed(method) => Some(method),
      MethodRef::Refer(ref did) => $doc.data.verification_method.query_mut(did),
    }
  };
}

impl<D, T, U, V> CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  /// Creates a [`DocumentBuilder`] to configure a new `CoreDocument`.
  ///
  /// This is the same as [`DocumentBuilder::new`].
  pub fn builder(properties: T) -> DocumentBuilder<D, T, U, V> {
    DocumentBuilder::new(properties)
  }

  /// Returns a new `CoreDocument` based on the [`DocumentBuilder`] configuration.
  pub fn from_builder(builder: DocumentBuilder<D, T, U, V>) -> Result<Self> {
    Self::try_from(CoreDocumentData {
      id: builder.id.ok_or(Error::InvalidDocument("missing id", None))?,
      controller: Some(builder.controller)
        .filter(|controllers| !controllers.is_empty())
        .map(TryFrom::try_from)
        .transpose()
        .map_err(|err| Error::InvalidDocument("controller", Some(err)))?,
      also_known_as: builder
        .also_known_as
        .try_into()
        .map_err(|err| Error::InvalidDocument("also_known_as", Some(err)))?,
      verification_method: builder
        .verification_method
        .try_into()
        .map_err(|err| Error::InvalidDocument("verification_method", Some(err)))?,
      authentication: builder
        .authentication
        .try_into()
        .map_err(|err| Error::InvalidDocument("authentication", Some(err)))?,
      assertion_method: builder
        .assertion_method
        .try_into()
        .map_err(|err| Error::InvalidDocument("assertion_method", Some(err)))?,
      key_agreement: builder
        .key_agreement
        .try_into()
        .map_err(|err| Error::InvalidDocument("key_agreement", Some(err)))?,
      capability_delegation: builder
        .capability_delegation
        .try_into()
        .map_err(|err| Error::InvalidDocument("capability_delegation", Some(err)))?,
      capability_invocation: builder
        .capability_invocation
        .try_into()
        .map_err(|err| Error::InvalidDocument("capability_invocation", Some(err)))?,
      service: builder
        .service
        .try_into()
        .map_err(|err| Error::InvalidDocument("service", Some(err)))?,
      properties: builder.properties,
    })
  }

  /// Returns a reference to the `CoreDocument` id.
  pub fn id(&self) -> &D {
    &self.data.id
  }

  /// Returns a mutable reference to the `CoreDocument` id.
  ///
  /// # Warning
  /// Incorrect use of this method can lead to broken URI dereferencing.
  pub fn id_mut_unchecked(&mut self) -> &mut D {
    &mut self.data.id
  }

  /// Returns a reference to the `CoreDocument` controller.
  pub fn controller(&self) -> Option<&OneOrSet<D>> {
    self.data.controller.as_ref()
  }

  /// Returns a mutable reference to the `CoreDocument` controller.
  pub fn controller_mut(&mut self) -> &mut Option<OneOrSet<D>> {
    &mut self.data.controller
  }

  /// Returns a reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as(&self) -> &OrderedSet<Url> {
    &self.data.also_known_as
  }

  /// Returns a mutable reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as_mut(&mut self) -> &mut OrderedSet<Url> {
    &mut self.data.also_known_as
  }

  /// Returns a reference to the `CoreDocument` verificationMethod set.
  pub fn verification_method(&self) -> &OrderedSet<VerificationMethod<D, U>> {
    &self.data.verification_method
  }

  /// Returns a reference to the `CoreDocument` authentication set.
  pub fn authentication(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.data.authentication
  }

  /// Returns a reference to the `CoreDocument` assertionMethod set.
  pub fn assertion_method(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.data.assertion_method
  }

  /// Returns a reference to the `CoreDocument` keyAgreement set.
  pub fn key_agreement(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.data.key_agreement
  }

  /// Returns a reference to the `CoreDocument` capabilityDelegation set.
  pub fn capability_delegation(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.data.capability_delegation
  }

  /// Returns a reference to the `CoreDocument` capabilityInvocation set.
  pub fn capability_invocation(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.data.capability_invocation
  }

  /// Returns a reference to the `CoreDocument` service set.
  pub fn service(&self) -> &OrderedSet<Service<D, V>> {
    &self.data.service
  }

  /// Returns a reference to the custom `CoreDocument` properties.
  pub fn properties(&self) -> &T {
    &self.data.properties
  }

  /// Returns a mutable reference to the custom `CoreDocument` properties.
  /// # Warning
  /// Incorrect use of this method can lead to broken invariants.
  pub fn properties_mut_unchecked(&mut self) -> &mut T {
    &mut self.data.properties
  }

  /// Maps `CoreDocument<D,T>` to `CoreDocument<C,U>` by applying a function `f` to all [`DID`] fields
  /// and another function `g` to the custom properties.
  ///
  /// # Warning
  /// Can lead to broken invariants if used incorrectly. See [`Self::try_map`](CoreDocument::try_map()) for a fallible
  /// version with additional built-in checks.
  pub fn map_unchecked<S, C, F, G>(self, mut f: F, g: G) -> CoreDocument<C, S, U, V>
  where
    C: DID + KeyComparable,
    F: FnMut(D) -> C,
    G: FnOnce(T) -> S,
  {
    let current_inner = self.data;
    CoreDocument::<C, S, U, V> {
      data: CoreDocumentData {
        id: f(current_inner.id),
        controller: current_inner
          .controller
          .map(|controller_set| controller_set.map(&mut f)),
        also_known_as: current_inner.also_known_as,
        verification_method: current_inner
          .verification_method
          .into_iter()
          .map(|method| method.map(&mut f))
          .collect(),
        authentication: current_inner
          .authentication
          .into_iter()
          .map(|method_ref| method_ref.map(&mut f))
          .collect(),
        assertion_method: current_inner
          .assertion_method
          .into_iter()
          .map(|method_ref| method_ref.map(&mut f))
          .collect(),
        key_agreement: current_inner
          .key_agreement
          .into_iter()
          .map(|method_ref| method_ref.map(&mut f))
          .collect(),
        capability_delegation: current_inner
          .capability_delegation
          .into_iter()
          .map(|method_ref| method_ref.map(&mut f))
          .collect(),
        capability_invocation: current_inner
          .capability_invocation
          .into_iter()
          .map(|method_ref| method_ref.map(&mut f))
          .collect(),
        service: current_inner
          .service
          .into_iter()
          .map(|service| service.map(&mut f))
          .collect(),
        properties: g(current_inner.properties),
      },
    }
  }

  /// Fallible version of [`CoreDocument::map_unchecked`] with additional identifier uniqueness checks.
  ///
  /// # Errors
  ///
  /// `try_map` can fail if either of the provided functions fail or if the mapping `f`
  /// introduces methods referencing embedded method identifiers, or services with identifiers matching method
  /// identifiers. In the case of the latter the provided function `h` will be called to construct a compatible error.
  pub fn try_map<S, C, F, G, E, H>(self, mut f: F, g: G, h: H) -> std::result::Result<CoreDocument<C, S, U, V>, E>
  where
    C: DID + KeyComparable,
    F: FnMut(D) -> Result<C, E>,
    G: FnOnce(T) -> Result<S, E>,
    H: FnOnce(crate::error::Error) -> E,
    E: std::error::Error,
  {
    let current_inner = self.data;
    let helper = || -> Result<CoreDocumentData<C, S, U, V>, E> {
      Ok(CoreDocumentData {
        id: f(current_inner.id)?,
        controller: current_inner
          .controller
          .map(|controller_set| controller_set.try_map(&mut f))
          .transpose()?,
        also_known_as: current_inner.also_known_as,
        verification_method: current_inner
          .verification_method
          .into_iter()
          .map(|method| method.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        authentication: current_inner
          .authentication
          .into_iter()
          .map(|method_ref| method_ref.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        assertion_method: current_inner
          .assertion_method
          .into_iter()
          .map(|method_ref| method_ref.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        key_agreement: current_inner
          .key_agreement
          .into_iter()
          .map(|method_ref| method_ref.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        capability_delegation: current_inner
          .capability_delegation
          .into_iter()
          .map(|method_ref| method_ref.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        capability_invocation: current_inner
          .capability_invocation
          .into_iter()
          .map(|method_ref| method_ref.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        service: current_inner
          .service
          .into_iter()
          .map(|service| service.try_map(&mut f))
          .collect::<Result<_, E>>()?,
        properties: g(current_inner.properties)?,
      })
    };
    helper().and_then(|data| CoreDocument::try_from(data).map_err(h))
  }

  /// Adds a new [`VerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method or service with the same fragment already exists.
  pub fn insert_method(&mut self, method: VerificationMethod<D, U>, scope: MethodScope) -> Result<()> {
    // check that the method identifier is not already in use by an existing method or service.
    if self.resolve_method(method.id(), None).is_some() || self.service().query(method.id()).is_some() {
      return Err(Error::MethodInsertionError);
    }
    match scope {
      MethodScope::VerificationMethod => self.data.verification_method.append(method),
      MethodScope::VerificationRelationship(MethodRelationship::Authentication) => {
        self.data.authentication.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => {
        self.data.assertion_method.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => {
        self.data.key_agreement.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => {
        self.data.capability_delegation.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => {
        self.data.capability_invocation.append(MethodRef::Embed(method))
      }
    };

    Ok(())
  }

  /// Removes and returns the [`VerificationMethod`] from the document.
  ///
  /// # Note
  ///
  /// All _references to the method_ found in the document will be removed.
  /// This includes cases where the reference is to a method contained in another DID document.
  pub fn remove_method(&mut self, did: &DIDUrl<D>) -> Option<VerificationMethod<D, U>> {
    for method_ref in [
      self.data.authentication.remove(did),
      self.data.assertion_method.remove(did),
      self.data.key_agreement.remove(did),
      self.data.capability_delegation.remove(did),
      self.data.capability_invocation.remove(did),
    ]
    .into_iter()
    .flatten()
    {
      if let MethodRef::<D, U>::Embed(embedded_method) = method_ref {
        // embedded methods cannot be referenced, or be in the set of general purpose verification methods hence the
        // search is complete
        return Some(embedded_method);
      }
    }

    self.data.verification_method.remove(did)
  }

  /// Adds a new [`Service`] to the document.
  ///
  /// # Errors
  ///
  /// Returns an error if there already exists a service or verification method with the same identifier.
  pub fn insert_service(&mut self, service: Service<D, V>) -> Result<()> {
    let service_id = service.id();
    let id_shared_with_method = self
      .verification_relationships()
      .map(|method_ref| method_ref.id())
      .chain(self.verification_method().iter().map(|method| method.id()))
      .any(|id| id == service_id);

    ((!id_shared_with_method) && self.data.service.append(service))
      .then_some(())
      .ok_or(Error::InvalidServiceInsertion)
  }

  /// Removes and returns a [`Service`] from the document if it exists.
  pub fn remove_service(&mut self, id: &DIDUrl<D>) -> Option<Service<D, V>> {
    self.data.service.remove(id)
  }
  /// Attaches the relationship to the method resolved by `method_query`.
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist or if it is embedded.
  /// To convert an embedded method into a generic verification method, remove it first
  /// and insert it with [`MethodScope::VerificationMethod`].
  pub fn attach_method_relationship<'query, Q>(
    &mut self,
    method_query: Q,
    relationship: MethodRelationship,
  ) -> Result<bool>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    let method_query: DIDUrlQuery<'query> = method_query.into();

    match self.resolve_method(method_query.clone(), Some(MethodScope::VerificationMethod)) {
      None => match self.resolve_method(method_query, None) {
        Some(_) => Err(Error::InvalidMethodEmbedded),
        None => Err(Error::MethodNotFound),
      },
      Some(method) => {
        let method_ref = MethodRef::Refer(method.id().clone());

        let was_attached = match relationship {
          MethodRelationship::Authentication => self.data.authentication.append(method_ref),
          MethodRelationship::AssertionMethod => self.data.assertion_method.append(method_ref),
          MethodRelationship::KeyAgreement => self.data.key_agreement.append(method_ref),
          MethodRelationship::CapabilityDelegation => self.data.capability_delegation.append(method_ref),
          MethodRelationship::CapabilityInvocation => self.data.capability_invocation.append(method_ref),
        };

        Ok(was_attached)
      }
    }
  }

  /// Detaches the relationship from the method resolved by `method_query`.
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist or is embedded.
  /// To remove an embedded method, use [`Self::remove_method`].
  ///
  /// # Note
  ///
  /// If the method is referenced in the given scope, but the document does not contain the referenced verification
  /// method, then the reference will persist in the document (i.e. it is not removed).
  // TODO: Is this the behaviour we want?
  pub fn detach_method_relationship<'query, Q>(
    &mut self,
    method_query: Q,
    relationship: MethodRelationship,
  ) -> Result<bool>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    let method_query: DIDUrlQuery<'query> = method_query.into();
    match self.resolve_method(method_query.clone(), Some(MethodScope::VerificationMethod)) {
      None => match self.resolve_method(method_query, None) {
        Some(_) => Err(Error::InvalidMethodEmbedded),
        None => Err(Error::MethodNotFound),
      },
      Some(method) => {
        let did_url: DIDUrl<D> = method.id().clone();

        let was_detached = match relationship {
          MethodRelationship::Authentication => self.data.authentication.remove(&did_url),
          MethodRelationship::AssertionMethod => self.data.assertion_method.remove(&did_url),
          MethodRelationship::KeyAgreement => self.data.key_agreement.remove(&did_url),
          MethodRelationship::CapabilityDelegation => self.data.capability_delegation.remove(&did_url),
          MethodRelationship::CapabilityInvocation => self.data.capability_invocation.remove(&did_url),
        };

        Ok(was_detached.is_some())
      }
    }
  }

  /// Returns a `Vec` of verification method references whose verification relationship matches `scope`.
  ///
  /// If `scope` is `None`, an iterator over all **embedded** methods is returned.
  pub fn methods(&self, scope: Option<MethodScope>) -> Vec<&VerificationMethod<D, U>>
  where
    D: DID,
  {
    if let Some(scope) = scope {
      match scope {
        MethodScope::VerificationMethod => self.verification_method().iter().collect(),
        MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => self
          .assertion_method()
          .iter()
          .filter_map(|method_ref| self.resolve_method_ref(method_ref))
          .collect(),
        MethodScope::VerificationRelationship(MethodRelationship::Authentication) => self
          .authentication()
          .iter()
          .filter_map(|method_ref| self.resolve_method_ref(method_ref))
          .collect(),
        MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => self
          .capability_delegation()
          .iter()
          .filter_map(|method_ref| self.resolve_method_ref(method_ref))
          .collect(),
        MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => self
          .capability_invocation()
          .iter()
          .filter_map(|method_ref| self.resolve_method_ref(method_ref))
          .collect(),
        MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => self
          .key_agreement()
          .iter()
          .filter_map(|method_ref| self.resolve_method_ref(method_ref))
          .collect(),
      }
    } else {
      self.all_methods().collect()
    }
  }

  /// Returns an iterator over all embedded verification methods in the DID Document.
  ///
  /// This excludes verification methods that are referenced by the DID Document.
  fn all_methods(&self) -> impl Iterator<Item = &VerificationMethod<D, U>> {
    fn __filter_ref<D, T>(method: &MethodRef<D, T>) -> Option<&VerificationMethod<D, T>>
    where
      D: DID,
    {
      match method {
        MethodRef::Embed(method) => Some(method),
        MethodRef::Refer(_) => None,
      }
    }

    self
      .data
      .verification_method
      .iter()
      .chain(self.data.authentication.iter().filter_map(__filter_ref))
      .chain(self.data.assertion_method.iter().filter_map(__filter_ref))
      .chain(self.data.key_agreement.iter().filter_map(__filter_ref))
      .chain(self.data.capability_delegation.iter().filter_map(__filter_ref))
      .chain(self.data.capability_invocation.iter().filter_map(__filter_ref))
  }

  /// Returns an iterator over all verification relationships.
  ///
  /// This includes embedded and referenced [`VerificationMethods`](VerificationMethod).
  pub fn verification_relationships(&self) -> impl Iterator<Item = &MethodRef<D, U>> {
    self
      .data
      .authentication
      .iter()
      .chain(self.data.assertion_method.iter())
      .chain(self.data.key_agreement.iter())
      .chain(self.data.capability_delegation.iter())
      .chain(self.data.capability_invocation.iter())
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the
  /// provided `query` and the verification relationship specified by `scope` if present.
  pub fn resolve_method<'query, 'me, Q>(
    &'me self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    match scope {
      Some(scope) => {
        let resolve_ref_helper = |method_ref: &'me MethodRef<D, U>| self.resolve_method_ref(method_ref);

        match scope {
          MethodScope::VerificationMethod => self.data.verification_method.query(query.into()),
          MethodScope::VerificationRelationship(MethodRelationship::Authentication) => self
            .data
            .authentication
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => self
            .data
            .assertion_method
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => {
            self.data.key_agreement.query(query.into()).and_then(resolve_ref_helper)
          }
          MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => self
            .data
            .capability_delegation
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => self
            .data
            .capability_invocation
            .query(query.into())
            .and_then(resolve_ref_helper),
        }
      }
      None => self.resolve_method_inner(query.into()),
    }
  }

  /// Returns a mutable reference to the first [`VerificationMethod`] with an `id` property
  /// matching the provided `query`.
  ///
  /// # Warning
  /// Incorrect use of this method can lead to broken URI dereferencing
  pub fn resolve_method_mut<'query, 'me, Q>(
    &'me mut self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&'me mut VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    match scope {
      Some(scope) => match scope {
        MethodScope::VerificationMethod => self.data.verification_method.query_mut(query.into()),
        MethodScope::VerificationRelationship(MethodRelationship::Authentication) => {
          method_ref_mut_helper!(self, authentication, query)
        }
        MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => {
          method_ref_mut_helper!(self, assertion_method, query)
        }
        MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => {
          method_ref_mut_helper!(self, key_agreement, query)
        }
        MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => {
          method_ref_mut_helper!(self, capability_delegation, query)
        }
        MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => {
          method_ref_mut_helper!(self, capability_invocation, query)
        }
      },
      None => self.resolve_method_mut_inner(query.into()),
    }
  }

  #[doc(hidden)]
  pub fn resolve_method_ref<'a>(&'a self, method_ref: &'a MethodRef<D, U>) -> Option<&'a VerificationMethod<D, U>> {
    match method_ref {
      MethodRef::Embed(method) => Some(method),
      MethodRef::Refer(did) => self.data.verification_method.query(did),
    }
  }

  fn resolve_method_inner(&self, query: DIDUrlQuery<'_>) -> Option<&VerificationMethod<D, U>> {
    let mut method: Option<&MethodRef<D, U>> = None;

    if method.is_none() {
      method = self.data.authentication.query(query.clone());
    }

    if method.is_none() {
      method = self.data.assertion_method.query(query.clone());
    }

    if method.is_none() {
      method = self.data.key_agreement.query(query.clone());
    }

    if method.is_none() {
      method = self.data.capability_delegation.query(query.clone());
    }

    if method.is_none() {
      method = self.data.capability_invocation.query(query.clone());
    }

    match method {
      Some(MethodRef::Embed(method)) => Some(method),
      Some(MethodRef::Refer(did)) => self.data.verification_method.query(&did.to_string()),
      None => self.data.verification_method.query(query),
    }
  }

  fn resolve_method_mut_inner(&mut self, query: DIDUrlQuery<'_>) -> Option<&mut VerificationMethod<D, U>> {
    let mut method: Option<&mut MethodRef<D, U>> = None;

    if method.is_none() {
      method = self.data.authentication.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.data.assertion_method.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.data.key_agreement.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.data.capability_delegation.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.data.capability_invocation.query_mut(query.clone());
    }

    match method {
      Some(MethodRef::Embed(method)) => Some(method),
      Some(MethodRef::Refer(did)) => self.data.verification_method.query_mut(&did.to_string()),
      None => self.data.verification_method.query_mut(query),
    }
  }
}

impl<D, T, U, V> CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  /// Verifies the signature of the provided `data` was created using a verification method
  /// in this DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  pub fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> Result<()>
  where
    X: Serialize + GetSignature + ?Sized,
  {
    let signature: &Proof = data.signature().ok_or(Error::InvalidSignature("missing signature"))?;

    // Retrieve the method used to create the signature and check it has the required verification
    // method relationship (purpose takes precedence over method_scope).
    let purpose_scope = options.purpose.map(|purpose| match purpose {
      ProofPurpose::AssertionMethod => MethodScope::assertion_method(),
      ProofPurpose::Authentication => MethodScope::authentication(),
    });
    let method: &VerificationMethod<D, U> = match (purpose_scope, options.method_scope) {
      (Some(purpose_scope), _) => self
        .resolve_method(signature, Some(purpose_scope))
        .ok_or(Error::InvalidSignature("method with purpose scope not found"))?,
      (None, Some(scope)) => self
        .resolve_method(signature, Some(scope))
        .ok_or(Error::InvalidSignature("method with specified scope not found"))?,
      (None, None) => self
        .resolve_method(signature, None)
        .ok_or(Error::InvalidSignature("method not found"))?,
    };

    // Check method type.
    if let Some(ref method_types) = options.method_type {
      if !method_types.is_empty() && !method_types.contains(&method.type_) {
        return Err(Error::InvalidSignature("invalid method type"));
      }
    }

    // Check challenge.
    if options.challenge.is_some() && options.challenge != signature.challenge {
      return Err(Error::InvalidSignature("invalid challenge"));
    }

    // Check domain.
    if options.domain.is_some() && options.domain != signature.domain {
      return Err(Error::InvalidSignature("invalid domain"));
    }

    // Check purpose.
    if options.purpose.is_some() && options.purpose != signature.purpose {
      return Err(Error::InvalidSignature("invalid purpose"));
    }

    // Check expired.
    if let Some(expires) = signature.expires {
      if !options.allow_expired.unwrap_or(false) && Timestamp::now_utc() > expires {
        return Err(Error::InvalidSignature("expired"));
      }
    }

    // Check signature.
    Self::do_verify(method, data)
  }

  /// Verifies the signature of the provided data matches the public key data from the given
  /// verification method.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  fn do_verify<X>(method: &VerificationMethod<D, U>, data: &X) -> Result<()>
  where
    X: Serialize + GetSignature + ?Sized,
  {
    let public_key: Vec<u8> = method.data().try_decode()?;

    match method.type_() {
      MethodType::Ed25519VerificationKey2018 => {
        JcsEd25519::<Ed25519>::verify_signature(data, &public_key)?;
      }
      MethodType::X25519KeyAgreementKey2019 => {
        return Err(Error::InvalidMethodType);
      }
    }

    Ok(())
  }
}

impl<D, T, U, V> Document for CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  type D = D;
  type U = U;
  type V = V;

  fn id(&self) -> &Self::D {
    CoreDocument::id(self)
  }

  fn resolve_service<'query, 'me, Q>(&'me self, query: Q) -> Option<&Service<Self::D, Self::V>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.service().query(query.into())
  }

  fn resolve_method<'query, 'me, Q>(
    &'me self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod<Self::D, Self::U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    CoreDocument::resolve_method(self, query, scope)
  }

  fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> Result<()>
  where
    X: Serialize + GetSignature + ?Sized,
  {
    CoreDocument::verify_data(self, data, options)
  }
}

impl<D, T, U, V> TryFrom<CoreDocumentData<D, T, U, V>> for CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  type Error = crate::error::Error;
  fn try_from(value: CoreDocumentData<D, T, U, V>) -> Result<Self, Self::Error> {
    match value.check_id_constraints() {
      Ok(_) => Ok(Self { data: value }),
      Err(err) => Err(err),
    }
  }
}

#[cfg(feature = "revocation-bitmap")]
mod core_document_revocation {
  use identity_core::common::KeyComparable;

  use crate::did::DID;
  use crate::revocation::RevocationBitmap;
  use crate::service::Service;
  use crate::utils::DIDUrlQuery;
  use crate::utils::Queryable;
  use crate::Error;
  use crate::Result;

  use super::CoreDocument;

  impl<D, T, U, V> CoreDocument<D, T, U, V>
  where
    D: DID + KeyComparable,
  {
    /// If the document has a [`RevocationBitmap`] service identified by `service_query`,
    /// revoke all specified `indices`.
    pub fn revoke_credentials<'query, 'me, Q>(&mut self, service_query: Q, indices: &[u32]) -> Result<()>
    where
      Q: Into<DIDUrlQuery<'query>>,
    {
      self.update_revocation_bitmap(service_query, |revocation_bitmap| {
        for credential in indices {
          revocation_bitmap.revoke(*credential);
        }
      })
    }

    /// If the document has a [`RevocationBitmap`] service identified by `service_query`,
    /// unrevoke all specified `indices`.
    pub fn unrevoke_credentials<'query, 'me, Q>(&'me mut self, service_query: Q, indices: &[u32]) -> Result<()>
    where
      Q: Into<DIDUrlQuery<'query>>,
    {
      self.update_revocation_bitmap(service_query, |revocation_bitmap| {
        for credential in indices {
          revocation_bitmap.unrevoke(*credential);
        }
      })
    }

    fn update_revocation_bitmap<'query, 'me, F, Q>(&'me mut self, service_query: Q, f: F) -> Result<()>
    where
      F: FnOnce(&mut RevocationBitmap),
      Q: Into<DIDUrlQuery<'query>>,
    {
      let service: &mut Service<D, V> = self
        .data
        .service
        .query_mut(service_query)
        .ok_or(Error::InvalidService("invalid id - service not found"))?;

      let mut revocation_bitmap: RevocationBitmap = RevocationBitmap::try_from(&*service)?;
      f(&mut revocation_bitmap);

      std::mem::swap(service.service_endpoint_mut(), &mut revocation_bitmap.to_endpoint()?);

      Ok(())
    }
  }
}

// =============================================================================
// Signature Extensions
// =============================================================================

impl<D, T, U, V> CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  /// Creates a new [`DocumentSigner`] that can be used to create digital
  /// signatures from verification methods in this DID Document.
  pub fn signer<'base>(&'base self, private: &'base PrivateKey) -> DocumentSigner<'base, '_, D, T, U, V> {
    DocumentSigner::new(self, private)
  }
}

impl<D, T, U, V> TryMethod for CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  const TYPE: MethodUriType = MethodUriType::Relative;
}

impl<D, T, U, V> Display for CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable + Serialize,
  T: Serialize,
  U: Serialize,
  V: Serialize,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.fmt_json(f)
  }
}

#[cfg(test)]
mod tests {
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;

  use crate::service::ServiceBuilder;
  use crate::verification::MethodBuilder;
  use crate::verification::MethodData;

  use super::*;

  fn controller() -> CoreDID {
    "did:example:1234".parse().unwrap()
  }

  fn method(controller: &CoreDID, fragment: &str) -> VerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(controller.to_url().join(fragment).unwrap())
      .controller(controller.clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn document() -> CoreDocument {
    let controller: CoreDID = controller();

    CoreDocument::builder(Default::default())
      .id(controller.clone())
      .verification_method(method(&controller, "#key-1"))
      .verification_method(method(&controller, "#key-2"))
      .verification_method(method(&controller, "#key-3"))
      .authentication(method(&controller, "#auth-key"))
      .authentication(controller.to_url().join("#key-3").unwrap())
      .key_agreement(controller.to_url().join("#key-4").unwrap())
      .build()
      .unwrap()
  }

  #[test]
  fn test_controller() {
    // One controller.
    {
      let mut document: CoreDocument = document();
      let expected: CoreDID = CoreDID::parse("did:example:one1234").unwrap();
      *document.controller_mut() = Some(OneOrSet::new_one(expected.clone()));
      assert_eq!(document.controller().unwrap().as_slice(), &[expected]);
      // Unset.
      *document.controller_mut() = None;
      assert!(document.controller().is_none());
    }

    // Many controllers.
    {
      let mut document: CoreDocument = document();
      let expected_controllers: Vec<CoreDID> = vec![
        CoreDID::parse("did:example:many1234").unwrap(),
        CoreDID::parse("did:example:many4567").unwrap(),
        CoreDID::parse("did:example:many8910").unwrap(),
      ];
      *document.controller_mut() = Some(expected_controllers.clone().try_into().unwrap());
      assert_eq!(document.controller().unwrap().as_slice(), &expected_controllers);
      // Unset.
      *document.controller_mut() = None;
      assert!(document.controller().is_none());
    }
  }

  #[rustfmt::skip]
  #[test]
  fn test_resolve_method() {
    let document: CoreDocument = document();

    // Resolve methods by fragment.
    assert_eq!(document.resolve_method("#key-1", None).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method("#key-2", None).unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method("#key-3", None).unwrap().id().to_string(), "did:example:1234#key-3");

    // Fine to omit the octothorpe.
    assert_eq!(document.resolve_method("key-1", None).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method("key-2", None).unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method("key-3", None).unwrap().id().to_string(), "did:example:1234#key-3");

    // Resolve methods by full DID Url id.
    assert_eq!(document.resolve_method("did:example:1234#key-1", None).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method("did:example:1234#key-2", None).unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method("did:example:1234#key-3", None).unwrap().id().to_string(), "did:example:1234#key-3");

    // Scope.
    assert_eq!(
      document.resolve_method("#key-1", Some(MethodScope::VerificationMethod)).unwrap().id().to_string(), "did:example:1234#key-1"
    );
  }

  #[rustfmt::skip]
  #[test]
  fn test_resolve_method_mut() {
    let mut document: CoreDocument = document();

    // Resolve methods by fragment.
    assert_eq!(document.resolve_method_mut("#key-1", None).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method_mut("#key-3", None).unwrap().id().to_string(), "did:example:1234#key-3");
    assert_eq!(document.resolve_method_mut("#key-2", None).unwrap().id().to_string(), "did:example:1234#key-2");

    // Fine to omit the octothorpe.
    assert_eq!(document.resolve_method_mut("key-1", None).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method_mut("key-2", None).unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method_mut("key-3", None).unwrap().id().to_string(), "did:example:1234#key-3");

    // Resolve methods by full DID Url id.
    assert_eq!(document.resolve_method_mut("did:example:1234#key-1", None).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method_mut("did:example:1234#key-2", None).unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method_mut("did:example:1234#key-3", None).unwrap().id().to_string(), "did:example:1234#key-3");

    // Resolve with scope.
    assert_eq!(
      document.resolve_method_mut("#key-1", Some(MethodScope::VerificationMethod)).unwrap().id().to_string(), "did:example:1234#key-1"
    );
  }

  #[test]
  fn test_resolve_method_fails() {
    let document: CoreDocument = document();

    // Resolving an existing reference to a missing method returns None.
    assert_eq!(document.resolve_method("#key-4", None), None);

    // Resolving a plain DID returns None.
    assert_eq!(document.resolve_method("did:example:1234", None), None);

    // Resolving an empty string returns None.
    assert_eq!(document.resolve_method("", None), None);

    // Resolve with scope.
    assert_eq!(
      document.resolve_method("#key-1", Some(MethodScope::key_agreement())),
      None
    );
  }

  #[test]
  fn test_resolve_method_mut_fails() {
    let mut document: CoreDocument = document();

    // Resolving an existing reference to a missing method returns None.
    assert_eq!(document.resolve_method_mut("#key-4", None), None);

    // Resolving a plain DID returns None.
    assert_eq!(document.resolve_method_mut("did:example:1234", None), None);

    // Resolving an empty string returns None.
    assert_eq!(document.resolve_method_mut("", None), None);

    // Resolve with scope.
    assert_eq!(
      document.resolve_method_mut("#key-1", Some(MethodScope::key_agreement())),
      None
    );
  }

  #[rustfmt::skip]
  #[test]
  fn test_methods_index() {
    let document: CoreDocument = document();

    // Access methods by index.
    assert_eq!(document.methods(None).get(0).unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.methods(None).get(2).unwrap().id().to_string(), "did:example:1234#key-3");
  }

  #[test]
  fn test_methods_scope() {
    let document: CoreDocument = document();

    // VerificationMethod
    let verification_methods: Vec<&VerificationMethod> = document.methods(Some(MethodScope::VerificationMethod));
    assert_eq!(
      verification_methods.get(0).unwrap().id().to_string(),
      "did:example:1234#key-1"
    );
    assert_eq!(
      verification_methods.get(1).unwrap().id().to_string(),
      "did:example:1234#key-2"
    );
    assert_eq!(
      verification_methods.get(2).unwrap().id().to_string(),
      "did:example:1234#key-3"
    );
    assert_eq!(verification_methods.len(), 3);

    // Authentication
    let authentication: Vec<&VerificationMethod> = document.methods(Some(MethodScope::authentication()));
    assert_eq!(
      authentication.get(0).unwrap().id().to_string(),
      "did:example:1234#auth-key"
    );
    assert_eq!(
      authentication.get(1).unwrap().id().to_string(),
      "did:example:1234#key-3"
    );
    assert_eq!(authentication.len(), 2);
  }

  #[test]
  fn test_attach_verification_relationships() {
    let mut document: CoreDocument = document();

    let fragment = "#attach-test";
    let method = method(document.id(), fragment);
    document.insert_method(method, MethodScope::VerificationMethod).unwrap();

    assert!(document
      .attach_method_relationship(
        document.id().to_url().join(fragment).unwrap(),
        MethodRelationship::CapabilityDelegation,
      )
      .unwrap());

    assert_eq!(document.verification_relationships().count(), 4);

    // Adding it a second time is not an error, but returns false (idempotent).
    assert!(!document
      .attach_method_relationship(
        document.id().to_url().join(fragment).unwrap(),
        MethodRelationship::CapabilityDelegation,
      )
      .unwrap());

    // len is still 2.
    assert_eq!(document.verification_relationships().count(), 4);

    // Attempting to attach a relationship to a non-existing method fails.
    assert!(document
      .attach_method_relationship(
        document.id().to_url().join("#doesNotExist").unwrap(),
        MethodRelationship::CapabilityDelegation,
      )
      .is_err());

    // Attempt to attach to an embedded method.
    assert!(document
      .attach_method_relationship(
        document.id().to_url().join("#auth-key").unwrap(),
        MethodRelationship::CapabilityDelegation,
      )
      .is_err());
  }

  #[test]
  fn test_detach_verification_relationships() {
    let mut document: CoreDocument = document();

    let fragment = "#detach-test";
    let method = method(document.id(), fragment);
    document.insert_method(method, MethodScope::VerificationMethod).unwrap();

    assert!(document
      .attach_method_relationship(
        document.id().to_url().join(fragment).unwrap(),
        MethodRelationship::AssertionMethod,
      )
      .is_ok());

    assert!(document
      .detach_method_relationship(
        document.id().to_url().join(fragment).unwrap(),
        MethodRelationship::AssertionMethod,
      )
      .unwrap());

    // len is 1; the relationship was removed.
    assert_eq!(document.verification_relationships().count(), 3);

    // Removing it a second time is not an error, but returns false (idempotent).
    assert!(!document
      .detach_method_relationship(
        document.id().to_url().join(fragment).unwrap(),
        MethodRelationship::AssertionMethod,
      )
      .unwrap());

    // len is still 1.
    assert_eq!(document.verification_relationships().count(), 3);

    // Attempting to detach a relationship from a non-existing method fails.
    assert!(document
      .detach_method_relationship(
        document.id().to_url().join("#doesNotExist").unwrap(),
        MethodRelationship::AssertionMethod,
      )
      .is_err());
  }

  #[test]
  fn test_method_insert_duplication() {
    let mut document: CoreDocument = document();

    let fragment = "#duplication-test";
    let method1 = method(document.id(), fragment);
    assert!(document
      .insert_method(method1.clone(), MethodScope::VerificationMethod)
      .is_ok());
    assert!(document
      .insert_method(method1.clone(), MethodScope::VerificationMethod)
      .is_err());
    assert!(document
      .insert_method(method1.clone(), MethodScope::authentication())
      .is_err());

    let fragment = "#duplication-test-2";
    let method2 = method(document.id(), fragment);
    assert!(document.insert_method(method2, MethodScope::assertion_method()).is_ok());
    assert!(document
      .insert_method(method1.clone(), MethodScope::VerificationMethod)
      .is_err());
    assert!(document
      .insert_method(method1, MethodScope::capability_delegation())
      .is_err());
  }

  #[test]
  fn test_method_remove_existence() {
    let mut document: CoreDocument = document();

    let fragment = "#existence-test";
    let method1 = method(document.id(), fragment);
    assert!(document
      .insert_method(method1.clone(), MethodScope::VerificationMethod)
      .is_ok());
    assert_eq!(method1, document.remove_method(method1.id()).unwrap());
    assert!(document.remove_method(method1.id()).is_none());

    let fragment = "#existence-test-2";
    let method2 = method(document.id(), fragment);
    assert!(document.insert_method(method2, MethodScope::assertion_method()).is_ok());
    assert!(document.remove_method(method1.id()).is_none());
    assert!(document.remove_method(method1.id()).is_none());

    let fragment = "#removal-test-3";
    let method3 = method(document.id(), fragment);
    assert!(document
      .insert_method(method3.clone(), MethodScope::VerificationMethod)
      .is_ok());
    assert!(document
      .attach_method_relationship(fragment, MethodRelationship::CapabilityDelegation)
      .is_ok());

    assert_eq!(method3, document.remove_method(method3.id()).unwrap());

    // Ensure *all* references were removed.
    assert!(document.capability_delegation().query(method3.id()).is_none());
    assert!(document.verification_method().query(method3.id()).is_none());
  }

  #[cfg(feature = "revocation-bitmap")]
  #[test]
  fn test_revocation() {
    let mut document: CoreDocument = document();
    let indices_1 = [3, 9, 254, 65536];
    let indices_2 = [2, 15, 1337, 1000];

    let service_id = document.id().to_url().join("#revocation-service").unwrap();

    // The methods error if the service doesn't exist.
    assert!(document.revoke_credentials(&service_id, &indices_2).is_err());
    assert!(document.unrevoke_credentials(&service_id, &indices_2).is_err());

    // Add service with indices_1 already revoked.
    let mut bitmap: crate::revocation::RevocationBitmap = crate::revocation::RevocationBitmap::new();
    for index in indices_1.iter() {
      bitmap.revoke(*index);
    }
    assert!(document
      .insert_service(
        Service::builder(Object::new())
          .id(service_id.clone())
          .type_(crate::revocation::RevocationBitmap::TYPE)
          .service_endpoint(bitmap.to_endpoint().unwrap())
          .build()
          .unwrap()
      )
      .is_ok());

    // Revoke indices_2.
    document.revoke_credentials(&service_id, &indices_2).unwrap();
    let service: &Service = document.resolve_service(&service_id).unwrap();
    let decoded_bitmap: crate::revocation::RevocationBitmap = service.try_into().unwrap();

    // We expect all indices to be revoked now.
    for index in indices_1.iter().chain(indices_2.iter()) {
      assert!(decoded_bitmap.is_revoked(*index));
    }

    // Unrevoke indices_1.
    document.unrevoke_credentials(&service_id, &indices_1).unwrap();

    let service: &Service = document.resolve_service(&service_id).unwrap();
    let decoded_bitmap: crate::revocation::RevocationBitmap = service.try_into().unwrap();

    // Expect indices_2 to be revoked, but not indices_1.
    for index in indices_2 {
      assert!(decoded_bitmap.is_revoked(index));
    }
    for index in indices_1 {
      assert!(!decoded_bitmap.is_revoked(index));
    }
  }

  #[test]
  fn test_service_updates() {
    let mut document = document();
    let service_id = document.id().to_url().join("#service-update-test").unwrap();
    let service_type = "test";
    let service_endpoint = Url::parse("https://example.com").unwrap();

    let service: Service = ServiceBuilder::default()
      .id(service_id)
      .type_(service_type)
      .service_endpoint(service_endpoint)
      .build()
      .unwrap();
    // inserting a service with an identifier not present in the document should be Ok.
    assert!(document.insert_service(service.clone()).is_ok());
    // inserting a service with the same identifier as an already existing service should fail.
    let mut service_clone = service.clone();
    *service_clone.service_endpoint_mut() = Url::parse("https://other-example.com").unwrap().into();
    assert!(document.insert_service(service_clone).is_err());
    // removing an existing service should succeed
    assert_eq!(service, document.remove_service(service.id()).unwrap());
    // it should now be possible to insert the service again
    assert!(document.insert_service(service.clone()).is_ok());

    // inserting a method with the same identifier as an existing service should fail
    let method: VerificationMethod = MethodBuilder::default()
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::PublicKeyBase58(
        "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J".into(),
      ))
      .id(service.id().clone())
      .controller(document.id().clone())
      .build()
      .unwrap();

    let method_scopes = [
      MethodScope::VerificationMethod,
      MethodScope::assertion_method(),
      MethodScope::authentication(),
      MethodScope::key_agreement(),
      MethodScope::capability_delegation(),
      MethodScope::capability_invocation(),
    ];
    for scope in method_scopes {
      let mut document_clone = document.clone();
      assert!(document_clone.insert_method(method.clone(), scope).is_err());
      // should succeed after removing the service
      assert!(document_clone.remove_service(service.id()).is_some());
      assert!(document_clone.insert_method(method.clone(), scope).is_ok());
    }

    // inserting a service with the same identifier as a method should fail
    for scope in method_scopes {
      let mut doc_clone = document.clone();
      let valid_method_id = document.id().to_url().join("#valid-method-identifier").unwrap();
      let mut valid_method = method.clone();
      valid_method.set_id(valid_method_id.clone()).unwrap();
      // make sure that the method actually gets inserted
      assert!(doc_clone.insert_method(valid_method.clone(), scope).is_ok());
      let mut service_clone = service.clone();
      service_clone.set_id(valid_method_id).unwrap();
      assert!(doc_clone.insert_service(service_clone.clone()).is_err());
      // but should work after the method has been removed
      assert!(doc_clone.remove_method(valid_method.id()).is_some());
      assert!(doc_clone.insert_service(service_clone).is_ok());
    }

    //removing a service that does not exist should fail
    assert!(document
      .remove_service(&service.id().join("#service-does-not-exist").unwrap())
      .is_none());
  }

  #[test]
  fn serialize_deserialize_roundtrip() {
    let document: CoreDocument = document();
    let doc_json: String = document.to_json().unwrap();
    let doc_json_value: serde_json::Value = document.to_json_value().unwrap();
    let doc_json_vec: Vec<u8> = document.to_json_vec().unwrap();
    assert_eq!(document, CoreDocument::from_json(&doc_json).unwrap());
    assert_eq!(document, CoreDocument::from_json_value(doc_json_value).unwrap());

    assert_eq!(document, CoreDocument::from_json_slice(&doc_json_vec).unwrap());
  }

  #[test]
  fn deserialize_valid() {
    // The verification method types here are really Ed25519VerificationKey2020, changed to be compatible
    // with the current version of this library.
    const JSON_DOCUMENT: &str = r#"{
      "@context": [
        "https://www.w3.org/ns/did/v1",
        "https://w3id.org/security/suites/ed25519-2020/v1"
      ],
      "id": "did:example:123",
      "authentication": [
        {
          "id": "did:example:123#z6MkecaLyHuYWkayBDLw5ihndj3T1m6zKTGqau3A51G7RBf3",
          "type": "Ed25519VerificationKey2018",
          "controller": "did:example:123",
          "publicKeyMultibase": "zAKJP3f7BD6W4iWEQ9jwndVTCBq8ua2Utt8EEjJ6Vxsf"
        }
      ],
      "capabilityInvocation": [
        {
          "id": "did:example:123#z6MkhdmzFu659ZJ4XKj31vtEDmjvsi5yDZG5L7Caz63oP39k",
          "type": "Ed25519VerificationKey2018",
          "controller": "did:example:123",
          "publicKeyMultibase": "z4BWwfeqdp1obQptLLMvPNgBw48p7og1ie6Hf9p5nTpNN"
        }
      ],
      "capabilityDelegation": [
        {
          "id": "did:example:123#z6Mkw94ByR26zMSkNdCUi6FNRsWnc2DFEeDXyBGJ5KTzSWyi",
          "type": "Ed25519VerificationKey2018",
          "controller": "did:example:123",
          "publicKeyMultibase": "zHgo9PAmfeoxHG8Mn2XHXamxnnSwPpkyBHAMNF3VyXJCL"
        }
      ],
      "assertionMethod": [
        {
          "id": "did:example:123#z6MkiukuAuQAE8ozxvmahnQGzApvtW7KT5XXKfojjwbdEomY",
          "type": "Ed25519VerificationKey2018",
          "controller": "did:example:123",
          "publicKeyMultibase": "z5TVraf9itbKXrRvt2DSS95Gw4vqU3CHAdetoufdcKazA"
        }
      ]
  }"#;
    let doc: std::result::Result<CoreDocument, Box<dyn std::error::Error>> =
      CoreDocument::from_json(JSON_DOCUMENT).map_err(Into::into);
    // Print debug representation if the test fails.
    dbg!(&doc);
    assert!(doc.is_ok());
  }

  #[test]
  fn deserialize_duplicate_method_different_scopes() {
    const JSON_VERIFICATION_METHOD_KEY_AGREEMENT: &str = r#"{
      "id": "did:example:1234",
      "verificationMethod": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
        }
      ],
      "keyAgreement": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "X25519KeyAgreementKey2019",
          "publicKeyBase58": "FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x"
        }
      ]
    }"#;

    const JSON_KEY_AGREEMENT_CAPABILITY_INVOCATION: &str = r#"{
      "id": "did:example:1234",
      "capabilityInvocation": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
        }
      ],
      "keyAgreement": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "X25519KeyAgreementKey2019",
          "publicKeyBase58": "FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x"
        }
      ]
    }"#;

    const JSON_ASSERTION_METHOD_CAPABILITY_INVOCATION: &str = r#"{
      "id": "did:example:1234",
      "assertionMethod": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
      ],
      "capabilityInvocation": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
        }
      ]
    }"#;

    const JSON_VERIFICATION_METHOD_AUTHENTICATION: &str = r#"{
      "id": "did:example:1234",
      "verificationMethod": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
        }
      ],
      "authentication": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
      ]
    }"#;

    const JSON_CAPABILITY_DELEGATION_ASSERTION_METHOD: &str = r#"{
      "id": "did:example:1234",
      "capabilityDelegation": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
        }
      ],
      "assertionMethod": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
      ]
    }"#;

    let verifier = |json: &str| {
      let result: std::result::Result<CoreDocument, Box<dyn std::error::Error>> =
        CoreDocument::from_json(json).map_err(Into::into);
      // Print the json if the test fails to aid debugging.
      println!("the following non-spec compliant document was deserialized: \n {json}");
      assert!(result.is_err());
    };

    for json in [
      JSON_VERIFICATION_METHOD_KEY_AGREEMENT,
      JSON_KEY_AGREEMENT_CAPABILITY_INVOCATION,
      JSON_ASSERTION_METHOD_CAPABILITY_INVOCATION,
      JSON_VERIFICATION_METHOD_AUTHENTICATION,
      JSON_CAPABILITY_DELEGATION_ASSERTION_METHOD,
    ] {
      verifier(json);
    }
  }

  #[test]
  fn deserialize_invalid_id_references() {
    const JSON_KEY_AGREEMENT_CAPABILITY_INVOCATION: &str = r#"{
      "id": "did:example:1234",
      "capabilityInvocation": [
        "did:example:1234#key1"
      ],
      "keyAgreement": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "X25519KeyAgreementKey2019",
          "publicKeyBase58": "FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x"
        }
      ]
    }"#;

    const JSON_ASSERTION_METHOD_CAPABILITY_INVOCATION: &str = r#"{
      "id": "did:example:1234",
      "assertionMethod": [
        "did:example:1234#key1", 
        {
          "id": "did:example:1234#key2",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
      ],
      "capabilityInvocation": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyBase58": "3M5RCDjPTWPkKSN3sxUmmMqHbmRPegYP1tjcKyrDbt9J"
        }
      ]
    }"#;

    const JSON_AUTHENTICATION_KEY_AGREEMENT: &str = r#"{
      "id": "did:example:1234",
      "keyAgreement": [
         "did:example:1234#key1"
      ],
      "authentication": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "Ed25519VerificationKey2018",
          "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }
      ]
    }"#;

    const JSON_CAPABILITY_DELEGATION_ASSERTION_METHOD: &str = r#"{
      "id": "did:example:1234",
      "capabilityDelegation": [
        "did:example:1234#key1"
      ],
      "assertionMethod": [
        {
          "id": "did:example:1234#key1",
          "controller": "did:example:1234",
          "type": "X25519KeyAgreementKey2019",
          "publicKeyBase58": "FbQWLPRhTH95MCkQUeFYdiSoQt8zMwetqfWoxqPgaq7x"
        }
      ]
    }"#;

    let verifier = |json: &str| {
      let result: std::result::Result<CoreDocument, Box<dyn std::error::Error>> =
        CoreDocument::from_json(json).map_err(Into::into);
      // Print the json if the test fails to aid debugging.
      println!("the following non-spec compliant document was deserialized: \n {json}");
      assert!(result.is_err());
    };

    for json in [
      JSON_KEY_AGREEMENT_CAPABILITY_INVOCATION,
      JSON_ASSERTION_METHOD_CAPABILITY_INVOCATION,
      JSON_AUTHENTICATION_KEY_AGREEMENT,
      JSON_CAPABILITY_DELEGATION_ASSERTION_METHOD,
    ] {
      verifier(json);
    }
  }
}
