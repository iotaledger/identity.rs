// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::ops::Deref;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::convert::SerdeInto;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::SecretKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_did::did::DID as CoreDID;
use identity_did::document::Document as CoreDocument;
use identity_did::verifiable::Properties as VerifiableProperties;
use identity_did::verifiable::Public;
use identity_did::verifiable::Secret;
use identity_did::verification::Method;
use identity_did::verification::MethodData;
use identity_did::verification::MethodQuery;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use serde::Serialize;

use crate::client::Client;
use crate::client::Network;
use crate::did::DocumentDiff;
use crate::did::Properties as BaseProperties;
use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::MessageId;
use crate::tangle::TangleRef;
use crate::utils::utf8_to_trytes;

const AUTH_QUERY: (usize, MethodScope) = (0, MethodScope::Authentication);

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, ()>;

/// A DID Document adhering to the IOTA DID method specification.
///
/// This is a thin wrapper around the [`Document`][`CoreDocument`] type from the
/// [`identity_did`][`identity_did`] crate.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "CoreDocument", into = "BaseDocument")]
pub struct Document {
  document: BaseDocument,
  message_id: MessageId,
}

impl Document {
  /// Creates a new DID Document from the given KeyPair.
  ///
  /// The DID Document will be pre-populated with a single authentication
  /// method based on the provided [`KeyPair`].
  ///
  /// The authentication method will have the DID URL fragment `#authentication`
  /// and can be easily retrieved with [`Document::authentication`].
  pub fn from_keypair(keypair: &KeyPair) -> Result<Self> {
    fn __method_from_keypair(keypair: &KeyPair) -> Result<Method> {
      let did: DID = DID::new(keypair.public().as_ref())?;
      let key: DID = did.join("#authentication")?;

      let key_type: MethodType = match keypair.type_() {
        KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
      };

      let key_data: MethodData = match keypair.type_() {
        KeyType::Ed25519 => MethodData::new_b58(keypair.public()),
      };

      Method::builder(Default::default())
        .id(key.into())
        .controller(did.into())
        .key_type(key_type)
        .key_data(key_data)
        .build()
        .map_err(Into::into)
    }

    let method: Method = __method_from_keypair(keypair)?;

    let document: Self = CoreDocument::builder(Default::default())
      .id(method.controller().clone())
      .authentication(method)
      .build()
      .map(CoreDocument::into_verifiable)
      .map(Into::into)?;

    Ok(document)
  }

  /// Converts a generic DID `Document` to an IOTA DID Document.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the document is not a valid IOTA DID Document.
  pub fn try_from_document(document: CoreDocument) -> Result<Self> {
    let did: &DID = DID::try_from_borrowed(document.id())?;
    let key: &CoreDID = document.try_resolve(AUTH_QUERY)?.into_method().id();

    // Ensure the authentication method has an identifying fragment
    if key.fragment().is_none() {
      return Err(Error::InvalidDocumentAuthFragment);
    }

    // Ensure the authentication method DID matches the document DID
    if key.authority() != did.authority() {
      return Err(Error::InvalidDocumentAuthAuthority);
    }

    Ok(Self {
      document: document.serde_into()?,
      message_id: MessageId::NONE,
    })
  }

  /// Returns a reference to the underlying [`Document`][`CoreDocument`].
  pub fn as_document(&self) -> &BaseDocument {
    &self.document
  }

  /// Returns a mutable reference to the underlying [`Document`][`CoreDocument`].
  ///
  /// # Safety
  ///
  /// This function is unsafe because it does not check that modifications
  /// made to the `Document` maintain a valid IOTA DID Document.
  ///
  /// If this constraint is violated, it may cause issues with future uses of
  /// the DID Document.
  pub unsafe fn as_document_mut(&mut self) -> &mut BaseDocument {
    &mut self.document
  }

  /// Returns the DID document [`id`][`DID`].
  pub fn id(&self) -> &DID {
    // SAFETY: We checked the validity of the DID Document ID in the
    // DID Document constructors; we don't provide mutable references so
    // the value cannot change with typical "safe" Rust.
    unsafe { DID::new_unchecked_ref(self.document.id()) }
  }

  /// Returns the default authentication method of the DID document.
  pub fn authentication(&self) -> &Method {
    // This `unwrap` is "fine" - a valid document will
    // always have a resolvable authentication method.
    self.document.resolve(AUTH_QUERY).unwrap().into_method()
  }

  /// Returns the timestamp of when the DID document was created.
  pub fn created(&self) -> Timestamp {
    self.document.properties().created
  }

  /// Sets the timestamp of when the DID document was created.
  pub fn set_created(&mut self, value: Timestamp) {
    self.document.properties_mut().created = value;
  }

  /// Returns the timestamp of the last DID document update.
  pub fn updated(&self) -> Timestamp {
    self.document.properties().updated
  }

  /// Sets the timestamp of the last DID document update.
  pub fn set_updated(&mut self, value: Timestamp) {
    self.document.properties_mut().updated = value;
  }

  /// Returns the Tangle message id of the previous DID document, if any.
  pub fn previous_message_id(&self) -> &MessageId {
    &self.document.properties().previous_message_id
  }

  /// Sets the Tangle message id the previous DID document.
  pub fn set_previous_message_id(&mut self, value: impl Into<MessageId>) {
    self.document.properties_mut().previous_message_id = value.into();
  }

  /// Returns true if the DID Document is flagged as immutable.
  pub fn immutable(&self) -> bool {
    self.document.properties().immutable
  }

  /// Sets the value of the `immutable` flag.
  pub fn set_immutable(&mut self, value: bool) {
    self.document.properties_mut().immutable = value;
  }

  /// Returns a reference to the custom DID Document properties.
  pub fn properties(&self) -> &Object {
    &self.document.properties().properties
  }

  /// Returns a mutable reference to the custom DID Document properties.
  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.document.properties_mut().properties
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Signs the DID document with the default authentication method.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the signature operation fails.
  pub fn sign(&mut self, secret: &SecretKey) -> Result<()> {
    self.document.sign_this(AUTH_QUERY, secret.as_ref()).map_err(Into::into)
  }

  /// Verifies the signature of the DID document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the verification operation fails.
  pub fn verify(&self) -> Result<()> {
    self.document.verify_this().map_err(Into::into)
  }

  /// Signs the provided data with the default authentication method.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the signature operation fails.
  pub fn sign_data<'a, 'b, X, Q, S>(&self, data: &mut X, query: Q, secret: S) -> Result<()>
  where
    X: Serialize + SetSignature,
    Q: Into<MethodQuery<'a>>,
    S: Into<Secret<'b>>,
  {
    self.document.sign_that(data, query, secret).map_err(Into::into)
  }

  /// Verfies the signature of the provided data.
  ///
  /// Note: It is assumed that the signature was created using a verification
  /// method contained within the DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the verification operation fails.
  pub fn verify_data<'a, X, P>(&self, data: &X, public: P) -> Result<()>
  where
    X: Serialize + TrySignature,
    P: Into<Public<'a>>,
  {
    self.document.verify_that(data, public).map_err(Into::into)
  }

  // ===========================================================================
  // Diffs
  // ===========================================================================

  /// Creates a `DocumentDiff` representing the changes between `self` and `other`.
  ///
  /// The returned `DocumentDiff` will have a digital signature created using the
  /// default authentication method and `secret`.
  ///
  /// # Errors
  ///
  /// Fails if the diff operation or signature operation fails.
  pub fn diff(&self, other: &Self, message: MessageId, secret: &SecretKey) -> Result<DocumentDiff> {
    let mut diff: DocumentDiff = DocumentDiff::new(self, other, message)?;

    self.sign_data(&mut diff, AUTH_QUERY, secret)?;

    Ok(diff)
  }

  /// Verifies a `DocumentDiff` signature and merges the changes into `self`.
  ///
  /// If merging fails `self` remains unmodified, otherwise `self` represents
  /// the merged document state.
  ///
  /// # Errors
  ///
  /// Fails if the merge operation or signature operation fails.
  pub fn merge(&mut self, diff: &DocumentDiff) -> Result<()> {
    self.verify_data(diff, ())?;

    *self = diff.merge(self)?;

    Ok(())
  }

  // ===========================================================================
  // Publishing
  // ===========================================================================

  /// Publishes the DID Document to the Tangle
  ///
  /// Uses the provided [`client`][``Client``] or a default `Client` based on
  /// the DID network.
  pub async fn publish<'client, C>(&mut self, client: C) -> Result<()>
  where
    C: Into<Option<&'client Client>>,
  {
    let network: Network = self.id().into();

    // Publish the DID Document to the Tangle.
    let message: MessageId = match client.into() {
      Some(client) if client.network() == network => client.publish_document(self).await?,
      Some(_) => return Err(Error::InvalidDIDNetwork),
      None => Client::from_network(network)?.publish_document(self).await?,
    };

    // Update the `self` with the `MessageId` of the bundled transaction.
    self.set_message_id(message);

    Ok(())
  }

  /// Returns the Tangle address of the DID diff chain.
  pub fn diff_address(message_id: &MessageId) -> Result<String> {
    if message_id.is_none() {
      return Err(Error::InvalidDocumentMessageId);
    }

    let hash: String = DID::encode_key(message_id.as_str().as_bytes());

    let mut trytes: String = utf8_to_trytes(&hash);
    trytes.truncate(iota_constants::HASH_TRYTES_SIZE);
    Ok(trytes)
  }
}

impl Display for Document {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.document, f)
  }
}

impl Debug for Document {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.document, f)
  }
}

impl Deref for Document {
  type Target = BaseDocument;

  fn deref(&self) -> &Self::Target {
    &self.document
  }
}

impl From<BaseDocument> for Document {
  fn from(other: BaseDocument) -> Self {
    Self {
      document: other,
      message_id: MessageId::NONE,
    }
  }
}

impl From<Document> for BaseDocument {
  fn from(other: Document) -> Self {
    other.document
  }
}

impl TryFrom<CoreDocument> for Document {
  type Error = Error;

  fn try_from(other: CoreDocument) -> Result<Self, Self::Error> {
    Self::try_from_document(other)
  }
}

impl TrySignature for Document {
  fn signature(&self) -> Option<&Signature> {
    self.document.proof()
  }
}

impl TrySignatureMut for Document {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.document.proof_mut()
  }
}

impl SetSignature for Document {
  fn set_signature(&mut self, signature: Signature) {
    self.document.set_proof(signature)
  }
}

impl TangleRef for Document {
  fn message_id(&self) -> &MessageId {
    &self.message_id
  }

  fn set_message_id(&mut self, message_id: MessageId) {
    self.message_id = message_id;
  }

  fn previous_message_id(&self) -> &MessageId {
    Document::previous_message_id(self)
  }

  fn set_previous_message_id(&mut self, message_id: MessageId) {
    Document::set_previous_message_id(self, message_id)
  }
}
