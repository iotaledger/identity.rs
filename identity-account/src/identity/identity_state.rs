// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use hashbrown::HashMap;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signer;
use identity_did::document::CoreDocument;
use identity_did::document::DocumentBuilder;
use identity_did::service::Service as CoreService;
use identity_did::verifiable::Properties as VerifiableProperties;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRef as CoreMethodRef;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDocument;
use identity_iota::did::Properties as BaseProperties;
use identity_iota::tangle::MessageId;
use identity_iota::tangle::MessageIdExt;
use identity_iota::tangle::TangleRef;
use serde::Serialize;

use crate::crypto::RemoteKey;
use crate::crypto::RemoteSign;
use crate::error::Error;
use crate::error::Result;
use crate::identity::IdentityId;
use crate::storage::Storage;
use crate::types::Fragment;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::UnixTimestamp;

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, Object>;

pub type RemoteEd25519<'a, T> = JcsEd25519<RemoteSign<'a, T>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentityState {
  // =========== //
  // Chain State //
  // =========== //
  id: IdentityId,
  auth_generation: Generation,
  diff_generation: Generation,
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  this_message_id: MessageId,
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  last_auth_message_id: MessageId,
  #[serde(default = "MessageId::null", skip_serializing_if = "MessageId::is_null")]
  last_diff_message_id: MessageId,

  // ============== //
  // Document State //
  // ============== //
  #[serde(skip_serializing_if = "Option::is_none")]
  did: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  controller: Option<IotaDID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  also_known_as: Option<Vec<Url>>,
  #[serde(skip_serializing_if = "Methods::is_empty")]
  methods: Methods,
  #[serde(default, skip_serializing_if = "Services::is_empty")]
  services: Services,
  #[serde(default, skip_serializing_if = "UnixTimestamp::is_epoch")]
  created: UnixTimestamp,
  #[serde(default, skip_serializing_if = "UnixTimestamp::is_epoch")]
  updated: UnixTimestamp,
}

impl IdentityState {
  pub fn new(id: IdentityId) -> Self {
    Self {
      id,
      auth_generation: Generation::new(),
      diff_generation: Generation::new(),
      this_message_id: MessageId::null(),
      last_auth_message_id: MessageId::null(),
      last_diff_message_id: MessageId::null(),
      did: None,
      controller: None,
      also_known_as: None,
      methods: Methods::new(),
      services: Services::new(),
      created: UnixTimestamp::EPOCH,
      updated: UnixTimestamp::EPOCH,
    }
  }

  // ===========================================================================
  // Internal State
  // ===========================================================================

  /// Returns the identifier for this identity.
  pub fn id(&self) -> IdentityId {
    self.id
  }

  /// Returns the current generation of the identity auth chain.
  pub fn auth_generation(&self) -> Generation {
    self.auth_generation
  }

  /// Returns the current generation of the identity diff chain.
  pub fn diff_generation(&self) -> Generation {
    self.diff_generation
  }

  /// Increments the generation of the identity auth chain.
  pub fn increment_auth_generation(&mut self) -> Result<()> {
    self.auth_generation = self.auth_generation.try_increment()?;
    self.diff_generation = Generation::new();

    Ok(())
  }

  /// Increments the generation of the identity diff chain.
  pub fn increment_diff_generation(&mut self) -> Result<()> {
    self.diff_generation = self.diff_generation.try_increment()?;

    Ok(())
  }

  // ===========================================================================
  // Tangle State
  // ===========================================================================

  /// Returns the current auth Tangle message id of the identity.
  pub fn this_message_id(&self) -> &MessageId {
    &self.this_message_id
  }

  /// Returns the previous auth Tangle message id of the identity.
  pub fn last_message_id(&self) -> &MessageId {
    &self.last_auth_message_id
  }

  /// Returns the previous diff Tangle message id, or the current auth message id.
  pub fn diff_message_id(&self) -> &MessageId {
    if self.last_diff_message_id.is_null() {
      &self.this_message_id
    } else {
      &self.last_diff_message_id
    }
  }

  /// Sets the current Tangle auth message id of the identity.
  pub fn set_auth_message_id(&mut self, message: MessageId) {
    // Set the current auth message id as the previous auth message.
    self.last_auth_message_id = self.this_message_id;

    // Clear the diff message id
    self.last_diff_message_id = MessageId::null();

    // Set the new auth message id
    self.this_message_id = message;
  }

  /// Sets the current Tangle diff message id of the identity.
  pub fn set_diff_message_id(&mut self, message: MessageId) {
    self.last_diff_message_id = message;
  }

  // ===========================================================================
  // Document State
  // ===========================================================================

  /// Returns the DID identifying the DID Document for the state.
  pub fn did(&self) -> Option<&IotaDID> {
    self.did.as_ref()
  }

  /// Returns the DID identifying the DID Document for the state.
  ///
  /// # Errors
  ///
  /// Fails if the DID is not set.
  pub fn try_did(&self) -> Result<&IotaDID> {
    self.did().ok_or(Error::MissingDocumentId)
  }

  /// Sets the DID identifying the DID Document for the state.
  pub fn set_did(&mut self, did: IotaDID) {
    self.did = Some(did);
  }

  /// Returns the timestamp of when the state was created.
  pub fn created(&self) -> UnixTimestamp {
    self.created
  }

  /// Returns the timestamp of when the state was last updated.
  pub fn updated(&self) -> UnixTimestamp {
    self.updated
  }

  /// Sets the timestamp of when the state was created.
  pub fn set_created(&mut self, timestamp: UnixTimestamp) {
    self.created = timestamp;
  }

  /// Sets the timestamp of when the state was last updated.
  pub fn set_updated(&mut self, timestamp: UnixTimestamp) {
    self.updated = timestamp;
  }

  /// Returns a reference to the state methods.
  pub fn methods(&self) -> &Methods {
    &self.methods
  }

  /// Returns a mutable reference to the state methods.
  pub fn methods_mut(&mut self) -> &mut Methods {
    &mut self.methods
  }

  /// Returns a reference to the state services.
  pub fn services(&self) -> &Services {
    &self.services
  }

  /// Returns a mutable reference to the state services.
  pub fn services_mut(&mut self) -> &mut Services {
    &mut self.services
  }

  /// Returns the latest authentication method in the state.
  pub fn authentication(&self) -> Result<&TinyMethod> {
    self
      .methods()
      .iter()
      .filter(|method| method.is_authentication())
      .max_by_key(|method| method.location().auth_generation())
      .ok_or(Error::MethodNotFound)
  }

  /// Returns a key location suitable for the specified `fragment`.
  pub fn key_location(&self, method: MethodType, fragment: String) -> Result<KeyLocation> {
    Ok(KeyLocation {
      method,
      fragment: Fragment::new(fragment),
      auth_generation: self.auth_generation(),
      diff_generation: self.diff_generation(),
    })
  }

  // ===========================================================================
  // DID Document Helpers
  // ===========================================================================

  /// Creates a new DID Document based on the identity state.
  pub fn to_document(&self) -> Result<IotaDocument> {
    let properties: BaseProperties = BaseProperties::new();
    let properties: Properties = VerifiableProperties::new(properties);
    let mut builder: DocumentBuilder<_, _, _> = BaseDocument::builder(properties);

    let document_id: &IotaDID = self.try_did()?;

    builder = builder.id(document_id.clone().into());

    if let Some(value) = self.controller.as_ref() {
      builder = builder.controller(value.clone().into());
    }

    if let Some(values) = self.also_known_as.as_deref() {
      for value in values {
        builder = builder.also_known_as(value.clone());
      }
    }

    for method in self.methods.slice(MethodScope::VerificationMethod) {
      builder = match method.to_core(document_id)? {
        CoreMethodRef::Embed(inner) => builder.verification_method(inner),
        CoreMethodRef::Refer(_) => unreachable!(),
      };
    }

    for method in self.methods.slice(MethodScope::Authentication) {
      builder = builder.authentication(method.to_core(document_id)?);
    }

    for method in self.methods.slice(MethodScope::AssertionMethod) {
      builder = builder.assertion_method(method.to_core(document_id)?);
    }

    for method in self.methods.slice(MethodScope::KeyAgreement) {
      builder = builder.key_agreement(method.to_core(document_id)?);
    }

    for method in self.methods.slice(MethodScope::CapabilityDelegation) {
      builder = builder.capability_delegation(method.to_core(document_id)?);
    }

    for method in self.methods.slice(MethodScope::CapabilityInvocation) {
      builder = builder.capability_invocation(method.to_core(document_id)?);
    }

    for service in self.services.iter() {
      builder = builder.service(service.to_core(document_id)?);
    }

    let mut document: IotaDocument = builder.build()?.try_into()?;

    if !self.this_message_id.is_null() {
      document.set_message_id(self.this_message_id);
    }

    if !self.last_auth_message_id.is_null() {
      document.set_previous_message_id(self.last_auth_message_id);
    }

    document.set_created(self.created.into());
    document.set_updated(self.updated.into());

    Ok(document)
  }

  pub async fn sign_data<T, U>(&self, store: &T, location: &KeyLocation, target: &mut U) -> Result<()>
  where
    T: Storage,
    U: Serialize + SetSignature,
  {
    // Create a secret key suitable for identity_core::crypto
    let secret: RemoteKey<'_, T> = RemoteKey::new(self.id, location, store);

    // Create the Verification Method identifier
    let fragment: &str = location.fragment.identifier();
    let method: IotaDID = self.try_did()?.join(fragment)?;

    match location.method() {
      MethodType::Ed25519VerificationKey2018 => {
        RemoteEd25519::create_signature(target, method.as_str(), &secret)?;
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
  /// Returns true if the method reference is an authentication method.
  pub fn is_authentication(&self) -> bool {
    match self {
      Self::Embed(inner) => inner.is_authentication(),
      Self::Refer(_) => false,
    }
  }

  /// Returns the fragment identifying the Verification Method reference.
  pub fn fragment(&self) -> &Fragment {
    match self {
      Self::Embed(inner) => &inner.location.fragment,
      Self::Refer(inner) => &inner,
    }
  }

  /// Creates a new `CoreMethodRef` from the method reference state.
  pub fn to_core(&self, did: &IotaDID) -> Result<CoreMethodRef> {
    match self {
      Self::Embed(inner) => inner.to_core(did).map(CoreMethodRef::Embed),
      Self::Refer(inner) => did
        .join(inner.identifier())
        .map(Into::into)
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

  /// Returns true if the Verification Method is an internal authentication method.
  pub fn is_authentication(&self) -> bool {
    self.location.is_authentication()
  }

  /// Creates a new [VerificationMethod].
  pub fn to_core(&self, did: &IotaDID) -> Result<VerificationMethod> {
    let properties: Object = self.properties.clone().unwrap_or_default();
    let id: IotaDID = did.join(self.location.fragment.identifier())?;

    VerificationMethod::builder(properties)
      .id(id.into())
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
    self.iter().find(|method| method.location().fragment() == fragment)
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
    self.iter().any(|method| method.location().fragment() == fragment)
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
    let id: IotaDID = did.join(self.fragment().identifier())?;

    CoreService::builder(properties)
      .id(id.into())
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
