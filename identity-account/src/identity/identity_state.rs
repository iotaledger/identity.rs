// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;
use serde::Serialize;

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signer;
use identity_did::did::CoreDIDUrl;
use identity_did::did::DID;
use identity_did::service::Service as CoreService;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRef as CoreMethodRef;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDIDUrl;
use identity_iota::did::IotaDocument;
use identity_iota::tangle::TangleRef;

use crate::crypto::RemoteKey;
use crate::crypto::RemoteSign;
use crate::error::Error;
use crate::error::Result;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;

pub type RemoteEd25519<'a> = JcsEd25519<RemoteSign<'a>>;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IdentityState {
  generation: Generation,
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  method_generations: HashMap<Fragment, Generation>,

  document: IotaDocument,
}

impl IdentityState {
  pub fn new(document: IotaDocument) -> Self {
    Self {
      generation: Generation::new(),
      method_generations: HashMap::new(),
      document,
    }
  }

  // ===========================================================================
  // Internal State
  // ===========================================================================

  /// Returns the current generation of the identity integration chain.
  pub fn generation(&self) -> Generation {
    self.generation
  }

  /// Increments the generation of the identity diff chain.
  pub fn increment_generation(&mut self) -> Result<()> {
    self.generation = self.generation.try_increment()?;

    Ok(())
  }

  /// Stores the generations at which the method was inserted.
  pub fn set_method_generations(&mut self, fragment: Fragment) {
    self.method_generations.insert(fragment, self.generation());
  }

  /// Return the `KeyLocation` of the given method.
  pub fn method_location(&self, method_type: MethodType, fragment: String) -> Result<KeyLocation> {
    let fragment = Fragment::new(fragment);
    // We don't return `MethodNotFound`, as this error might occur when a method exists
    // in the document, but is not present locally (e.g. in a distributed setup).
    let generation = self.method_generations.get(&fragment).ok_or(Error::KeyNotFound)?;

    Ok(KeyLocation::new(method_type, fragment.into(), *generation))
  }

  // ===========================================================================
  // Document State
  // ===========================================================================

  pub fn as_document(&self) -> &IotaDocument {
    &self.document
  }

  pub fn as_document_mut(&mut self) -> &mut IotaDocument {
    &mut self.document
  }

  /// Returns a key location suitable for the specified `fragment`.
  pub fn key_location(&self, method: MethodType, fragment: String) -> Result<KeyLocation> {
    Ok(KeyLocation::new(method, fragment, self.generation()))
  }

  pub async fn sign_data<U>(
    &self,
    did: &IotaDID,
    store: &dyn Storage,
    location: &KeyLocation,
    target: &mut U,
  ) -> Result<()>
  where
    U: Serialize + SetSignature,
  {
    // Create a private key suitable for identity_core::crypto
    let private: RemoteKey<'_> = RemoteKey::new(did, location, store);

    // Create the Verification Method identifier
    let fragment: &str = location.fragment().identifier();
    let method_url: IotaDIDUrl = self.document.did().to_url().join(fragment)?;

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        RemoteEd25519::create_signature(target, method_url.to_string(), &private)?;
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("Handle MerkleKeyCollection2021")
      }
    }

    Ok(())
  }
}

// =============================================================================
// TinyMethodRef
// =============================================================================

/// A thin representation of a Verification Method reference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TinyMethodRef {
  Embed(TinyMethod),
  Refer(Fragment),
}

impl TinyMethodRef {
  /// Returns the fragment identifying the Verification Method reference.
  pub fn fragment(&self) -> &Fragment {
    match self {
      Self::Embed(inner) => inner.location.fragment(),
      Self::Refer(inner) => inner,
    }
  }

  /// Creates a new `CoreMethodRef` from the method reference state.
  pub fn to_core(&self, did: &IotaDID) -> Result<CoreMethodRef> {
    match self {
      Self::Embed(inner) => inner.to_core(did).map(CoreMethodRef::Embed),
      Self::Refer(inner) => did
        .to_url()
        .join(inner.identifier())
        .map(CoreDIDUrl::from)
        .map(CoreMethodRef::Refer)
        .map_err(Into::into),
    }
  }

  fn __embed(method: &TinyMethodRef) -> Option<&TinyMethod> {
    match method {
      Self::Embed(inner) => Some(inner),
      Self::Refer(_) => None,
    }
  }
}

// =============================================================================
// TinyMethod
// =============================================================================

/// A thin representation of a Verification Method.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TinyMethod {
  #[serde(rename = "1")]
  location: KeyLocation,
  #[serde(rename = "2")]
  key_data: MethodData,
  #[serde(rename = "3")]
  properties: Option<Object>,
}

impl TinyMethod {
  /// Creates a new `TinyMethod`.
  pub fn new(location: KeyLocation, key_data: MethodData, properties: Option<Object>) -> Self {
    Self {
      location,
      key_data,
      properties,
    }
  }

  /// Returns the key location of the Verification Method.
  pub fn location(&self) -> &KeyLocation {
    &self.location
  }

  /// Returns the computed method data of the Verification Method.
  pub fn key_data(&self) -> &MethodData {
    &self.key_data
  }

  /// Returns any additional Verification Method properties.
  pub fn properties(&self) -> Option<&Object> {
    self.properties.as_ref()
  }

  /// Creates a new [VerificationMethod].
  pub fn to_core(&self, did: &IotaDID) -> Result<VerificationMethod> {
    let properties: Object = self.properties.clone().unwrap_or_default();
    let id: IotaDIDUrl = did.to_url().join(self.location.fragment().identifier())?;

    VerificationMethod::builder(properties)
      .id(CoreDIDUrl::from(id))
      .controller(did.clone().into())
      .key_type(self.location.method())
      .key_data(self.key_data.clone())
      .build()
      .map_err(Into::into)
  }
}

// =============================================================================
// Methods
// =============================================================================

/// A map of Verification Method states.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Methods {
  data: HashMap<MethodScope, Vec<TinyMethodRef>>,
}

impl Methods {
  /// Creates a new `Methods` instance.
  pub fn new() -> Self {
    Self { data: HashMap::new() }
  }

  /// Returns the total number of Verification Methods in the map.
  ///
  /// Note: This does not include Verification Method references.
  pub fn len(&self) -> usize {
    self.iter().count()
  }

  /// Returns true if the map has no Verification Methods.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns a slice of the Verification Methods applicable to the given `scope`.
  pub fn slice(&self, scope: MethodScope) -> &[TinyMethodRef] {
    self.data.get(&scope).map(|data| &**data).unwrap_or_default()
  }

  /// Returns an iterator over all embedded Verification Methods.
  pub fn iter(&self) -> impl Iterator<Item = &TinyMethod> {
    self.iter_ref().filter_map(TinyMethodRef::__embed)
  }

  /// Returns an iterator over all Verification Methods.
  ///
  /// Note: This includes Verification Method references.
  pub fn iter_ref(&self) -> impl Iterator<Item = &TinyMethodRef> {
    self
      .slice(MethodScope::VerificationMethod)
      .iter()
      .chain(self.slice(MethodScope::Authentication).iter())
      .chain(self.slice(MethodScope::AssertionMethod).iter())
      .chain(self.slice(MethodScope::KeyAgreement).iter())
      .chain(self.slice(MethodScope::CapabilityDelegation).iter())
      .chain(self.slice(MethodScope::CapabilityInvocation).iter())
  }

  /// Returns a reference to the Verification Method identified by the given
  /// `fragment`.
  pub fn get(&self, fragment: &str) -> Option<&TinyMethod> {
    self.iter().find(|method| method.location().fragment_name() == fragment)
  }

  /// Returns a reference to the Verification Method identified by the given
  /// `fragment`.
  ///
  /// # Errors
  ///
  /// Fails if no matching Verification Method is found.
  pub fn fetch(&self, fragment: &str) -> Result<&TinyMethod> {
    self.get(fragment).ok_or(Error::MethodNotFound)
  }

  /// Returns true if the map contains a method with the given `fragment`.
  pub fn contains(&self, fragment: &str) -> bool {
    self.iter().any(|method| method.location().fragment_name() == fragment)
  }

  /// Adds a new method to the map - no validation is performed.
  pub fn insert(&mut self, scope: MethodScope, method: TinyMethodRef) {
    self.data.entry(scope).or_default().push(method);
  }

  /// Removes the method specified by `fragment` from the given `scope`.
  pub fn detach(&mut self, scope: MethodScope, fragment: &str) {
    if let Some(list) = self.data.get_mut(&scope) {
      list.retain(|method| method.fragment().name() != fragment);
    }
  }

  /// Removes the Verification Method specified by the given `fragment`.
  ///
  /// Note: This includes both references and embedded structures.
  pub fn delete(&mut self, fragment: &str) {
    for (_, list) in self.data.iter_mut() {
      list.retain(|method| method.fragment().name() != fragment);
    }
  }
}

// =============================================================================
// TinyService
// =============================================================================

/// A thin representation of a DID Document service.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TinyService {
  #[serde(rename = "1")]
  fragment: Fragment,
  #[serde(rename = "2")]
  type_: String,
  #[serde(rename = "3")]
  endpoint: Url,
  #[serde(rename = "4")]
  properties: Option<Object>,
}

impl TinyService {
  /// Creates a new `TinyService`.
  pub fn new(fragment: String, type_: String, endpoint: Url, properties: Option<Object>) -> Self {
    Self {
      fragment: Fragment::new(fragment),
      type_,
      endpoint,
      properties,
    }
  }

  /// Returns the fragment identifying the service.
  pub fn fragment(&self) -> &Fragment {
    &self.fragment
  }

  /// Creates a new `CoreService` from the service state.
  pub fn to_core(&self, did: &IotaDID) -> Result<CoreService<Object>> {
    let properties: Object = self.properties.clone().unwrap_or_default();
    let id: IotaDIDUrl = did.to_url().join(self.fragment().identifier())?;

    CoreService::builder(properties)
      .id(CoreDIDUrl::from(id))
      .type_(&self.type_)
      .service_endpoint(self.endpoint.clone())
      .build()
      .map_err(Into::into)
  }
}

// =============================================================================
// Services
// =============================================================================

/// A set of DID Document service states.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Services {
  data: Vec<TinyService>,
}

impl Services {
  /// Creates a new `Services` instance.
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  /// Returns the total number of services in the set.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns true if the set has no services.
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// Returns an iterator over the services in the set.
  pub fn iter(&self) -> impl Iterator<Item = &TinyService> {
    self.data.iter()
  }

  /// Returns a reference to the service identified by the given `fragment`.
  pub fn get(&self, fragment: &str) -> Option<&TinyService> {
    self.iter().find(|service| service.fragment().name() == fragment)
  }

  /// Returns a reference to the service identified by the given `fragment`.
  ///
  /// # Errors
  ///
  /// Fails if no matching service is found.
  pub fn fetch(&self, fragment: &str) -> Result<&TinyService> {
    self.get(fragment).ok_or(Error::ServiceNotFound)
  }

  /// Returns true if the set contains a service with the given `fragment`.
  pub fn contains(&self, fragment: &str) -> bool {
    self.iter().any(|service| service.fragment().name() == fragment)
  }

  /// Adds a new `service` to the set - no validation is performed.
  pub fn insert(&mut self, service: TinyService) {
    self.data.push(service);
  }

  /// Removes the service specified by the given `fragment`.
  pub fn delete(&mut self, fragment: &str) {
    self.data.retain(|service| service.fragment().name() != fragment);
  }
}
