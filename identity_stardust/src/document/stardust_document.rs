// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;

use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Url;
use identity_core::convert::FmtJson;
use identity_core::crypto::GetSignature;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::ProofOptions;
use identity_core::crypto::SetSignature;
use identity_did::document::CoreDocument;
use identity_did::document::Document;
use identity_did::service::Service;
use identity_did::utils::DIDUrlQuery;
use identity_did::verifiable::DocumentSigner;
use identity_did::verifiable::VerifierOptions;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodUriType;
use identity_did::verification::TryMethod;
use identity_did::verification::VerificationMethod;
use serde::Deserialize;
use serde::Serialize;

use crate::error::Result;
use crate::NetworkName;
use crate::StardustDID;
use crate::StardustDIDUrl;
use crate::StardustDocumentMetadata;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;

/// A [`VerificationMethod`] adhering to the IOTA DID method specification.
pub type StardustVerificationMethod = VerificationMethod<StardustDID, Object>;

/// A [`Service`] adhering to the IOTA DID method specification.
pub type StardustService = Service<StardustDID, Object>;

/// A [`CoreDocument`] whose fields adhere to the IOTA DID method specification.
pub type StardustCoreDocument = CoreDocument<StardustDID>;

/// A DID Document adhering to the IOTA DID method specification.
///
/// This extends [`CoreDocument`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StardustDocument {
  #[serde(rename = "doc")]
  pub(crate) document: StardustCoreDocument,
  #[serde(rename = "meta")]
  pub metadata: StardustDocumentMetadata,
}

impl StardustDocument {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs an empty DID Document with a [`StardustDID::placeholder`] identifier
  /// for the given `network`.
  // TODO: always take Option<NetworkName> or `new_with_options` for a particular network?
  // TODO: store the network in the serialized state metadata? Currently it's lost during packing.
  pub fn new(network: &NetworkName) -> Self {
    Self::new_with_id(StardustDID::placeholder(network))
  }

  /// Constructs an empty DID Document with the given identifier.
  pub fn new_with_id(id: StardustDID) -> Self {
    // PANIC: constructing an empty DID Document is infallible, caught by tests otherwise.
    let document: StardustCoreDocument = CoreDocument::builder(Object::default())
      .id(id)
      .build()
      .expect("empty StardustDocument constructor failed");
    let metadata: StardustDocumentMetadata = StardustDocumentMetadata::new();
    Self { document, metadata }
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID document identifier.
  pub fn id(&self) -> &StardustDID {
    self.document.id()
  }

  /// Returns a reference to the DID controllers.
  ///
  /// NOTE: controllers are determined by the `state_controller` unlock condition of the output
  /// during resolution and are omitted when publishing.
  pub fn controller(&self) -> Option<&OneOrSet<StardustDID>> {
    self.document.controller()
  }

  /// Returns a reference to the `alsoKnownAs` set.
  pub fn also_known_as(&self) -> &OrderedSet<Url> {
    self.document.also_known_as()
  }

  /// Returns a mutable reference to the `alsoKnownAs` set.
  pub fn also_known_as_mut(&mut self) -> &mut OrderedSet<Url> {
    self.document.also_known_as_mut()
  }

  /// Returns a reference to the underlying [`StardustCoreDocument`].
  pub fn core_document(&self) -> &StardustCoreDocument {
    &self.document
  }

  /// Returns a mutable reference to the underlying [`StardustCoreDocument`].
  ///
  /// WARNING: mutating the inner document directly bypasses restrictions and
  /// may have undesired consequences.
  pub fn core_document_mut(&mut self) -> &mut StardustCoreDocument {
    &mut self.document
  }

  /// Returns a reference to the custom DID Document properties.
  pub fn properties(&self) -> &Object {
    self.document.properties()
  }

  /// Returns a mutable reference to the custom DID Document properties.
  pub fn properties_mut(&mut self) -> &mut Object {
    self.document.properties_mut()
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all [`StardustService`]s in the document.
  pub fn service(&self) -> &OrderedSet<StardustService> {
    self.document.service()
  }

  /// Add a new [`StardustService`] to the document.
  ///
  /// Returns `true` if the service was added.
  pub fn insert_service(&mut self, service: StardustService) -> bool {
    if service.id().fragment().is_none() {
      false
    } else {
      self.core_document_mut().service_mut().append(service)
    }
  }

  /// Remove a [`StardustService`] identified by the given [`StardustDIDUrl`] from the document.
  ///
  /// Returns `true` if a service was removed.
  pub fn remove_service(&mut self, did_url: &StardustDIDUrl) -> bool {
    self.core_document_mut().service_mut().remove(did_url)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns an iterator over all [`StardustVerificationMethod`] in the DID Document.
  pub fn methods(&self) -> impl Iterator<Item = &StardustVerificationMethod> {
    self.document.methods()
  }

  /// Adds a new [`StardustVerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method with the same fragment already exists.
  pub fn insert_method(&mut self, method: StardustVerificationMethod, scope: MethodScope) -> Result<()> {
    Ok(self.core_document_mut().insert_method(method, scope)?)
  }

  /// Removes all references to the specified [`StardustVerificationMethod`].
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist.
  pub fn remove_method(&mut self, did_url: &StardustDIDUrl) -> Result<()> {
    Ok(self.core_document_mut().remove_method(did_url)?)
  }

  /// Attaches the relationship to the given method, if the method exists.
  ///
  /// Note: The method needs to be in the set of verification methods,
  /// so it cannot be an embedded one.
  pub fn attach_method_relationship(
    &mut self,
    did_url: &StardustDIDUrl,
    relationship: MethodRelationship,
  ) -> Result<bool> {
    Ok(
      self
        .core_document_mut()
        .attach_method_relationship(did_url, relationship)?,
    )
  }

  /// Detaches the given relationship from the given method, if the method exists.
  pub fn detach_method_relationship(
    &mut self,
    did_url: &StardustDIDUrl,
    relationship: MethodRelationship,
  ) -> Result<bool> {
    Ok(
      self
        .core_document_mut()
        .detach_method_relationship(did_url, relationship)?,
    )
  }

  /// Returns the first [`StardustVerificationMethod`] with an `id` property matching the
  /// provided `query` and the verification relationship specified by `scope` if present.
  ///
  /// WARNING: improper usage of this allows violating the uniqueness of the verification method
  /// sets.
  pub fn resolve_method_mut<'query, Q>(
    &mut self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&mut StardustVerificationMethod>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.document.resolve_method_mut(query, scope)
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Creates a new [`DocumentSigner`] that can be used to create digital signatures
  /// from verification methods in this DID Document.
  pub fn signer<'base>(&'base self, private_key: &'base PrivateKey) -> DocumentSigner<'base, '_, StardustDID> {
    self.document.signer(private_key)
  }

  /// Signs the provided `data` with the verification method specified by `method_query`.
  /// See [`StardustDocument::signer`] for creating signatures with a builder pattern.
  ///
  /// NOTE: does not validate whether `private_key` corresponds to the verification method.
  /// See [`StardustDocument::verify_data`].
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used, data
  /// serialization fails, or the signature operation fails.
  pub fn sign_data<'query, 'this: 'query, X, Q>(
    &'this self,
    data: &mut X,
    private_key: &'this PrivateKey,
    method_query: Q,
    options: ProofOptions,
  ) -> Result<()>
  where
    X: Serialize + SetSignature + TryMethod,
    Q: Into<DIDUrlQuery<'query>>,
  {
    self
      .signer(private_key)
      .method(method_query)
      .options(options)
      .sign(data)
      .map_err(Into::into)
  }

  // ===========================================================================
  // Packing
  // ===========================================================================

  /// Serializes the document for inclusion in an Alias Output's state metadata
  /// with the default [`StateMetadataEncoding`].
  pub fn pack(self) -> Result<Vec<u8>> {
    self.pack_with_encoding(StateMetadataEncoding::Json)
  }

  /// Serializes the document for inclusion in an Alias Output's state metadata.
  pub fn pack_with_encoding(self, encoding: StateMetadataEncoding) -> Result<Vec<u8>> {
    StateMetadataDocument::from(self).pack(encoding)
  }

  /// Deserializes the document from the state metadata bytes of an Alias Output.
  ///
  /// NOTE: `did` is required since it is omitted from the serialized DID Document and
  /// cannot be inferred from the state metadata. It also indicates the network, which is not
  /// encoded in the `AliasId` alone.
  pub fn unpack(did: &StardustDID, state_metadata: &[u8]) -> Result<StardustDocument> {
    StateMetadataDocument::unpack(state_metadata).and_then(|doc| doc.into_stardust_document(did))
  }
}

impl Document for StardustDocument {
  type D = StardustDID;
  type U = Object;
  type V = Object;

  fn id(&self) -> &Self::D {
    StardustDocument::id(self)
  }

  fn resolve_service<'query, 'me, Q>(&'me self, query: Q) -> Option<&Service<Self::D, Self::V>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.document.resolve_service(query)
  }

  fn resolve_method<'query, 'me, Q>(
    &'me self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&VerificationMethod<Self::D, Self::U>>
  where
    Q: Into<DIDUrlQuery<'query>>,
  {
    self.document.resolve_method(query, scope)
  }

  fn verify_data<X>(&self, data: &X, options: &VerifierOptions) -> identity_did::Result<()>
  where
    X: Serialize + GetSignature + ?Sized,
  {
    self.document.verify_data(data, options)
  }
}

#[cfg(feature = "revocation-bitmap")]
mod iota_document_revocation {
  use identity_did::utils::DIDUrlQuery;

  use crate::Error;
  use crate::Result;

  use super::StardustDocument;

  impl StardustDocument {
    /// If the document has a [`RevocationBitmap`](identity_did::revocation::RevocationBitmap)
    /// service identified by `service_query`, revoke all specified `indices`.
    pub fn revoke_credentials<'query, 'me, Q>(&mut self, service_query: Q, indices: &[u32]) -> Result<()>
    where
      Q: Into<DIDUrlQuery<'query>>,
    {
      self
        .core_document_mut()
        .revoke_credentials(service_query, indices)
        .map_err(Error::RevocationError)
    }

    /// If the document has a [`RevocationBitmap`](identity_did::revocation::RevocationBitmap)
    /// service with an id by `service_query`, unrevoke all specified `indices`.
    pub fn unrevoke_credentials<'query, 'me, Q>(&'me mut self, service_query: Q, indices: &[u32]) -> Result<()>
    where
      Q: Into<DIDUrlQuery<'query>>,
    {
      self
        .core_document_mut()
        .unrevoke_credentials(service_query, indices)
        .map_err(Error::RevocationError)
    }
  }
}

impl From<StardustDocument> for StardustCoreDocument {
  fn from(document: StardustDocument) -> Self {
    document.document
  }
}

impl From<(StardustCoreDocument, StardustDocumentMetadata)> for StardustDocument {
  fn from((document, metadata): (StardustCoreDocument, StardustDocumentMetadata)) -> Self {
    Self { document, metadata }
  }
}

impl Display for StardustDocument {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_json(f)
  }
}

impl TryMethod for StardustDocument {
  const TYPE: MethodUriType = MethodUriType::Absolute;
}

#[cfg(test)]
mod tests {
  use identity_core::common::Timestamp;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::did::DID;
  use identity_did::verifiable::VerifiableProperties;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodType;

  use super::*;

  fn valid_did() -> StardustDID {
    "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  fn generate_method(controller: &StardustDID, fragment: &str) -> StardustVerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(controller.to_url().join(fragment).unwrap())
      .controller(controller.clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn generate_document(id: &StardustDID) -> StardustDocument {
    let mut metadata: StardustDocumentMetadata = StardustDocumentMetadata::new();
    metadata.created = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());
    metadata.updated = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());

    let document: StardustCoreDocument = StardustCoreDocument::builder(Object::default())
      .id(id.clone())
      .controller(id.clone())
      .verification_method(generate_method(id, "#key-1"))
      .verification_method(generate_method(id, "#key-2"))
      .verification_method(generate_method(id, "#key-3"))
      .authentication(generate_method(id, "#auth-key"))
      .authentication(id.to_url().join("#key-3").unwrap())
      .build()
      .unwrap();

    StardustDocument::from((document, metadata))
  }

  #[test]
  fn test_new() {
    // VALID new().
    let network: NetworkName = NetworkName::try_from("test").unwrap();
    let placeholder: StardustDID = StardustDID::placeholder(&network);
    let doc1: StardustDocument = StardustDocument::new(&network);
    assert_eq!(doc1.id().network_str(), network.as_ref());
    assert_eq!(doc1.id().tag(), placeholder.tag());
    assert_eq!(doc1.id(), &placeholder);
    assert_eq!(doc1.methods().count(), 0);
    assert!(doc1.service().is_empty());

    // VALID new_with_id().
    let did: StardustDID = valid_did();
    let doc2: StardustDocument = StardustDocument::new_with_id(did.clone());
    assert_eq!(doc2.id(), &did);
    assert_eq!(doc2.methods().count(), 0);
    assert!(doc2.service().is_empty());
  }

  #[test]
  fn test_methods() {
    let controller: StardustDID = valid_did();
    let document: StardustDocument = generate_document(&controller);
    let expected: Vec<StardustVerificationMethod> = vec![
      generate_method(&controller, "#key-1"),
      generate_method(&controller, "#key-2"),
      generate_method(&controller, "#key-3"),
      generate_method(&controller, "#auth-key"),
    ];

    let mut methods = document.methods();
    assert_eq!(methods.next(), Some(&expected[0]));
    assert_eq!(methods.next(), Some(&expected[1]));
    assert_eq!(methods.next(), Some(&expected[2]));
    assert_eq!(methods.next(), Some(&expected[3]));
    assert_eq!(methods.next(), None);
  }

  #[test]
  fn test_verify_data_with_scope() {
    fn generate_data() -> VerifiableProperties {
      use identity_core::json;
      let mut properties: VerifiableProperties = VerifiableProperties::default();
      properties.properties.insert("int_key".to_owned(), json!(1));
      properties.properties.insert("str".to_owned(), json!("some value"));
      properties
        .properties
        .insert("object".to_owned(), json!({ "inner": 42 }));
      properties
    }

    let mut document: StardustDocument = StardustDocument::new_with_id(valid_did());

    // Try sign using each type of verification relationship.
    for scope in [
      MethodScope::assertion_method(),
      MethodScope::authentication(),
      MethodScope::capability_delegation(),
      MethodScope::capability_invocation(),
      MethodScope::key_agreement(),
      MethodScope::VerificationMethod,
    ] {
      // Add a new method.
      let key_new: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
      let method_fragment = format!("{}-1", scope.as_str().to_ascii_lowercase());
      let method_new: StardustVerificationMethod = StardustVerificationMethod::new(
        document.id().clone(),
        key_new.type_(),
        key_new.public(),
        method_fragment.as_str(),
      )
      .unwrap();
      document.insert_method(method_new, scope).unwrap();

      // Sign and verify data.
      let mut data = generate_data();
      document
        .sign_data(
          &mut data,
          key_new.private(),
          method_fragment.as_str(),
          ProofOptions::default(),
        )
        .unwrap();
      // Signature should still be valid for every scope.
      assert!(document.verify_data(&data, &VerifierOptions::default()).is_ok());

      // Ensure only the correct scope is valid.
      for scope_check in [
        MethodScope::assertion_method(),
        MethodScope::authentication(),
        MethodScope::capability_delegation(),
        MethodScope::capability_invocation(),
        MethodScope::key_agreement(),
        MethodScope::VerificationMethod,
      ] {
        let result = document.verify_data(&data, &VerifierOptions::new().method_scope(scope_check));
        // Any other scope should fail validation.
        if scope_check == scope {
          assert!(result.is_ok());
        } else {
          assert!(result.is_err());
        }
      }
    }
  }

  #[test]
  fn test_services() {
    // VALID: add one service.
    let mut document: StardustDocument = StardustDocument::new_with_id(valid_did());
    let url1: StardustDIDUrl = document.id().to_url().join("#linked-domain").unwrap();
    let service1: StardustService = Service::from_json(&format!(
      r#"{{
      "id":"{}",
      "type": "LinkedDomains",
      "serviceEndpoint": "https://bar.example.com"
    }}"#,
      url1
    ))
    .unwrap();
    document.insert_service(service1.clone());
    assert_eq!(1, document.service().len());
    assert_eq!(document.resolve_service(&url1), Some(&service1));
    assert_eq!(document.resolve_service("#linked-domain"), Some(&service1));
    assert_eq!(document.resolve_service("linked-domain"), Some(&service1));
    assert_eq!(document.resolve_service(""), None);
    assert_eq!(document.resolve_service("#other"), None);

    // VALID: add two services.
    let url2: StardustDIDUrl = document.id().to_url().join("#revocation").unwrap();
    let service2: StardustService = Service::from_json(&format!(
      r#"{{
      "id":"{}",
      "type": "RevocationBitmap2022",
      "serviceEndpoint": "data:,blah"
    }}"#,
      url2
    ))
    .unwrap();
    document.insert_service(service2.clone());
    assert_eq!(2, document.service().len());
    assert_eq!(document.resolve_service(&url2), Some(&service2));
    assert_eq!(document.resolve_service("#revocation"), Some(&service2));
    assert_eq!(document.resolve_service("revocation"), Some(&service2));
    assert_eq!(document.resolve_service(""), None);
    assert_eq!(document.resolve_service("#other"), None);

    // INVALID: insert service with duplicate fragment fails.
    let duplicate: StardustService = Service::from_json(&format!(
      r#"{{
      "id":"{}",
      "type": "DuplicateService",
      "serviceEndpoint": "data:,duplicate"
    }}"#,
      url1
    ))
    .unwrap();
    assert!(!document.insert_service(duplicate.clone()));
    assert_eq!(2, document.service().len());
    let resolved: &StardustService = document.resolve_service(&url1).unwrap();
    assert_eq!(resolved, &service1);
    assert_ne!(resolved, &duplicate);

    // VALID: remove services.
    assert!(document.remove_service(&url1));
    assert_eq!(1, document.service().len());
    let last_service: &StardustService = document.resolve_service(&url2).unwrap();
    assert_eq!(last_service, &service2);

    assert!(document.remove_service(&url2));
    assert_eq!(0, document.service().len());
  }

  #[test]
  fn test_document_equality() {
    let mut original_doc: StardustDocument = StardustDocument::new_with_id(valid_did());
    let keypair1: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let method1: StardustVerificationMethod = StardustVerificationMethod::new(
      original_doc.id().to_owned(),
      keypair1.type_(),
      keypair1.public(),
      "test-0",
    )
    .unwrap();
    original_doc
      .insert_method(method1, MethodScope::capability_invocation())
      .unwrap();

    // Update the key material of the existing verification method #test-0.
    let mut doc1 = original_doc.clone();
    let keypair2: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let method2: StardustVerificationMethod =
      StardustVerificationMethod::new(doc1.id().to_owned(), keypair2.type_(), keypair2.public(), "test-0").unwrap();

    doc1
      .remove_method(&doc1.id().to_url().join("#test-0").unwrap())
      .unwrap();
    doc1
      .insert_method(method2, MethodScope::capability_invocation())
      .unwrap();

    // Even though the method fragment is the same, the key material has been updated
    // so the two documents are expected to not be equal.
    assert_ne!(original_doc, doc1);

    let mut doc2 = doc1.clone();
    let keypair3: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let method3: StardustVerificationMethod =
      StardustVerificationMethod::new(doc1.id().to_owned(), keypair3.type_(), keypair3.public(), "test-0").unwrap();

    let insertion_result = doc2.insert_method(method3, MethodScope::capability_invocation());

    // Nothing was inserted, because a method with the same fragment already existed.
    assert!(insertion_result.is_err());
    assert_eq!(doc1, doc2);
  }

  #[test]
  fn test_json_roundtrip() {
    let document: StardustDocument = generate_document(&valid_did());

    let ser: String = document.to_json().unwrap();
    let de: StardustDocument = StardustDocument::from_json(&ser).unwrap();
    assert_eq!(document, de);
  }

  #[test]
  fn test_json_fieldnames() {
    let document: StardustDocument = StardustDocument::new_with_id(valid_did());
    let serialization: String = document.to_json().unwrap();
    assert_eq!(
      serialization,
      format!("{{\"doc\":{},\"meta\":{}}}", document.document, document.metadata)
    );
  }
}
