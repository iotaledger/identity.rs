// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use hashbrown::HashMap;
use hashbrown::hash_map::Entry;
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
use crate::types::Fragment;
use crate::types::Index;
use crate::types::IndexMap;
use crate::types::Timestamp;

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, ()>;

pub type RemoteEd25519<'a, T> = JcsEd25519<RemoteSign<'a, T>>;
pub type MethodData = (MethodScope, Option<Object>);

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
  methods: HashMap<ChainKey, MethodData>,
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
      methods: HashMap::new(),
      services: None,
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
    self.diff_index.get(self.auth_index).copied().unwrap_or(Index::ZERO)
  }

  pub fn increment_auth_index(&mut self) -> Result<()> {
    todo!("increment_auth_index")
  }

  pub fn increment_diff_index(&mut self) -> Result<()> {
    match self.diff_index.entry(self.auth_index) {
      Entry::Occupied(mut entry) => {
        entry.insert(entry.get().try_increment()?);
      }
      Entry::Vacant(entry) => {
        entry.insert(Index::ONE);
      }
    }

    Ok(())
  }

  // ===========================================================================
  // Tangle State
  // ===========================================================================

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
    self.messages.set_diff_message_id(self.auth_index, self.diff_index(), message);
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

  pub fn methods(&self) -> impl Iterator<Item = (&ChainKey, &MethodData)> {
    self.methods.iter()
  }

  pub fn methods_mut(&mut self) -> impl Iterator<Item = (&ChainKey, &mut MethodData)> {
    self.methods.iter_mut()
  }

  pub fn method(&self, fragment: &str) -> Option<(&ChainKey, &MethodData)> {
    self.methods().find(|(location, _)| location.fragment() == fragment)
  }

  pub fn method_mut(&mut self, fragment: &str) -> Option<(&ChainKey, &mut MethodData)> {
    self.methods_mut().find(|(location, _)| location.fragment() == fragment)
  }

  pub fn set_method(
    &mut self,
    location: ChainKey,
    scope: MethodScope,
    extra: Option<Object>,
  ) {
    // TODO: Replace by fragment
    self.methods.insert(location, (scope, extra));
  }

  // pub fn auth(&self) -> ChainKey {
  //   ChainKey::auth(MethodType::Ed25519VerificationKey2018, self.auth_index())
  // }

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

    for (location, (scope, extra)) in self.methods.iter() {
      let public: PublicKey = store.key_get(self.chain, location).await?;
      let extra: Object = extra.as_ref().cloned().unwrap_or_default();
      let method: CoreMethod = location.to_core(document_id, &public, extra)?;

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
