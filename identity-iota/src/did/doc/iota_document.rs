// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::SerdeInto;
use identity_core::crypto::KeyPair;
use identity_core::crypto::SecretKey;
use identity_core::crypto::SetSignature;
use identity_core::crypto::Signature;
use identity_core::crypto::TrySignature;
use identity_core::crypto::TrySignatureMut;
use identity_did::document::CoreDocument;
use identity_did::service::Service;
use identity_did::utils::DIDKey;
use identity_did::utils::OrderedSet;
use identity_did::verifiable::DocumentSigner;
use identity_did::verifiable::DocumentVerifier;
use identity_did::verifiable::Properties as VerifiableProperties;
use identity_did::verification::MethodQuery;
use identity_did::verification::MethodRef;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_did::verification::MethodUriType;
use identity_did::verification::TryMethod;
use identity_did::verification::VerificationMethod;
use iota_client::bee_message::MessageId;
use serde::Serialize;

use crate::client::Client;
use crate::client::Network;
use crate::did::DocumentDiff;
use crate::did::IotaDID;
use crate::did::IotaVerificationMethod;
use crate::did::Properties as BaseProperties;
use crate::error::Error;
use crate::error::Result;
use crate::tangle::MessageIdExt;
use crate::tangle::TangleRef;

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, Object>;

pub type Signer<'a, 'b, 'c> = DocumentSigner<'a, 'b, 'c, Properties, Object, Object>;
pub type Verifier<'a> = DocumentVerifier<'a, Properties, Object, Object>;

/// A DID Document adhering to the IOTA DID method specification.
///
/// This is a thin wrapper around the [`CoreDocument`][CoreDocument] type from the
/// [identity_did] crate.
#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "CoreDocument", into = "BaseDocument")]
pub struct IotaDocument {
  document: BaseDocument,
  message_id: MessageId,
}

impl TryMethod for IotaDocument {
  const TYPE: MethodUriType = MethodUriType::Absolute;
}

impl IotaDocument {
  /// Creates a new DID Document from the given KeyPair.
  ///
  /// The DID Document will be pre-populated with a single authentication
  /// method based on the provided [KeyPair].
  ///
  /// The authentication method will have the DID URL fragment `#authentication`
  /// and can be easily retrieved with [Document::authentication].
  pub fn from_keypair(keypair: &KeyPair) -> Result<Self> {
    let method: IotaVerificationMethod = IotaVerificationMethod::from_keypair(keypair, "authentication")?;

    // SAFETY: We don't create invalid Methods.  Method::from_keypair() uses the MethodBuilder
    // internally which verifies correctness on construction.
    Ok(unsafe { Self::from_authentication_unchecked(method) })
  }

  /// Creates a new DID Document from the given KeyPair and network.
  ///
  /// The DID Document will be pre-populated with a single authentication
  /// method based on the provided [KeyPair].
  ///
  /// The authentication method will have the DID URL fragment `#authentication`
  /// and can be easily retrieved with [Document::authentication].
  pub fn from_keypair_with_network(keypair: &KeyPair, network: &str) -> Result<Self> {
    let method: IotaVerificationMethod =
      IotaVerificationMethod::from_keypair_with_network(keypair, "authentication", network)?;

    // SAFETY: We don't create invalid Methods.  Method::from_keypair() uses the MethodBuilder
    // internally which verifies correctness on construction.
    Ok(unsafe { Self::from_authentication_unchecked(method) })
  }

  /// Creates a new DID Document from the given verification [`method`][VerificationMethod].
  pub fn from_authentication(method: IotaVerificationMethod) -> Result<Self> {
    Self::check_authentication(&method)?;

    // SAFETY: We just checked the validity of the verification method.
    Ok(unsafe { Self::from_authentication_unchecked(method) })
  }

  /// Creates a new DID Document from the given verification [`method`][IotaVerificationMethod]
  /// without performing validation checks.
  ///
  /// # Safety
  ///
  /// This must be guaranteed safe by the caller.
  pub unsafe fn from_authentication_unchecked(method: IotaVerificationMethod) -> Self {
    CoreDocument::builder(Default::default())
      .id(method.controller().clone().into())
      .authentication(method)
      .build()
      .map(CoreDocument::into_verifiable)
      .map(TryInto::try_into)
      .map(Result::unwrap)
      .unwrap() // `unwrap` is fine - we provided all the necessary properties
  }

  /// Converts a generic DID [`Document`][`CoreDocument`] to an IOTA DID Document.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the document is not a valid IOTA DID Document.
  pub fn try_from_core(document: CoreDocument) -> Result<Self> {
    IotaDocument::validate_core_document(&document)?;

    Ok(Self {
      document: document.serde_into()?,
      message_id: MessageId::null(),
    })
  }

  /// Converts a generic DID [`Document`][`BaseDocument`] to an IOTA DID Document.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the document is not a valid IOTA DID Document.
  pub fn try_from_base(document: BaseDocument) -> Result<Self> {
    IotaDocument::validate_core_document(&document)?;

    Ok(Self {
      document: document.serde_into()?,
      message_id: MessageId::null(),
    })
  }

  /// Performs validation that a `CoreDocument` adheres to the IOTA spec.
  ///
  /// # Errors
  ///
  /// Returns `Err` if the document is not a valid IOTA DID Document.
  fn validate_core_document<T, U, V>(document: &CoreDocument<T, U, V>) -> Result<()> {
    // Validate that the DID conforms to the IotaDID specification.
    // This check is required to ensure the correctness of the `IotaDocument::id()` method which
    // creates an `IotaDID::new_unchecked_ref()` from the underlying DID.
    let did: &IotaDID = IotaDID::try_from_borrowed(document.id())?;

    // Validate that the document controller (if any) conforms to the IotaDID specification.
    // This check is required to ensure the correctness of the `IotaDocument::controller()` method which
    // creates an `IotaDID::new_unchecked_ref()` from the underlying controller.
    document.controller().map_or(Ok(()), |c| IotaDID::check_validity(c))?;

    // Validate that the verification methods conform to the IotaDID specification.
    // This check is required to ensure the correctness of the `IotaDocument::methods()`,
    // `IotaDocument::resolve()`, `IotaDocument::try_resolve()`, IotaDocument::resolve_mut()`,
    // and IotaDocument::try_resolve_mut()` methods which creates an `IotaDID::new_unchecked_ref()`
    // from the underlying controller.
    //
    // We check these `document.verification_method()` and `document.verification_relationships()`
    // separately because they have separate types.
    for verification_method in document.verification_method().iter() {
      IotaVerificationMethod::check_validity(&*verification_method)?;
    }
    for method_ref in document.verification_relationships() {
      match method_ref {
        MethodRef::Embed(method) => IotaVerificationMethod::check_validity(method)?,
        MethodRef::Refer(did) => IotaDID::check_validity(did)?,
      }
    }

    let method = document
      .authentication()
      .head()
      .and_then(|method| document.resolve_ref(method))
      .ok_or(Error::MissingAuthenticationMethod)?;

    Self::check_authentication(method)?;

    // Ensure the authentication method DID matches the document DID
    if method.id().authority() != did.authority() {
      return Err(Error::InvalidDocumentAuthAuthority);
    }
    Ok(())
  }

  fn check_authentication<T>(method: &VerificationMethod<T>) -> Result<()> {
    IotaVerificationMethod::check_validity(method)?;

    // Ensure the verification method type is supported
    match method.key_type() {
      MethodType::Ed25519VerificationKey2018 => {}
      MethodType::MerkleKeyCollection2021 => return Err(Error::InvalidDocumentAuthType),
    }

    Ok(())
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

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID document [`id`][IotaDID].
  pub fn id(&self) -> &IotaDID {
    // SAFETY: We checked the validity of the DID Document ID in the
    // DID Document constructors; we don't provide mutable references so
    // the value cannot change with typical "safe" Rust.
    unsafe { IotaDID::new_unchecked_ref(self.document.id()) }
  }

  /// Returns a reference to the `IotaDocument` controller.
  pub fn controller(&self) -> Option<&IotaDID> {
    // SAFETY: Validity of controller checked in DID Document constructors.
    unsafe { self.document.controller().map(|d| IotaDID::new_unchecked_ref(d)) }
  }

  /// Returns a reference to the `CoreDocument` alsoKnownAs set.
  pub fn also_known_as(&self) -> &[Url] {
    self.document.also_known_as()
  }

  /// Returns the default authentication method of the DID document.
  pub fn authentication(&self) -> &IotaVerificationMethod {
    // This `unwrap` is "fine" - a valid document will
    // always have a resolvable authentication method.
    let method: &MethodRef = self.document.authentication().head().unwrap();
    let method: &VerificationMethod = self.document.resolve_ref(method).unwrap();

    // SAFETY: We don't allow invalid authentication methods.
    unsafe { IotaVerificationMethod::new_unchecked_ref(method) }
  }

  fn authentication_id(&self) -> &str {
    // This `unwrap` is "fine" - a valid document will
    // always have a resolvable authentication method.
    self.document.authentication().head().unwrap().id().as_str()
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

  /// Returns a reference to the custom DID Document properties.
  pub fn properties(&self) -> &Object {
    &self.document.properties().properties
  }

  /// Returns a mutable reference to the custom DID Document properties.
  pub fn properties_mut(&mut self) -> &mut Object {
    &mut self.document.properties_mut().properties
  }

  /// Returns a reference to the [`proof`][`Signature`], if one exists.
  pub fn proof(&self) -> Option<&Signature> {
    self.document.proof()
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns an iterator over all verification methods in the DID Document.
  pub fn methods(&self) -> impl Iterator<Item = &IotaVerificationMethod> {
    // SAFETY: Validity of verification methods checked in `IotaVerificationMethod::check_validity`.
    unsafe {
      self
        .document
        .methods()
        .map(|m| IotaVerificationMethod::new_unchecked_ref(m))
    }
  }

  /// Adds a new Verification Method to the DID Document.
  pub fn insert_method(&mut self, scope: MethodScope, method: IotaVerificationMethod) -> bool {
    self.document.insert_method(scope, method.into())
  }

  /// Removes all references to the specified Verification Method.
  pub fn remove_method(&mut self, did: &IotaDID) -> Result<()> {
    if self.authentication_id() == did.as_str() {
      return Err(Error::CannotRemoveAuthMethod);
    }

    self.document.remove_method(did.as_ref());

    Ok(())
  }

  /// Returns the first verification [`method`][`IotaverificationMethod`] with an `id` property
  /// matching the provided `query`.
  pub fn resolve<'query, Q>(&self, query: Q) -> Option<&IotaVerificationMethod>
  where
    Q: Into<MethodQuery<'query>>,
  {
    // SAFETY: Validity of verification methods checked in `IotaVerificationMethod::check_validity`.
    unsafe {
      self
        .document
        .resolve(query)
        .map(|m| IotaVerificationMethod::new_unchecked_ref(m))
    }
  }

  /// Returns the first verification [`method`][`IotaVerificationMethod`] with an `id` property
  /// matching the provided `query`.
  ///
  /// # Errors
  ///
  /// Fails if no matching verification `IotaVerificationMethod` is found.
  pub fn try_resolve<'query, Q>(&self, query: Q) -> Result<&IotaVerificationMethod>
  where
    Q: Into<MethodQuery<'query>>,
  {
    // SAFETY: Validity of verification methods checked in `IotaVerificationMethod::check_validity`.
    unsafe {
      self
        .document
        .try_resolve(query)
        .map(|m| IotaVerificationMethod::new_unchecked_ref(m))
        .map_err(Error::InvalidDoc)
    }
  }

  #[doc(hidden)]
  pub fn try_resolve_mut<'query, Q>(&mut self, query: Q) -> Result<&mut VerificationMethod>
  where
    Q: Into<MethodQuery<'query>>,
  {
    self.document.try_resolve_mut(query).map_err(Into::into)
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
    let key: String = self.authentication_id().to_string();

    self.document.sign_this(&key, secret).map_err(Into::into)
  }

  /// Creates a new [`DocumentSigner`] that can be used to create digital
  /// signatures from verification methods in this DID Document.
  pub fn signer<'base>(&'base self, secret: &'base SecretKey) -> Signer<'base, 'base, 'base> {
    self.document.signer(secret)
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

  /// Creates a new [`DocumentVerifier`] that can be used to verify signatures
  /// created with this DID Document.
  pub fn verifier(&self) -> Verifier<'_> {
    self.document.verifier()
  }

  /// Signs the provided data with the default authentication method.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the signature operation fails.
  pub fn sign_data<X>(&self, data: &mut X, secret: &SecretKey) -> Result<()>
  where
    X: Serialize + SetSignature + TryMethod,
  {
    self
      .document
      .signer(secret)
      .method(self.authentication_id())
      .sign(data)
      .map_err(Into::into)
  }

  /// Verifies the signature of the provided data.
  ///
  /// Note: It is assumed that the signature was created using a verification
  /// method contained within the DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, document
  /// serialization fails, or the verification operation fails.
  pub fn verify_data<X>(&self, data: &X) -> Result<()>
  where
    X: Serialize + TrySignature,
  {
    self.document.verifier().verify(data).map_err(Into::into)
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

    self.sign_data(&mut diff, secret)?;

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
    self.verify_data(diff)?;

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
    let network = Network::from_did(self.id());

    // Publish the DID Document to the Tangle.
    let message: MessageId = match client.into() {
      Some(client) if client.network() == network => client.publish_document(self).await?,
      Some(_) => return Err(Error::InvalidDIDNetwork),
      None => Client::from_network(network).await?.publish_document(self).await?,
    };

    // Update the `self` with the `MessageId` of the bundled transaction.
    self.set_message_id(message);

    Ok(())
  }

  /// Returns the Tangle address of the DID diff chain.
  pub fn diff_address(message_id: &MessageId) -> Result<String> {
    if message_id.is_null() {
      return Err(Error::InvalidDocumentMessageId);
    }

    Ok(IotaDID::encode_key(message_id.encode_hex().as_bytes()))
  }

  pub fn service(&self) -> &OrderedSet<DIDKey<Service>> {
    self.document.service()
  }

  pub fn insert_service(&mut self, service: Service) -> bool {
    if service.id().fragment().is_none() {
      false
    } else {
      self.document.service_mut().append(service.into())
    }
  }

  pub fn remove_service(&mut self, did: &IotaDID) -> Result<()> {
    self.document.service_mut().remove(did.as_ref());
    Ok(())
  }
}

impl<'a, 'b, 'c> IotaDocument {}

impl Display for IotaDocument {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.document, f)
  }
}

impl Debug for IotaDocument {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.document, f)
  }
}

impl TryFrom<BaseDocument> for IotaDocument {
  type Error = Error;

  fn try_from(other: BaseDocument) -> Result<Self, Self::Error> {
    IotaDocument::try_from_base(other)
  }
}

impl From<IotaDocument> for BaseDocument {
  fn from(other: IotaDocument) -> Self {
    other.document
  }
}

impl TryFrom<CoreDocument> for IotaDocument {
  type Error = Error;

  fn try_from(other: CoreDocument) -> Result<Self, Self::Error> {
    Self::try_from_core(other)
  }
}

impl TrySignature for IotaDocument {
  fn signature(&self) -> Option<&Signature> {
    self.document.proof()
  }
}

impl TrySignatureMut for IotaDocument {
  fn signature_mut(&mut self) -> Option<&mut Signature> {
    self.document.proof_mut()
  }
}

impl SetSignature for IotaDocument {
  fn set_signature(&mut self, signature: Signature) {
    self.document.set_proof(signature)
  }
}

impl TangleRef for IotaDocument {
  fn message_id(&self) -> &MessageId {
    &self.message_id
  }

  fn set_message_id(&mut self, message_id: MessageId) {
    self.message_id = message_id;
  }

  fn previous_message_id(&self) -> &MessageId {
    IotaDocument::previous_message_id(self)
  }

  fn set_previous_message_id(&mut self, message_id: MessageId) {
    IotaDocument::set_previous_message_id(self, message_id)
  }
}

#[cfg(test)]
mod tests {

  use crate::did::doc::IotaDocument;
  use crate::did::doc::IotaVerificationMethod;
  use crate::did::url::IotaDID;
  use identity_core::common::Value;
  use identity_core::convert::FromJson;
  use identity_core::convert::SerdeInto;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_core::crypto::PublicKey;
  use identity_core::crypto::SecretKey;
  use identity_did::did::DID;
  use identity_did::document::CoreDocument;
  use identity_did::service::Service;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodRef;
  use identity_did::verification::MethodType;
  use identity_did::verification::VerificationMethod;
  use std::collections::BTreeMap;

  const DID_ID: &str = "did:iota:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M";
  const DID_AUTH: &str = "did:iota:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M#authentication";
  const DID_TESTNET_ID: &str = "did:iota:test:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M";
  const DID_TESTNET_AUTH: &str = "did:iota:test:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M#authentication";

  fn valid_did() -> DID {
    DID_ID.parse().unwrap()
  }

  fn valid_properties() -> BTreeMap<String, Value> {
    let mut properties: BTreeMap<String, Value> = BTreeMap::default();
    properties.insert("created".to_string(), "2020-01-02T00:00:00Z".into());
    properties.insert("updated".to_string(), "2020-01-02T00:00:00Z".into());
    properties
  }

  fn core_verification_method(controller: &DID, fragment: &str) -> VerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(controller.join(fragment).unwrap())
      .controller(controller.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::new_b58(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn iota_verification_method(controller: &DID, fragment: &str) -> IotaVerificationMethod {
    IotaVerificationMethod::try_from_core(
      VerificationMethod::builder(Default::default())
        .id(controller.join(fragment).unwrap())
        .controller(controller.clone())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::new_b58(fragment.as_bytes()))
        .build()
        .unwrap(),
    )
    .unwrap()
  }

  fn iota_document_from_core(controller: &DID) -> IotaDocument {
    let mut properties: BTreeMap<String, Value> = BTreeMap::default();
    properties.insert("created".to_string(), "2020-01-01T00:00:00Z".into());
    properties.insert("updated".to_string(), "2020-01-02T00:00:00Z".into());

    IotaDocument::try_from_core(
      CoreDocument::builder(properties)
        .id(controller.clone())
        .verification_method(core_verification_method(&controller, "#key-1"))
        .verification_method(core_verification_method(&controller, "#key-2"))
        .verification_method(core_verification_method(&controller, "#key-3"))
        .authentication(core_verification_method(&controller, "#auth-key"))
        .authentication(controller.join("#key-3").unwrap())
        .key_agreement(controller.join("#key-4").unwrap())
        .controller(controller.clone())
        .build()
        .unwrap(),
    )
    .unwrap()
  }

  fn generate_testkey() -> KeyPair {
    let secret_key: Vec<u8> = vec![
      40, 185, 109, 70, 134, 119, 123, 37, 190, 254, 232, 186, 106, 48, 213, 63, 133, 223, 167, 126, 159, 43, 178, 4,
      190, 217, 52, 66, 92, 63, 69, 84,
    ];
    let public_key: Vec<u8> = vec![
      212, 151, 158, 35, 16, 178, 19, 27, 83, 109, 212, 138, 141, 134, 122, 246, 156, 148, 227, 69, 68, 251, 190, 31,
      25, 101, 230, 20, 130, 188, 121, 196,
    ];
    KeyPair::from((
      KeyType::Ed25519,
      PublicKey::from(public_key),
      SecretKey::from(secret_key),
    ))
  }

  fn compare_document(document: &IotaDocument) {
    assert_eq!(document.id().to_string(), DID_ID);
    assert_eq!(document.authentication_id(), DID_AUTH);
    assert_eq!(
      document.authentication().key_type(),
      MethodType::Ed25519VerificationKey2018
    );
    assert_eq!(
      document.authentication().key_data(),
      &MethodData::PublicKeyBase58(String::from("FJsXMk9UqpJf3ZTKnfEQAhvBrVLKMSx9ZeYwQME6c6tT"))
    );
  }

  fn compare_document_testnet(document: &IotaDocument) {
    assert_eq!(document.id().to_string(), DID_TESTNET_ID);
    assert_eq!(document.authentication_id(), DID_TESTNET_AUTH);
    assert_eq!(
      document.authentication().key_type(),
      MethodType::Ed25519VerificationKey2018
    );
    assert_eq!(
      document.authentication().key_data(),
      &MethodData::PublicKeyBase58(String::from("FJsXMk9UqpJf3ZTKnfEQAhvBrVLKMSx9ZeYwQME6c6tT"))
    );
  }

  #[test]
  fn test_invalid_try_from_core_invalid_id() {
    let invalid_did: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        // INVALID
        .id(invalid_did)
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_no_created_field() {
    let mut properties: BTreeMap<String, Value> = BTreeMap::default();
    properties.insert("updated".to_string(), "2020-01-02T00:00:00Z".into());
    // INVALID - missing "created" field.

    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(properties)
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_no_updated_field() {
    let mut properties: BTreeMap<String, Value> = BTreeMap::default();
    properties.insert("created".to_string(), "2020-01-02T00:00:00Z".into());
    // INVALID - missing "updated" field.

    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(properties)
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_invalid_controller() {
    let invalid_controller: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        .id(valid_did())
        // INVALID
        .authentication(core_verification_method(&invalid_controller, "#auth-key"))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_invalid_authentication_method_ref() {
    let invalid_ref: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        // INVALID
        .authentication(MethodRef::Refer(invalid_ref))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_invalid_assertion_method_ref() {
    let invalid_ref: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        // INVALID
        .assertion_method(MethodRef::Refer(invalid_ref))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_invalid_key_agreement_ref() {
    let invalid_ref: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        // INVALID
        .key_agreement(MethodRef::Refer(invalid_ref))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_invalid_capability_delegation_ref() {
    let invalid_ref: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        // INVALID
        .capability_delegation(MethodRef::Refer(invalid_ref))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_invalid_try_from_core_invalid_capability_invocation_ref() {
    let invalid_ref: DID = "did:invalid:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M"
      .parse()
      .unwrap();
    let doc = IotaDocument::try_from_core(
      CoreDocument::builder(valid_properties())
        .id(valid_did())
        .authentication(core_verification_method(&valid_did(), "#auth-key"))
        // INVALID
        .capability_invocation(MethodRef::Refer(invalid_ref))
        .build()
        .unwrap(),
    );

    assert!(doc.is_err());
  }

  #[test]
  fn test_new() {
    //from keypair
    let keypair: KeyPair = generate_testkey();
    let document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();
    compare_document(&document);

    //from authentication
    let method = document.authentication().to_owned();
    let document: IotaDocument = IotaDocument::from_authentication(method).unwrap();
    compare_document(&document);

    //from core
    let document: IotaDocument = IotaDocument::try_from_core(document.serde_into().unwrap()).unwrap();
    compare_document(&document);
  }

  #[test]
  fn test_from_keypair_with_network() {
    //from keypair
    let keypair: KeyPair = generate_testkey();
    let document: IotaDocument = IotaDocument::from_keypair_with_network(&keypair, "test").unwrap();
    compare_document_testnet(&document);
  }

  #[test]
  fn test_no_controler() {
    let keypair: KeyPair = generate_testkey();
    let document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();
    assert_eq!(document.controller(), None);
  }

  #[test]
  fn test_controller_from_core() {
    let controller: DID = valid_did();
    let document: IotaDocument = iota_document_from_core(&controller);
    let expected_controller: Option<IotaDID> = Some(IotaDID::try_from_owned(controller).unwrap());
    assert_eq!(document.controller(), expected_controller.as_ref());
  }

  #[test]
  fn test_methods_from_keypair() {
    let keypair: KeyPair = generate_testkey();
    let document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();

    // An IotaDocument created from a keypair has a single verification method, namely an
    // Ed25519 signature.
    let expected = IotaVerificationMethod::try_from_core(
      VerificationMethod::builder(Default::default())
        .id(
          "did:iota:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1M#authentication"
            .parse()
            .unwrap(),
        )
        .controller(valid_did())
        .key_type(MethodType::Ed25519VerificationKey2018)
        .key_data(MethodData::PublicKeyBase58(
          "FJsXMk9UqpJf3ZTKnfEQAhvBrVLKMSx9ZeYwQME6c6tT".into(),
        ))
        .build()
        .unwrap(),
    )
    .unwrap();

    let mut methods = document.methods();

    assert_eq!(methods.next(), Some(expected).as_ref());
    assert_eq!(methods.next(), None);
  }

  #[test]
  fn test_methods_from_core() {
    let controller: DID = valid_did();
    let document: IotaDocument = iota_document_from_core(&controller);
    let expected: Vec<IotaVerificationMethod> = vec![
      iota_verification_method(&controller, "#key-1"),
      iota_verification_method(&controller, "#key-2"),
      iota_verification_method(&controller, "#key-3"),
      iota_verification_method(&controller, "#auth-key"),
    ];

    let mut methods = document.methods();
    assert_eq!(methods.next(), Some(&expected[0]));
    assert_eq!(methods.next(), Some(&expected[1]));
    assert_eq!(methods.next(), Some(&expected[2]));
    assert_eq!(methods.next(), Some(&expected[3]));
    assert_eq!(methods.next(), None);
  }

  #[test]
  fn test_json() {
    let keypair: KeyPair = generate_testkey();
    let mut document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();

    let json_doc: String = document.to_string();
    let document2: IotaDocument = IotaDocument::from_json(&json_doc).unwrap();
    assert_eq!(document, document2);

    assert!(document.sign(keypair.secret()).is_ok());

    let json_doc: String = document.to_string();
    let document2: IotaDocument = IotaDocument::from_json(&json_doc).unwrap();
    assert_eq!(document, document2);
  }

  #[test]
  fn test_authentication() {
    let keypair: KeyPair = generate_testkey();
    let document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();

    assert!(IotaDocument::check_authentication(document.authentication()).is_ok());
  }

  #[test]
  fn test_document_services() {
    let keypair: KeyPair = generate_testkey();
    let mut document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();
    let service: Service = Service::from_json(
      r#"{
      "id":"did:iota:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1N#linked-domain",
      "type": "LinkedDomains",
      "serviceEndpoint": "https://bar.example.com"
    }"#,
    )
    .unwrap();
    document.insert_service(service);

    assert_eq!(1, document.service().len());

    document
      .remove_service(
        &IotaDID::parse("did:iota:HGE4tecHWL2YiZv5qAGtH7gaeQcaz2Z1CR15GWmMjY1N#linked-domain".to_string()).unwrap(),
      )
      .ok();
    assert_eq!(0, document.service().len());
  }
  #[test]
  fn test_relative_method_uri() {
    let keypair: KeyPair = generate_testkey();
    let mut document: IotaDocument = IotaDocument::from_keypair(&keypair).unwrap();

    assert!(document.proof().is_none());
    assert!(document.sign(keypair.secret()).is_ok());

    assert_eq!(document.proof().unwrap().verification_method(), "#authentication");
  }
}
