// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto as _;
use core::fmt::Display;
use core::fmt::Formatter;
use std::collections::HashSet;

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
  fn check_id_constraints(&self) -> Result<()> {
    // Algorithm:
    // 1: Create two empty sets: `embedded_method_ids` and `id_references`.
    // 2: Loop through all the scoped verification methods and push the ids of the embedded methods and references into
    // `embedded_method_ids` and `id_references` respectively. If a duplicate embedded method is encountered (that
    // is an embedded method, identified by id, exists in two different scopes) then immediately throw an error.
    // 3: Ensure that the two constructed sets have an empty intersection, otherwise an embedded method is being
    // referenced and an error needs to be thrown. 4: Create a new set that is the union of the
    // `embedded_method_ids` and the set of ids of the unscoped methods (the remaining methods). If the two
    // aforementioned sets have a non trivial intersection there is a duplicate method and an error must be thrown.
    // 5: Create a set of all ids (embedded, references, and unscoped).
    // 6: Loop through all services and check that their ids are not contained in the set constructed in the previous
    // step.

    let mut embedded_method_ids: HashSet<&DIDUrl<D>> = HashSet::new();
    let mut id_references: HashSet<&DIDUrl<D>> = HashSet::new();
    for method_ref in self
      .authentication
      .iter()
      .chain(self.assertion_method.iter())
      .chain(self.key_agreement.iter())
      .chain(self.capability_delegation.iter())
      .chain(self.capability_invocation.iter())
    {
      match method_ref {
        MethodRef::Embed(method) => {
          if !embedded_method_ids.insert(method.id()) {
            //TODO: Consider renaming error or adding a MethodDuplicationAttempt error variant,
            return Err(Error::MethodAlreadyExists);
          }
        }
        MethodRef::Refer(id) => {
          id_references.insert(id);
        }
      }
    }
    if !embedded_method_ids.is_disjoint(&id_references) {
      // TODO: Consider renaming InvalidMethodEmbedded or create a new error variant
      return Err(Error::InvalidMethodEmbedded);
    }

    // Create the union of method ids that belong to embedded methods and those that belong to an unscoped verification
    // method. If the number of elements in the union is not equal to the sum of the number of elements in each of
    // the two aforementioned sets there is a non trivial intersection.
    let num_embedded = embedded_method_ids.len();
    let num_non_scoped = self.verification_method.len();
    let mut method_ids = {
      embedded_method_ids.extend(self.verification_method.iter().map(|method| method.id()));
      embedded_method_ids
    };
    if method_ids.len() < num_embedded + num_non_scoped {
      return Err(Error::MethodAlreadyExists);
    }
    method_ids.extend(id_references);

    for service_id in self.service.iter().map(|service| service.id()) {
      if method_ids.contains(service_id) {
        // TODO: consider using another error variant
        return Err(Error::InvalidService(
          "the service id is shared with a verification method",
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
  pub(crate) inner: CoreDocumentData<D,T,U,V>, 
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
    self.inner.serialize(serializer)
  }
}

// Workaround for lifetime issues with a mutable reference to self preventing closures from being used.
macro_rules! method_ref_mut_helper {
  ($doc:ident, $method: ident, $query: ident) => {
    match $doc.inner.$method.query_mut($query.into())? {
      MethodRef::Embed(method) => Some(method),
      MethodRef::Refer(ref did) => $doc.inner.verification_method.query_mut(did),
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
    &self.inner.id
  }

  /// Returns a mutable reference to the `CoreDocument` id.
  pub fn id_mut(&mut self) -> &mut D {
    &mut self.inner.id
  }

  /// Returns a reference to the `CoreDocument` controller.
  pub fn controller(&self) -> Option<&OneOrSet<D>> {
    self.inner.controller.as_ref()
  }

  /// Returns a mutable reference to the `CoreDocument` controller.
  pub fn controller_mut(&mut self) -> &mut Option<OneOrSet<D>> {
    &mut self.inner.controller
  }

  /// Returns a reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as(&self) -> &OrderedSet<Url> {
    &self.inner.also_known_as
  }

  /// Returns a mutable reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as_mut(&mut self) -> &mut OrderedSet<Url> {
    &mut self.inner.also_known_as
  }

  /// Returns a reference to the `CoreDocument` verificationMethod set.
  pub fn verification_method(&self) -> &OrderedSet<VerificationMethod<D, U>> {
    &self.inner.verification_method
  }

  /// Returns a mutable reference to the `CoreDocument` verificationMethod set.
  pub fn verification_method_mut(&mut self) -> &mut OrderedSet<VerificationMethod<D, U>> {
    &mut self.inner.verification_method
  }

  /// Returns a reference to the `CoreDocument` authentication set.
  pub fn authentication(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.inner.authentication
  }

  /// Returns a mutable reference to the `CoreDocument` authentication set.
  pub fn authentication_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.inner.authentication
  }

  /// Returns a reference to the `CoreDocument` assertionMethod set.
  pub fn assertion_method(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.inner.assertion_method
  }

  /// Returns a mutable reference to the `CoreDocument` assertionMethod set.
  pub fn assertion_method_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.inner.assertion_method
  }

  /// Returns a reference to the `CoreDocument` keyAgreement set.
  pub fn key_agreement(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.inner.key_agreement
  }

  /// Returns a mutable reference to the `CoreDocument` keyAgreement set.
  pub fn key_agreement_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.inner.key_agreement
  }

  /// Returns a reference to the `CoreDocument` capabilityDelegation set.
  pub fn capability_delegation(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.inner.capability_delegation
  }

  /// Returns a mutable reference to the `CoreDocument` capabilityDelegation set.
  pub fn capability_delegation_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.inner.capability_delegation
  }

  /// Returns a reference to the `CoreDocument` capabilityInvocation set.
  pub fn capability_invocation(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.inner.capability_invocation
  }

  /// Returns a mutable reference to the `CoreDocument` capabilityInvocation set.
  pub fn capability_invocation_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.inner.capability_invocation
  }

  /// Returns a reference to the `CoreDocument` service set.
  pub fn service(&self) -> &OrderedSet<Service<D, V>> {
    &self.inner.service
  }

  /// Returns a mutable reference to the `CoreDocument` service set.
  pub fn service_mut(&mut self) -> &mut OrderedSet<Service<D, V>> {
    &mut self.inner.service
  }

  /// Returns a reference to the custom `CoreDocument` properties.
  pub fn properties(&self) -> &T {
    &self.inner.properties
  }

  /// Returns a mutable reference to the custom `CoreDocument` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.inner.properties
  }

  /// Maps `CoreDocument<D,T>` to `CoreDocument<C,U>` by applying a function `f` to all [`DID`] fields
  /// and another function `g` to the custom properties.
  ///
  /// # Panics
  /// Panics if the mapping `f` introduces methods referencing embedded method identifiers,
  /// or services with identifiers matching method identifiers.
  pub fn map<S, C, F, G>(self, mut f: F, g: G) -> CoreDocument<C, S, U, V>
  where
    C: DID + KeyComparable,
    F: FnMut(D) -> C,
    G: FnOnce(T) -> S,
  {
    let current_inner = self.inner;
    CoreDocument::try_from(CoreDocumentData {
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
    })
    .unwrap()
  }

  /// Fallible version of [`CoreDocument::map`].
  ///
  /// # Errors
  ///
  /// `try_map` can fail if either of the provided functions fail or if the mapping `f`
  /// introduces methods referencing embedded method identifiers, or services with identifiers matching method
  /// identifiers..
  pub fn try_map<S, C, F, G, E>(self, mut f: F, g: G) -> Result<Result<CoreDocument<C, S, U, V>, Error>, E>
  where
    C: DID + KeyComparable,
    F: FnMut(D) -> Result<C, E>,
    G: FnOnce(T) -> Result<S, E>,
  {
    let current_inner = self.inner;
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
    helper().map(|data| CoreDocument::try_from(data))
  }

  /// Adds a new [`VerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method with the same fragment already exists.
  pub fn insert_method(&mut self, method: VerificationMethod<D, U>, scope: MethodScope) -> Result<()> {
    if self.resolve_method(method.id(), None).is_some() {
      return Err(Error::MethodAlreadyExists);
    }

    match scope {
      MethodScope::VerificationMethod => self.inner.verification_method.append(method),
      MethodScope::VerificationRelationship(MethodRelationship::Authentication) => {
        self.inner.authentication.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => {
        self.inner.assertion_method.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => {
        self.inner.key_agreement.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => {
        self.inner.capability_delegation.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => {
        self.inner.capability_invocation.append(MethodRef::Embed(method))
      }
    };

    Ok(())
  }

  /// Removes all references to the specified [`VerificationMethod`].
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist.
  pub fn remove_method(&mut self, did: &DIDUrl<D>) -> Result<()> {
    let was_removed: bool = [
      self.inner.authentication.remove(did),
      self.inner.assertion_method.remove(did),
      self.inner.key_agreement.remove(did),
      self.inner.capability_delegation.remove(did),
      self.inner.capability_invocation.remove(did),
      self.inner.verification_method.remove(did),
    ]
    .contains(&true);

    if was_removed {
      Ok(())
    } else {
      Err(Error::MethodNotFound)
    }
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
          MethodRelationship::Authentication => self.authentication_mut().append(method_ref),
          MethodRelationship::AssertionMethod => self.assertion_method_mut().append(method_ref),
          MethodRelationship::KeyAgreement => self.key_agreement_mut().append(method_ref),
          MethodRelationship::CapabilityDelegation => self.capability_delegation_mut().append(method_ref),
          MethodRelationship::CapabilityInvocation => self.capability_invocation_mut().append(method_ref),
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
          MethodRelationship::Authentication => self.authentication_mut().remove(&did_url),
          MethodRelationship::AssertionMethod => self.assertion_method_mut().remove(&did_url),
          MethodRelationship::KeyAgreement => self.key_agreement_mut().remove(&did_url),
          MethodRelationship::CapabilityDelegation => self.capability_delegation_mut().remove(&did_url),
          MethodRelationship::CapabilityInvocation => self.capability_invocation_mut().remove(&did_url),
        };

        Ok(was_detached)
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
      .inner
      .verification_method
      .iter()
      .chain(self.inner.authentication.iter().filter_map(__filter_ref))
      .chain(self.inner.assertion_method.iter().filter_map(__filter_ref))
      .chain(self.inner.key_agreement.iter().filter_map(__filter_ref))
      .chain(self.inner.capability_delegation.iter().filter_map(__filter_ref))
      .chain(self.inner.capability_invocation.iter().filter_map(__filter_ref))
  }

  /// Returns an iterator over all verification relationships.
  ///
  /// This includes embedded and referenced [`VerificationMethods`](VerificationMethod).
  pub fn verification_relationships(&self) -> impl Iterator<Item = &MethodRef<D, U>> {
    self
      .inner
      .authentication
      .iter()
      .chain(self.inner.assertion_method.iter())
      .chain(self.inner.key_agreement.iter())
      .chain(self.inner.capability_delegation.iter())
      .chain(self.inner.capability_invocation.iter())
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
          MethodScope::VerificationMethod => self.inner.verification_method.query(query.into()),
          MethodScope::VerificationRelationship(MethodRelationship::Authentication) => self
            .inner
            .authentication
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => self
            .inner
            .assertion_method
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => self
            .inner
            .key_agreement
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => self
            .inner
            .capability_delegation
            .query(query.into())
            .and_then(resolve_ref_helper),
          MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => self
            .inner
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
        MethodScope::VerificationMethod => self.inner.verification_method.query_mut(query.into()),
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
      MethodRef::Refer(did) => self.inner.verification_method.query(did),
    }
  }

  fn resolve_method_inner(&self, query: DIDUrlQuery<'_>) -> Option<&VerificationMethod<D, U>> {
    let mut method: Option<&MethodRef<D, U>> = None;

    if method.is_none() {
      method = self.inner.authentication.query(query.clone());
    }

    if method.is_none() {
      method = self.inner.assertion_method.query(query.clone());
    }

    if method.is_none() {
      method = self.inner.key_agreement.query(query.clone());
    }

    if method.is_none() {
      method = self.inner.capability_delegation.query(query.clone());
    }

    if method.is_none() {
      method = self.inner.capability_invocation.query(query.clone());
    }

    match method {
      Some(MethodRef::Embed(method)) => Some(method),
      Some(MethodRef::Refer(did)) => self.inner.verification_method.query(&did.to_string()),
      None => self.inner.verification_method.query(query),
    }
  }

  fn resolve_method_mut_inner(&mut self, query: DIDUrlQuery<'_>) -> Option<&mut VerificationMethod<D, U>> {
    let mut method: Option<&mut MethodRef<D, U>> = None;

    if method.is_none() {
      method = self.inner.authentication.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.inner.assertion_method.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.inner.key_agreement.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.inner.capability_delegation.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.inner.capability_invocation.query_mut(query.clone());
    }

    match method {
      Some(MethodRef::Embed(method)) => Some(method),
      Some(MethodRef::Refer(did)) => self.inner.verification_method.query_mut(&did.to_string()),
      None => self.inner.verification_method.query_mut(query),
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
  type Error = crate::Error;
  fn try_from(value: CoreDocumentData<D, T, U, V>) -> Result<Self, Self::Error> {
    match value.check_id_constraints() {
      Ok(_) => Ok(Self { inner: value }),
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
        .service_mut()
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
    assert!(document.remove_method(method1.id()).is_ok());
    assert!(document.remove_method(method1.id()).is_err());

    let fragment = "#existence-test-2";
    let method2 = method(document.id(), fragment);
    assert!(document.insert_method(method2, MethodScope::assertion_method()).is_ok());
    assert!(document.remove_method(method1.id()).is_err());
    assert!(document.remove_method(method1.id()).is_err());

    let fragment = "#removal-test-3";
    let method3 = method(document.id(), fragment);
    assert!(document
      .insert_method(method3.clone(), MethodScope::VerificationMethod)
      .is_ok());
    assert!(document
      .attach_method_relationship(fragment, MethodRelationship::CapabilityDelegation)
      .is_ok());

    assert!(document.remove_method(method3.id()).is_ok());

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
    assert!(document.service_mut().append(
      Service::builder(Object::new())
        .id(service_id.clone())
        .type_(crate::revocation::RevocationBitmap::TYPE)
        .service_endpoint(bitmap.to_endpoint().unwrap())
        .build()
        .unwrap()
    ));

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
