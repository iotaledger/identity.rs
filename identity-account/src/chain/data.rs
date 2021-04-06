// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::JcsEd25519;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signer;
use identity_did::document::Document as CoreDocument;
use identity_did::document::DocumentBuilder;
use identity_did::error::Error as DIDError;
use identity_did::service::Service;
use identity_did::verifiable::Properties as VerifiableProperties;
use identity_did::verification::Method as CoreMethod;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::Document;
use identity_iota::did::Properties as BaseProperties;
use identity_iota::did::DID;
use identity_iota::tangle::MessageId;
use identity_iota::tangle::TangleRef;
use serde::Serialize;

use crate::chain::ChainKey;
use crate::chain::ChainMessages;
use crate::crypto::RemoteKey;
use crate::crypto::RemoteSign;
use crate::error::Error;
use crate::error::Result;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Index;
use crate::types::IndexMap;
use crate::types::Timestamp;

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, ()>;

type RemoteEd25519<'a, T> = JcsEd25519<RemoteSign<'a, T>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChainData {
  // =========== //
  // Chain State //
  // =========== //
  chain: ChainId,
  auth_index: Index,
  diff_index: IndexMap<Index>,
  messages: ChainMessages,
  // ============== //
  // Document State //
  // ============== //
  document: Option<DID>,
  controller: Option<DID>,
  also_known_as: Option<Vec<Url>>,
  methods: Vec<(MethodScope, ChainKey, Object)>,
  services: Option<Vec<Service<()>>>,
  created: Timestamp,
  updated: Timestamp,
}

impl ChainData {
  pub fn new(chain: ChainId) -> Self {
    Self {
      chain,
      auth_index: Index::ZERO,
      diff_index: IndexMap::new(),
      messages: ChainMessages::new(),
      document: None,
      controller: None,
      also_known_as: None,
      methods: Vec::new(),
      services: None,
      created: Timestamp::EPOCH,
      updated: Timestamp::EPOCH,
    }
  }

  pub fn chain(&self) -> ChainId {
    self.chain
  }

  pub fn auth_index(&self) -> Index {
    self.auth_index
  }

  pub fn diff_index(&self) -> Index {
    self.diff_index.get(self.auth_index).copied().unwrap_or(Index::ZERO)
  }

  pub fn this_message_id(&self) -> Option<&MessageId> {
    self.messages.this_message_id(self.auth_index)
  }

  pub fn last_message_id(&self) -> Option<&MessageId> {
    self.messages.last_message_id(self.auth_index)
  }

  pub fn diff_message_id(&self) -> Option<&MessageId> {
    self
      .messages
      .diff_message_id(self.auth_index, self.diff_index())
      .or_else(|| self.this_message_id())
  }

  pub fn set_auth_message_id(&mut self, message: MessageId) {
    self.messages.set_auth_message_id(self.auth_index, message);
  }

  pub fn set_diff_message_id(&mut self, message: MessageId) {
    // self.messages.set_diff_message_id(self.auth_index, self.diff_index, message);
  }

  pub fn increment_diff_index(&mut self) -> Result<()> {
    let entry: &mut Index = self
      .diff_index
      .entry(self.auth_index)
      .or_default();

    *entry = entry.try_increment()?;

    Ok(())
  }

  pub fn set_created(&mut self, timestamp: Timestamp) {
    self.created = timestamp;
    self.updated = timestamp;
  }

  pub fn set_updated(&mut self, timestamp: Timestamp) {
    self.updated = timestamp;
  }

  pub fn append_method(&mut self, scope: MethodScope, location: ChainKey) {
    self.append_method_with_data(scope, location, Object::new());
  }

  pub fn append_method_with_data(&mut self, scope: MethodScope, location: ChainKey, data: Object) {
    self.methods.push((scope, location, data));
  }

  pub fn document(&self) -> Option<&DID> {
    self.document.as_ref()
  }

  pub fn try_document(&self) -> Result<&DID> {
    self.document().ok_or(Error::MissingChainDocument)
  }

  pub fn set_document(&mut self, document: DID) {
    self.document = Some(document);
  }

  pub fn current_auth(&self) -> ChainKey {
    ChainKey::auth(MethodType::Ed25519VerificationKey2018, self.auth_index())
  }

  pub async fn to_document<T>(&self, store: &T) -> Result<Document>
  where
    T: Storage,
  {
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

    let authtype: MethodType = MethodType::Ed25519VerificationKey2018; // TODO: FIXME
    let location: ChainKey = ChainKey::auth(authtype, self.auth_index);
    let authdata: PublicKey = store.key_get(self.chain, &location).await?;
    let method: CoreMethod = location.to_core(document_id, &authdata, Default::default())?;

    builder = builder.authentication(method);

    for (scope, key, properties) in self.methods.iter() {
      let public: PublicKey = store.key_get(self.chain, &key).await?;
      let method: CoreMethod = key.to_core(document_id, &public, properties.clone())?;

      builder = match scope {
        MethodScope::VerificationMethod => builder.verification_method(method),
        MethodScope::Authentication => builder.authentication(method),
        MethodScope::AssertionMethod => builder.assertion_method(method),
        MethodScope::KeyAgreement => builder.key_agreement(method),
        MethodScope::CapabilityDelegation => builder.capability_delegation(method),
        MethodScope::CapabilityInvocation => builder.capability_invocation(method),
      };
    }

    if let Some(values) = self.services.as_ref() {
      for value in values {
        builder = builder.service(value.clone());
      }
    }

    // TODO: This completely bypasses method validation...
    let mut document: Document = builder.build().map(Into::into)?;

    if let Some(message) = self.this_message_id() {
      document.set_message_id(message.clone());
    }

    if let Some(message) = self.last_message_id() {
      document.set_previous_message_id(message.clone());
    }

    document.set_created(self.created.into());
    document.set_updated(self.updated.into());

    Ok(document)
  }

  pub async fn to_signed_document<T>(&self, store: &T) -> Result<Document>
  where
    T: Storage,
  {
    let mut document: Document = self.to_document(store).await?;

    // Sign the DID Document with the authentication method
    let authtype: MethodType = MethodType::Ed25519VerificationKey2018; // TODO: FIXME
    let location: ChainKey = ChainKey::auth(authtype, self.auth_index);

    Self::sign_document(self.chain, store, &location, &mut document).await?;

    Ok(document)
  }

  pub async fn sign_data<T, U>(&self, location: &ChainKey, store: &T, target: &mut U) -> Result<()>
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
  // Misc. Helpers
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
