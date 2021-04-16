// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signer;
use identity_did::document::Document as CoreDocument;
use identity_did::document::DocumentBuilder;
use identity_did::error::Error as DIDError;
use identity_did::service::Service as CoreService;
use identity_did::verifiable::Properties as VerifiableProperties;
use identity_did::verification::Method as CoreMethod;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRef as CoreMethodRef;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::Document;
use identity_iota::did::Properties as BaseProperties;
use identity_iota::did::DID;
use identity_iota::tangle::MessageId;
use identity_iota::tangle::MessageIdExt;
use identity_iota::tangle::TangleRef;
use serde::Serialize;

use crate::chain::ChainKey;
use crate::crypto::RemoteKey;
use crate::crypto::RemoteSign;
use crate::error::Error;
use crate::error::Result;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Fragment;
use crate::types::Index;
use crate::types::Timestamp;

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, Object>;

pub type RemoteEd25519<'a, T> = JcsEd25519<RemoteSign<'a, T>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainData {
  // =========== //
  // Chain State //
  // =========== //
  chain: ChainId,
  auth_index: Index,
  diff_index: Index,
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
  document: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  controller: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  also_known_as: Option<Vec<Url>>,
  #[serde(skip_serializing_if = "Methods::is_empty")]
  methods: Methods,
  #[serde(skip_serializing_if = "Services::is_empty")]
  services: Services,
  #[serde(skip_serializing_if = "Timestamp::is_epoch")]
  created: Timestamp,
  #[serde(skip_serializing_if = "Timestamp::is_epoch")]
  updated: Timestamp,
}

impl ChainData {
  pub fn new(chain: ChainId) -> Self {
    Self {
      chain,
      auth_index: Index::ZERO,
      diff_index: Index::ZERO,
      this_message_id: MessageId::null(),
      last_auth_message_id: MessageId::null(),
      last_diff_message_id: MessageId::null(),
      document: None,
      controller: None,
      also_known_as: None,
      methods: Methods::new(),
      services: Services::new(),
      created: Timestamp::EPOCH,
      updated: Timestamp::EPOCH,
    }
  }

  // ===========================================================================
  // Chain State
  // ===========================================================================

  pub fn chain(&self) -> ChainId {
    self.chain
  }

  pub fn auth_index(&self) -> Index {
    self.auth_index
  }

  pub fn diff_index(&self) -> Index {
    self.diff_index
  }

  pub fn increment_auth_index(&mut self) -> Result<()> {
    self.auth_index = self.auth_index.try_increment()?;
    self.diff_index = Index::ZERO;

    Ok(())
  }

  pub fn increment_diff_index(&mut self) -> Result<()> {
    self.diff_index = self.diff_index.try_increment()?;

    Ok(())
  }

  // ===========================================================================
  // Tangle State
  // ===========================================================================

  pub fn this_message_id(&self) -> &MessageId {
    &self.this_message_id
  }

  pub fn last_message_id(&self) -> &MessageId {
    &self.last_auth_message_id
  }

  pub fn diff_message_id(&self) -> &MessageId {
    if !self.last_diff_message_id.is_null() {
      &self.last_diff_message_id
    } else {
      &self.this_message_id
    }
  }

  pub fn set_auth_message_id(&mut self, message: MessageId) {
    self.last_auth_message_id = self.this_message_id;
    self.last_diff_message_id = MessageId::null();
    self.this_message_id = message;
  }

  pub fn set_diff_message_id(&mut self, message: MessageId) {
    self.last_diff_message_id = message;
  }

  // ===========================================================================
  // Document State
  // ===========================================================================

  pub fn document(&self) -> Option<&DID> {
    self.document.as_ref()
  }

  pub fn try_document(&self) -> Result<&DID> {
    self.document().ok_or(Error::MissingChainDocument)
  }

  pub fn set_document(&mut self, document: DID) {
    self.document = Some(document);
  }

  pub fn created(&self) -> Timestamp {
    self.created
  }

  pub fn updated(&self) -> Timestamp {
    self.updated
  }

  pub fn set_created(&mut self, timestamp: Timestamp) {
    self.created = timestamp;
    self.updated = timestamp;
  }

  pub fn set_updated(&mut self, timestamp: Timestamp) {
    self.updated = timestamp;
  }

  pub fn methods(&self) -> &Methods {
    &self.methods
  }

  pub fn methods_mut(&mut self) -> &mut Methods {
    &mut self.methods
  }

  pub fn services(&self) -> &Services {
    &self.services
  }

  pub fn services_mut(&mut self) -> &mut Services {
    &mut self.services
  }

  pub fn authentication(&self) -> Result<&TinyMethod> {
    self.methods.fetch(ChainKey::AUTH)
  }

  pub fn key(&self, type_: MethodType, fragment: String) -> Result<ChainKey> {
    Ok(ChainKey {
      type_,
      auth: self.auth_index(),
      diff: self.diff_index(),
      fragment: Fragment::new(fragment),
    })
  }

  // ===========================================================================
  // DID Document Helpers
  // ===========================================================================

  pub fn to_document(&self) -> Result<Document> {
    let properties: BaseProperties = BaseProperties::new();
    let properties: Properties = VerifiableProperties::new(properties);
    let mut builder: DocumentBuilder<_, _, _> = BaseDocument::builder(properties);

    let document_id: &DID = self.try_document()?;

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

    // TODO: This completely bypasses method validation...
    let mut document: Document = builder.build().map(Into::into)?;

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

  pub async fn to_signed_document<T>(&self, store: &T) -> Result<Document>
  where
    T: Storage,
  {
    let mut document: Document = self.to_document()?;
    let location: &ChainKey = self.authentication()?.location();

    // Sign the DID Document with the authentication method
    Self::sign_document(self.chain, store, location, &mut document).await?;

    Ok(document)
  }

  pub async fn sign_data<T, U>(&self, store: &T, location: &ChainKey, target: &mut U) -> Result<()>
  where
    T: Storage,
    U: Serialize + SetSignature,
  {
    // Create a secret key suitable for identity_core::crypto
    let secret: RemoteKey<'_, T> = RemoteKey::new(self.chain, location, store);

    // Create the verification method identifier for verification operations
    let method: DID = self.try_document()?.join(location.fragment.ident())?;
    let method: &str = method.as_str();

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => {
        RemoteEd25519::create_signature(target, method, &secret)?;
      }
      MethodType::MerkleKeyCollection2021 => {
        todo!("Handle MerkleKeyCollection2021")
      }
    }

    Ok(())
  }

  // ===========================================================================
  // Private
  // ===========================================================================

  async fn sign_document<T: Storage>(
    chain: ChainId,
    store: &T,
    location: &ChainKey,
    document: &mut Document,
  ) -> Result<()> {
    // Create a secret key suitable for identity_core::crypto
    let secret: RemoteKey<'_, T> = RemoteKey::new(chain, location, store);

    // Create the verification method identifier for verification operations
    let method: DID = document.id().join(format!("#{}", ChainKey::AUTH))?;
    let method: &str = method.as_str();

    match location.type_() {
      MethodType::Ed25519VerificationKey2018 => {
        RemoteEd25519::create_signature(document, method, &secret).map_err(Into::into)
      }
      MethodType::MerkleKeyCollection2021 => {
        // DID Documents can't be signed with Merkle Key Collections
        Err(DIDError::InvalidMethodType.into())
      }
    }
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TinyMethodRef {
  Embed(TinyMethod),
  Refer(Fragment),
}

impl TinyMethodRef {
  fn fragment(&self) -> &str {
    match self {
      Self::Embed(inner) => inner.location.fragment(),
      Self::Refer(inner) => inner.value(),
    }
  }

  fn __filter(method: &TinyMethodRef) -> Option<&TinyMethod> {
    match method {
      Self::Embed(inner) => Some(inner),
      Self::Refer(_) => None,
    }
  }

  fn to_core(&self, document: &DID) -> Result<CoreMethodRef> {
    match self {
      Self::Embed(inner) => inner.to_core(document).map(CoreMethodRef::Embed),
      Self::Refer(inner) => document
        .join(inner.ident())
        .map(Into::into)
        .map(CoreMethodRef::Refer)
        .map_err(Into::into),
    }
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TinyMethod {
  #[serde(rename = "1")]
  location: ChainKey,
  #[serde(rename = "2")]
  key_data: MethodData,
  #[serde(rename = "3")]
  properties: Option<Object>,
}

impl TinyMethod {
  pub fn new(location: ChainKey, key_data: MethodData) -> Self {
    Self {
      location,
      key_data,
      properties: None,
    }
  }

  pub fn location(&self) -> &ChainKey {
    &self.location
  }

  pub fn key_data(&self) -> &MethodData {
    &self.key_data
  }

  pub fn properties(&self) -> Option<&Object> {
    self.properties.as_ref()
  }

  fn to_core(&self, document: &DID) -> Result<CoreMethod> {
    let kdata: MethodData = self.key_data.clone();
    let extra: Option<Object> = self.properties.clone();

    self.location.to_core(document, kdata, extra.unwrap_or_default())
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Methods {
  data: HashMap<MethodScope, Vec<TinyMethodRef>>,
}

impl Methods {
  pub fn new() -> Self {
    Self { data: HashMap::new() }
  }

  pub fn len(&self) -> usize {
    self.iter().count()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn slice(&self, scope: MethodScope) -> &[TinyMethodRef] {
    self.data.get(&scope).map(|data| &**data).unwrap_or_default()
  }

  pub fn iter(&self) -> impl Iterator<Item = &TinyMethod> {
    self.iter_ref().filter_map(TinyMethodRef::__filter)
  }

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

  pub fn get(&self, fragment: &str) -> Option<&TinyMethod> {
    self.iter().find(|method| method.location.fragment() == fragment)
  }

  pub fn fetch(&self, fragment: &str) -> Result<&TinyMethod> {
    self.get(fragment).ok_or(Error::MethodNotFound)
  }

  pub fn contains(&self, fragment: &str) -> bool {
    self.iter().any(|method| method.location.fragment() == fragment)
  }

  pub fn insert(&mut self, scope: MethodScope, method: TinyMethod) {
    self.delete(method.location.fragment());

    self.data.entry(scope).or_default().push(TinyMethodRef::Embed(method));
  }

  pub fn detach(&mut self, scope: MethodScope, fragment: &str) {
    if let Some(list) = self.data.get_mut(&scope) {
      list.retain(|method| method.fragment() != fragment);
    }
  }

  pub fn delete(&mut self, fragment: &str) {
    for (_, list) in self.data.iter_mut() {
      list.retain(|method| method.fragment() != fragment);
    }
  }
}

// =============================================================================
// =============================================================================

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
  pub fn new(fragment: String, type_: String, endpoint: Url, properties: Option<Object>) -> Self {
    Self {
      fragment: Fragment::new(fragment),
      type_,
      endpoint,
      properties,
    }
  }

  fn fragment(&self) -> &str {
    self.fragment.value()
  }

  fn to_core(&self, document: &DID) -> Result<CoreService<Object>> {
    CoreService::builder(self.properties.clone().unwrap_or_default())
      .id(document.join(self.fragment.ident()).map(Into::into)?)
      .type_(&self.type_)
      .service_endpoint(self.endpoint.clone())
      .build()
      .map_err(Into::into)
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Services {
  data: Vec<TinyService>,
}

impl Services {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  pub fn iter(&self) -> impl Iterator<Item = &TinyService> {
    self.data.iter()
  }

  pub fn get(&self, fragment: &str) -> Option<&TinyService> {
    self.iter().find(|service| service.fragment() == fragment)
  }

  pub fn fetch(&self, fragment: &str) -> Result<&TinyService> {
    self.get(fragment).ok_or(Error::ServiceNotFound)
  }

  pub fn contains(&self, fragment: &str) -> bool {
    self.iter().any(|service| service.fragment() == fragment)
  }

  pub fn insert(&mut self, service: TinyService) {
    self.data.push(service);
  }

  pub fn delete(&mut self, fragment: &str) {
    self.data.retain(|service| service.fragment() != fragment);
  }
}
