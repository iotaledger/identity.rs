// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto as _;
use core::fmt::Display;
use core::fmt::Formatter;

use serde::Serialize;

use identity_core::common::BitSet;
use identity_core::common::KeyComparable;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_core::crypto::merkle_key::Blake2b256;
use identity_core::crypto::merkle_key::MerkleDigest;
use identity_core::crypto::merkle_key::MerkleDigestTag;
use identity_core::crypto::merkle_key::MerkleKey;
use identity_core::crypto::merkle_key::MerkleSignature;
use identity_core::crypto::merkle_key::MerkleSignatureTag;
use identity_core::crypto::merkle_key::MerkleVerifier;
use identity_core::crypto::merkle_key::Sha256;
use identity_core::crypto::merkle_key::VerificationKey;
use identity_core::crypto::Ed25519;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::ProofPurpose;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::Verifier;
use identity_core::crypto::Verify;

use crate::did::CoreDID;
use crate::did::DIDUrl;
use crate::did::DID;
use crate::document::DocumentBuilder;
use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::utils::DIDUrlQuery;
use crate::utils::Queryable;
use crate::verifiable::DocumentSigner;
use crate::verifiable::Revocation;
use crate::verifiable::VerifierOptions;
use crate::verification::MethodRef;
use crate::verification::MethodRelationship;
use crate::verification::MethodScope;
use crate::verification::MethodType;
use crate::verification::MethodUriType;
use crate::verification::TryMethod;
use crate::verification::VerificationMethod;

/// A DID Document.
///
/// [Specification](https://www.w3.org/TR/did-core/#did-document-properties)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[rustfmt::skip]
pub struct CoreDocument<D = CoreDID, T = Object, U = Object, V = Object>
  where
    D: DID + KeyComparable
{
  pub(crate) id: D,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) controller: Option<OneOrSet<D>>,
  #[serde(default = "Default::default", rename = "alsoKnownAs", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) also_known_as: OrderedSet<Url>,
  #[serde(default = "Default::default", rename = "verificationMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) verification_method: OrderedSet<VerificationMethod<D,U>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) authentication: OrderedSet<MethodRef<D,U>>,
  #[serde(default = "Default::default", rename = "assertionMethod", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) assertion_method: OrderedSet<MethodRef<D,U>>,
  #[serde(default = "Default::default", rename = "keyAgreement", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) key_agreement: OrderedSet<MethodRef<D,U>>,
  #[serde(default = "Default::default", rename = "capabilityDelegation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_delegation: OrderedSet<MethodRef<D,U>>,
  #[serde(default = "Default::default", rename = "capabilityInvocation", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) capability_invocation: OrderedSet<MethodRef<D,U>>,
  #[serde(default = "Default::default", skip_serializing_if = "OrderedSet::is_empty")]
  pub(crate) service: OrderedSet<Service<D,V>>,
  #[serde(flatten)]
  pub(crate) properties: T,
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
    Ok(Self {
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
    &self.id
  }

  /// Returns a mutable reference to the `CoreDocument` id.
  pub fn id_mut(&mut self) -> &mut D {
    &mut self.id
  }

  /// Returns a reference to the `CoreDocument` controller.
  pub fn controller(&self) -> Option<&OneOrSet<D>> {
    self.controller.as_ref()
  }

  /// Returns a mutable reference to the `CoreDocument` controller.
  pub fn controller_mut(&mut self) -> &mut Option<OneOrSet<D>> {
    &mut self.controller
  }

  /// Returns a reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as(&self) -> &OrderedSet<Url> {
    &self.also_known_as
  }

  /// Returns a mutable reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as_mut(&mut self) -> &mut OrderedSet<Url> {
    &mut self.also_known_as
  }

  /// Returns a reference to the `CoreDocument` verificationMethod set.
  pub fn verification_method(&self) -> &OrderedSet<VerificationMethod<D, U>> {
    &self.verification_method
  }

  /// Returns a mutable reference to the `CoreDocument` verificationMethod set.
  pub fn verification_method_mut(&mut self) -> &mut OrderedSet<VerificationMethod<D, U>> {
    &mut self.verification_method
  }

  /// Returns a reference to the `CoreDocument` authentication set.
  pub fn authentication(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.authentication
  }

  /// Returns a mutable reference to the `CoreDocument` authentication set.
  pub fn authentication_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.authentication
  }

  /// Returns a reference to the `CoreDocument` assertionMethod set.
  pub fn assertion_method(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.assertion_method
  }

  /// Returns a mutable reference to the `CoreDocument` assertionMethod set.
  pub fn assertion_method_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.assertion_method
  }

  /// Returns a reference to the `CoreDocument` keyAgreement set.
  pub fn key_agreement(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.key_agreement
  }

  /// Returns a mutable reference to the `CoreDocument` keyAgreement set.
  pub fn key_agreement_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.key_agreement
  }

  /// Returns a reference to the `CoreDocument` capabilityDelegation set.
  pub fn capability_delegation(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.capability_delegation
  }

  /// Returns a mutable reference to the `CoreDocument` capabilityDelegation set.
  pub fn capability_delegation_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.capability_delegation
  }

  /// Returns a reference to the `CoreDocument` capabilityInvocation set.
  pub fn capability_invocation(&self) -> &OrderedSet<MethodRef<D, U>> {
    &self.capability_invocation
  }

  /// Returns a mutable reference to the `CoreDocument` capabilityInvocation set.
  pub fn capability_invocation_mut(&mut self) -> &mut OrderedSet<MethodRef<D, U>> {
    &mut self.capability_invocation
  }

  /// Returns a reference to the `CoreDocument` service set.
  pub fn service(&self) -> &OrderedSet<Service<D, V>> {
    &self.service
  }

  /// Returns a mutable reference to the `CoreDocument` service set.
  pub fn service_mut(&mut self) -> &mut OrderedSet<Service<D, V>> {
    &mut self.service
  }

  /// Returns a reference to the custom `CoreDocument` properties.
  pub fn properties(&self) -> &T {
    &self.properties
  }

  /// Returns a mutable reference to the custom `CoreDocument` properties.
  pub fn properties_mut(&mut self) -> &mut T {
    &mut self.properties
  }

  /// Maps `CoreDocument<D,T>` to `CoreDocument<C,U>` by applying a function `f` to all [`DID`] fields
  /// and another function `g` to the custom properties.
  pub fn map<S, C, F, G>(self, mut f: F, g: G) -> CoreDocument<C, S, U, V>
  where
    C: DID + KeyComparable,
    F: FnMut(D) -> C,
    G: FnOnce(T) -> S,
  {
    CoreDocument {
      id: f(self.id),
      controller: self.controller.map(|controller_set| controller_set.map(&mut f)),
      also_known_as: self.also_known_as,
      verification_method: self
        .verification_method
        .into_iter()
        .map(|method| method.map(&mut f))
        .collect(),
      authentication: self
        .authentication
        .into_iter()
        .map(|method_ref| method_ref.map(&mut f))
        .collect(),
      assertion_method: self
        .assertion_method
        .into_iter()
        .map(|method_ref| method_ref.map(&mut f))
        .collect(),
      key_agreement: self
        .key_agreement
        .into_iter()
        .map(|method_ref| method_ref.map(&mut f))
        .collect(),
      capability_delegation: self
        .capability_delegation
        .into_iter()
        .map(|method_ref| method_ref.map(&mut f))
        .collect(),
      capability_invocation: self
        .capability_invocation
        .into_iter()
        .map(|method_ref| method_ref.map(&mut f))
        .collect(),
      service: self.service.into_iter().map(|service| service.map(&mut f)).collect(),
      properties: g(self.properties),
    }
  }

  /// Fallible version of [`CoreDocument::map`].
  ///
  /// # Errors
  ///
  /// `try_map` can fail if either of the provided functions fail.
  pub fn try_map<S, C, F, G, E>(self, mut f: F, g: G) -> Result<CoreDocument<C, S, U, V>, E>
  where
    C: DID + KeyComparable,
    F: FnMut(D) -> Result<C, E>,
    G: FnOnce(T) -> Result<S, E>,
  {
    Ok(CoreDocument {
      id: f(self.id)?,
      controller: self
        .controller
        .map(|controller_set| controller_set.try_map(&mut f))
        .transpose()?,
      also_known_as: self.also_known_as,
      verification_method: self
        .verification_method
        .into_iter()
        .map(|method| method.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      authentication: self
        .authentication
        .into_iter()
        .map(|method_ref| method_ref.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      assertion_method: self
        .assertion_method
        .into_iter()
        .map(|method_ref| method_ref.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      key_agreement: self
        .key_agreement
        .into_iter()
        .map(|method_ref| method_ref.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      capability_delegation: self
        .capability_delegation
        .into_iter()
        .map(|method_ref| method_ref.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      capability_invocation: self
        .capability_invocation
        .into_iter()
        .map(|method_ref| method_ref.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      service: self
        .service
        .into_iter()
        .map(|service| service.try_map(&mut f))
        .collect::<Result<_, E>>()?,
      properties: g(self.properties)?,
    })
  }

  /// Adds a new [`VerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method with the same fragment already exists.
  pub fn insert_method(&mut self, method: VerificationMethod<D, U>, scope: MethodScope) -> Result<()> {
    if self.resolve_method(method.id()).is_some() {
      return Err(Error::MethodAlreadyExists);
    }

    match scope {
      MethodScope::VerificationMethod => self.verification_method.append(method),
      MethodScope::VerificationRelationship(MethodRelationship::Authentication) => {
        self.authentication.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => {
        self.assertion_method.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => {
        self.key_agreement.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => {
        self.capability_delegation.append(MethodRef::Embed(method))
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => {
        self.capability_invocation.append(MethodRef::Embed(method))
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
      self.authentication.remove(did),
      self.assertion_method.remove(did),
      self.key_agreement.remove(did),
      self.capability_delegation.remove(did),
      self.capability_invocation.remove(did),
      self.verification_method.remove(did),
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

    match self.resolve_method_with_scope(method_query.clone(), MethodScope::VerificationMethod) {
      None => match self.resolve_method(method_query) {
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
    match self.resolve_method_with_scope(method_query.clone(), MethodScope::VerificationMethod) {
      None => match self.resolve_method(method_query) {
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

  /// Returns an iterator over all embedded verification methods in the DID Document.
  ///
  /// This excludes verification methods that are referenced by the DID Document.
  pub fn methods(&self) -> impl Iterator<Item = &VerificationMethod<D, U>> {
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
      .verification_method
      .iter()
      .chain(self.authentication.iter().filter_map(__filter_ref))
      .chain(self.assertion_method.iter().filter_map(__filter_ref))
      .chain(self.key_agreement.iter().filter_map(__filter_ref))
      .chain(self.capability_delegation.iter().filter_map(__filter_ref))
      .chain(self.capability_invocation.iter().filter_map(__filter_ref))
  }

  /// Returns an iterator over all verification relationships.
  ///
  /// This includes embedded and referenced [`VerificationMethods`](VerificationMethod).
  pub fn verification_relationships(&self) -> impl Iterator<Item = &MethodRef<D, U>> {
    self
      .authentication
      .iter()
      .chain(self.assertion_method.iter())
      .chain(self.key_agreement.iter())
      .chain(self.capability_delegation.iter())
      .chain(self.capability_invocation.iter())
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the provided `query`.
  pub fn resolve_method<'query, Q>(&self, query: Q) -> Option<&VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.resolve_method_inner(query.into())
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the provided `query`.
  ///
  /// # Errors
  ///
  /// Fails if no matching method is found.
  pub fn try_resolve_method<'query, Q>(&self, query: Q) -> Result<&VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.resolve_method_inner(query.into()).ok_or(Error::MethodNotFound)
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the provided `query`
  /// and the verification relationship specified by `scope`.
  pub fn resolve_method_with_scope<'query, 'me, Q>(
    &'me self,
    query: Q,
    scope: MethodScope,
  ) -> Option<&VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    let resolve_ref_helper = |method_ref: &'me MethodRef<D, U>| self.resolve_method_ref(method_ref);

    match scope {
      MethodScope::VerificationMethod => self.verification_method.query(query.into()),
      MethodScope::VerificationRelationship(MethodRelationship::Authentication) => {
        self.authentication.query(query.into()).and_then(resolve_ref_helper)
      }
      MethodScope::VerificationRelationship(MethodRelationship::AssertionMethod) => {
        self.assertion_method.query(query.into()).and_then(resolve_ref_helper)
      }
      MethodScope::VerificationRelationship(MethodRelationship::KeyAgreement) => {
        self.key_agreement.query(query.into()).and_then(resolve_ref_helper)
      }
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityDelegation) => self
        .capability_delegation
        .query(query.into())
        .and_then(resolve_ref_helper),
      MethodScope::VerificationRelationship(MethodRelationship::CapabilityInvocation) => self
        .capability_invocation
        .query(query.into())
        .and_then(resolve_ref_helper),
    }
  }

  /// Returns the first [`VerificationMethod`] with an `id` property matching the provided `query`
  /// and the verification relationship specified by `scope`.
  ///
  /// # Errors
  ///
  /// Fails if no matching [`VerificationMethod`] is found.
  pub fn try_resolve_method_with_scope<'query, 's: 'query, Q>(
    &'s self,
    query: Q,
    scope: MethodScope,
  ) -> Result<&VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self
      .resolve_method_with_scope(query, scope)
      .ok_or(Error::MethodNotFound)
  }

  /// Returns a mutable reference to the first [`VerificationMethod`] with an `id` property
  /// matching the provided `query`.
  pub fn resolve_method_mut<'query, Q>(&mut self, query: Q) -> Option<&mut VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.resolve_method_mut_inner(query.into())
  }

  /// Returns a mutable reference to the first [`VerificationMethod`] with an `id` property
  /// matching the provided `query`.
  ///
  /// # Errors
  ///
  /// Fails if no matching [`VerificationMethod`] is found.
  pub fn try_resolve_method_mut<'query, Q>(&mut self, query: Q) -> Result<&mut VerificationMethod<D, U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.resolve_method_mut_inner(query.into()).ok_or(Error::MethodNotFound)
  }

  #[doc(hidden)]
  pub fn resolve_method_ref<'a>(&'a self, method_ref: &'a MethodRef<D, U>) -> Option<&'a VerificationMethod<D, U>> {
    match method_ref {
      MethodRef::Embed(method) => Some(method),
      MethodRef::Refer(did) => self.verification_method.query(did),
    }
  }

  fn resolve_method_inner(&self, query: DIDUrlQuery<'_>) -> Option<&VerificationMethod<D, U>> {
    let mut method: Option<&MethodRef<D, U>> = None;

    if method.is_none() {
      method = self.authentication.query(query.clone());
    }

    if method.is_none() {
      method = self.assertion_method.query(query.clone());
    }

    if method.is_none() {
      method = self.key_agreement.query(query.clone());
    }

    if method.is_none() {
      method = self.capability_delegation.query(query.clone());
    }

    if method.is_none() {
      method = self.capability_invocation.query(query.clone());
    }

    match method {
      Some(MethodRef::Embed(method)) => Some(method),
      Some(MethodRef::Refer(did)) => self.verification_method.query(&did.to_string()),
      None => self.verification_method.query(query),
    }
  }

  fn resolve_method_mut_inner(&mut self, query: DIDUrlQuery<'_>) -> Option<&mut VerificationMethod<D, U>> {
    let mut method: Option<&mut MethodRef<D, U>> = None;

    if method.is_none() {
      method = self.authentication.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.assertion_method.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.key_agreement.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.capability_delegation.query_mut(query.clone());
    }

    if method.is_none() {
      method = self.capability_invocation.query_mut(query.clone());
    }

    match method {
      Some(MethodRef::Embed(method)) => Some(method),
      Some(MethodRef::Refer(did)) => self.verification_method.query_mut(&did.to_string()),
      None => self.verification_method.query_mut(query),
    }
  }
}

impl<D, T, U: Revocation, V> CoreDocument<D, T, U, V>
where
  D: DID + KeyComparable,
{
  /// Verifies the signature of the provided data.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the verification operation fails.
  pub fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    let signature: &Signature = data
      .try_signature()
      .map_err(|_| Error::InvalidSignature("missing signature"))?;

    // Retrieve the method used to create the signature and check it has the required verification
    // method relationship (purpose takes precedence over method_scope).
    let purpose_scope = options.purpose.map(|purpose| match purpose {
      ProofPurpose::AssertionMethod => MethodScope::assertion_method(),
      ProofPurpose::Authentication => MethodScope::authentication(),
    });
    let method: &VerificationMethod<D, U> = match (purpose_scope, options.method_scope) {
      (Some(purpose_scope), _) => self
        .try_resolve_method_with_scope(signature, purpose_scope)
        .map_err(|_| Error::InvalidSignature("method with purpose scope not found"))?,
      (None, Some(scope)) => self
        .try_resolve_method_with_scope(signature, scope)
        .map_err(|_| Error::InvalidSignature("method with specified scope not found"))?,
      (None, None) => self
        .try_resolve_method(signature)
        .map_err(|_| Error::InvalidSignature("method not found"))?,
    };

    // Check method type.
    if let Some(ref method_types) = options.method_type {
      if !method_types.is_empty() && !method_types.contains(&method.key_type) {
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
    X: Serialize + TrySignature,
  {
    let public_key: Vec<u8> = method.key_data().try_decode()?;

    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {
        JcsEd25519::<Ed25519>::verify_signature(data, &public_key)?;
      }
      MethodType::X25519KeyAgreementKey2019 => {
        return Err(Error::InvalidMethodType);
      }
      MethodType::MerkleKeyCollection2021 => match MerkleKey::extract_tags(&public_key)? {
        (MerkleSignatureTag::ED25519, MerkleDigestTag::SHA256) => {
          merkle_key_verify::<D, X, Sha256, Ed25519, U>(data, method, &public_key)?;
        }
        (MerkleSignatureTag::ED25519, MerkleDigestTag::BLAKE2B_256) => {
          merkle_key_verify::<D, X, Blake2b256, Ed25519, U>(data, method, &public_key)?;
        }
        (_, _) => {
          return Err(Error::InvalidMethodType);
        }
      },
    }

    Ok(())
  }
}

fn merkle_key_verify<D, X, M, S, U>(that: &X, method: &VerificationMethod<D, U>, data: &[u8]) -> Result<()>
where
  D: DID,
  X: Serialize + TrySignature,
  M: MerkleDigest,
  S: MerkleSignature + Verify<Public = [u8]>,
  U: Revocation,
{
  let revocation: Option<BitSet> = method.revocation()?;
  let mut vkey: VerificationKey<'_> = VerificationKey::from_borrowed(data);

  if let Some(revocation) = revocation.as_ref() {
    vkey.set_revocation(revocation);
  }

  MerkleVerifier::<M, S>::verify_signature(that, &vkey)?;

  Ok(())
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
  pub fn signer<'base>(&'base self, private: &'base PrivateKey) -> DocumentSigner<'base, '_, '_, D, T, U, V> {
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
  use crate::did::CoreDID;
  use crate::did::DID;
  use crate::document::CoreDocument;
  use crate::utils::Queryable;
  use crate::verification::MethodData;
  use crate::verification::MethodRelationship;
  use crate::verification::MethodScope;
  use crate::verification::MethodType;
  use crate::verification::VerificationMethod;

  fn controller() -> CoreDID {
    "did:example:1234".parse().unwrap()
  }

  fn method(controller: &CoreDID, fragment: &str) -> VerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(controller.to_url().join(fragment).unwrap())
      .controller(controller.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_multibase(fragment.as_bytes()))
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

  #[rustfmt::skip]
  #[test]
  fn test_resolve_method() {
    let document: CoreDocument = document();

    // Resolve methods by fragment.
    assert_eq!(document.resolve_method("#key-1").unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method("#key-2").unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method("#key-3").unwrap().id().to_string(), "did:example:1234#key-3");

    // Fine to omit the octothorpe.
    assert_eq!(document.resolve_method("key-1").unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method("key-2").unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method("key-3").unwrap().id().to_string(), "did:example:1234#key-3");

    // Resolve methods by full DID Url id.
    assert_eq!(document.resolve_method("did:example:1234#key-1").unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.resolve_method("did:example:1234#key-2").unwrap().id().to_string(), "did:example:1234#key-2");
    assert_eq!(document.resolve_method("did:example:1234#key-3").unwrap().id().to_string(), "did:example:1234#key-3");
  }

  #[test]
  fn test_resolve_method_fails() {
    let document: CoreDocument = document();

    // Resolving an existing reference to a missing method returns None.
    assert_eq!(document.resolve_method("#key-4"), None);

    // Resolving a plain DID returns None.
    assert_eq!(document.resolve_method("did:example:1234"), None);

    // Resolving an empty string returns None.
    assert_eq!(document.resolve_method(""), None);
  }

  #[rustfmt::skip]
  #[test]
  fn test_methods_index() {
    let document: CoreDocument = document();

    // Access methods by index.
    assert_eq!(document.methods().next().unwrap().id().to_string(), "did:example:1234#key-1");
    assert_eq!(document.methods().nth(2).unwrap().id().to_string(), "did:example:1234#key-3");
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
}
