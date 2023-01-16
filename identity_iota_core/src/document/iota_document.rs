// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

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

use crate::error::Result;
use crate::Error;
use crate::IotaDID;
use crate::IotaDIDUrl;
use crate::IotaDocumentMetadata;
use crate::NetworkName;
use crate::StateMetadataDocument;
use crate::StateMetadataEncoding;

/// A [`VerificationMethod`] adhering to the IOTA DID method specification.
pub type IotaVerificationMethod = VerificationMethod<IotaDID, Object>;

/// A [`Service`] adhering to the IOTA DID method specification.
pub type IotaService = Service<IotaDID, Object>;

/// A [`CoreDocument`] whose fields adhere to the IOTA DID method specification.
pub type IotaCoreDocument = CoreDocument<IotaDID>;

/// A DID Document adhering to the IOTA DID method specification.
///
/// This extends [`CoreDocument`].
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct IotaDocument {
  #[serde(rename = "doc")]
  pub(crate) document: IotaCoreDocument,
  #[serde(rename = "meta")]
  pub metadata: IotaDocumentMetadata,
}

impl IotaDocument {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs an empty DID Document with a [`IotaDID::placeholder`] identifier
  /// for the given `network`.
  // TODO: always take Option<NetworkName> or `new_with_options` for a particular network?
  // TODO: store the network in the serialized state metadata? Currently it's lost during packing.
  pub fn new(network: &NetworkName) -> Self {
    Self::new_with_id(IotaDID::placeholder(network))
  }

  /// Constructs an empty DID Document with the given identifier.
  pub fn new_with_id(id: IotaDID) -> Self {
    // PANIC: constructing an empty DID Document is infallible, caught by tests otherwise.
    let document: IotaCoreDocument = CoreDocument::builder(Object::default())
      .id(id)
      .build()
      .expect("empty IotaDocument constructor failed");
    let metadata: IotaDocumentMetadata = IotaDocumentMetadata::new();
    Self { document, metadata }
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns the DID document identifier.
  pub fn id(&self) -> &IotaDID {
    self.document.id()
  }

  /// Returns a reference to the DID controllers.
  ///
  /// NOTE: controllers are determined by the `state_controller` unlock condition of the output
  /// during resolution and are omitted when publishing.
  pub fn controller(&self) -> Option<&OneOrSet<IotaDID>> {
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

  /// Returns a reference to the underlying [`IotaCoreDocument`].
  pub fn core_document(&self) -> &IotaCoreDocument {
    &self.document
  }

  /// Returns a mutable reference to the underlying [`IotaCoreDocument`].
  ///
  /// WARNING: mutating the inner document directly bypasses restrictions and
  /// may have undesired consequences.
  pub fn core_document_mut(&mut self) -> &mut IotaCoreDocument {
    &mut self.document
  }

  /// Returns a reference to the custom DID Document properties.
  pub fn properties(&self) -> &Object {
    self.document.properties()
  }

  /// Returns a mutable reference to the custom DID Document properties.
  ///
  /// # Warning
  ///
  /// The properties returned are not checked against the standard fields in a [`CoreDocument`]. Incautious use can have
  /// undesired consequences such as key collision when attempting to serialize the document or distinct resources (such
  /// as services and methods) being identified by the same DID URL.  
  pub fn properties_mut_unchecked(&mut self) -> &mut Object {
    self.document.properties_mut_unchecked()
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all [`IotaService`]s in the document.
  pub fn service(&self) -> &OrderedSet<IotaService> {
    self.document.service()
  }

  /// Add a new [`IotaService`] to the document.
  ///
  /// # Errors
  /// An error is returned if there already exists a service or (verification) method with
  /// the same identifier in the document.  
  pub fn insert_service(&mut self, service: IotaService) -> Result<()> {
    self
      .core_document_mut()
      .insert_service(service)
      .map_err(Error::InvalidDoc)
  }

  /// Remove and return the [`IotaService`] identified by the given [`IotaDIDUrl`] from the document.
  ///
  /// `None` is returned if the service does not exist in the document.
  pub fn remove_service(&mut self, did_url: &IotaDIDUrl) -> Option<IotaService> {
    self.core_document_mut().remove_service(did_url)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns a `Vec` of verification method references whose verification relationship matches `scope`.
  ///
  /// If `scope` is `None`, an iterator over all **embedded** methods is returned.
  pub fn methods(&self, scope: Option<MethodScope>) -> Vec<&IotaVerificationMethod> {
    self.document.methods(scope)
  }

  /// Adds a new [`IotaVerificationMethod`] to the document in the given [`MethodScope`].
  ///
  /// # Errors
  ///
  /// Returns an error if a method with the same fragment already exists.
  pub fn insert_method(&mut self, method: IotaVerificationMethod, scope: MethodScope) -> Result<()> {
    self
      .core_document_mut()
      .insert_method(method, scope)
      .map_err(Error::InvalidDoc)
  }

  /// Removes all references to the specified [`IotaVerificationMethod`].
  ///
  /// # Errors
  ///
  /// Returns an error if the method does not exist.
  pub fn remove_method(&mut self, did_url: &IotaDIDUrl) -> Option<IotaVerificationMethod> {
    self.core_document_mut().remove_method(did_url)
  }

  /// Attaches the relationship to the given method, if the method exists.
  ///
  /// Note: The method needs to be in the set of verification methods,
  /// so it cannot be an embedded one.
  pub fn attach_method_relationship(&mut self, did_url: &IotaDIDUrl, relationship: MethodRelationship) -> Result<bool> {
    self
      .core_document_mut()
      .attach_method_relationship(did_url, relationship)
      .map_err(Error::InvalidDoc)
  }

  /// Detaches the given relationship from the given method, if the method exists.
  pub fn detach_method_relationship(&mut self, did_url: &IotaDIDUrl, relationship: MethodRelationship) -> Result<bool> {
    self
      .core_document_mut()
      .detach_method_relationship(did_url, relationship)
      .map_err(Error::InvalidDoc)
  }

  /// Returns the first [`IotaVerificationMethod`] with an `id` property matching the
  /// provided `query` and the verification relationship specified by `scope` if present.
  ///
  /// # Warning
  ///
  /// Incorrect use of this method can lead to distinct document resources being identified by the same DID URL.
  pub fn resolve_method_mut<'query, Q>(
    &mut self,
    query: Q,
    scope: Option<MethodScope>,
  ) -> Option<&mut IotaVerificationMethod>
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
  pub fn signer<'base>(&'base self, private_key: &'base PrivateKey) -> DocumentSigner<'base, '_, IotaDID> {
    self.document.signer(private_key)
  }

  /// Signs the provided `data` with the verification method specified by `method_query`.
  /// See [`IotaDocument::signer`] for creating signatures with a builder pattern.
  ///
  /// NOTE: does not validate whether `private_key` corresponds to the verification method.
  /// See [`IotaDocument::verify_data`].
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
      .map_err(|err| Error::SigningError(err.into()))
  }

  // ===========================================================================
  // Packing
  // ===========================================================================

  /// Serializes the document for inclusion in an Alias Output's state metadata
  /// with the default [`StateMetadataEncoding`].
  pub fn pack(self) -> Result<Vec<u8>> {
    self.pack_with_encoding(StateMetadataEncoding::default())
  }

  /// Serializes the document for inclusion in an Alias Output's state metadata.
  pub fn pack_with_encoding(self, encoding: StateMetadataEncoding) -> Result<Vec<u8>> {
    StateMetadataDocument::from(self).pack(encoding)
  }
}

#[cfg(feature = "client")]
mod client_document {
  use std::ops::Deref;

  use crate::block::address::Address;
  use crate::block::output::AliasId;
  use crate::block::output::AliasOutput;
  use crate::block::output::Output;
  use crate::block::output::OutputId;
  use crate::block::payload::transaction::TransactionEssence;
  use crate::block::payload::Payload;
  use crate::block::Block;
  use crate::error::Result;
  use crate::Error;
  use crate::NetworkName;

  use super::*;

  impl IotaDocument {
    // ===========================================================================
    // Unpacking
    // ===========================================================================

    /// Deserializes the document from an Alias Output.
    ///
    /// If `allow_empty` is true, this will return an empty DID document marked as `deactivated`
    /// if `state_metadata` is empty.
    ///
    /// NOTE: `did` is required since it is omitted from the serialized DID Document and
    /// cannot be inferred from the state metadata. It also indicates the network, which is not
    /// encoded in the `AliasId` alone.
    pub fn unpack_from_output(did: &IotaDID, alias_output: &AliasOutput, allow_empty: bool) -> Result<IotaDocument> {
      let mut document: IotaDocument = if alias_output.state_metadata().is_empty() && allow_empty {
        let mut empty_document = IotaDocument::new_with_id(did.clone());
        empty_document.metadata.created = None;
        empty_document.metadata.updated = None;
        empty_document.metadata.deactivated = Some(true);
        empty_document
      } else {
        StateMetadataDocument::unpack(alias_output.state_metadata()).and_then(|doc| doc.into_iota_document(did))?
      };

      document.set_controller_and_governor_addresses(alias_output, &did.network_str().to_owned().try_into()?);
      Ok(document)
    }

    fn set_controller_and_governor_addresses(&mut self, alias_output: &AliasOutput, network_name: &NetworkName) {
      self.metadata.governor_address = Some(alias_output.governor_address().to_bech32(network_name));
      self.metadata.state_controller_address = Some(alias_output.state_controller_address().to_bech32(network_name));

      // Overwrite the DID Document controller.
      let controller_did: Option<IotaDID> = match alias_output.state_controller_address() {
        Address::Alias(alias_address) => Some(IotaDID::new(alias_address.alias_id(), network_name)),
        _ => None,
      };
      *self.core_document_mut().controller_mut() = controller_did.map(OneOrSet::new_one);
    }

    /// Returns all DID documents of the Alias Outputs contained in the block's transaction payload
    /// outputs, if any.
    ///
    /// Errors if any Alias Output does not contain a valid or empty DID Document.
    pub fn unpack_from_block(network: &NetworkName, block: &Block) -> Result<Vec<IotaDocument>> {
      let mut documents = Vec::new();

      if let Some(Payload::Transaction(tx_payload)) = block.payload() {
        let TransactionEssence::Regular(regular) = tx_payload.essence();

        for (index, output) in regular.outputs().iter().enumerate() {
          if let Output::Alias(alias_output) = output {
            let alias_id = if alias_output.alias_id().is_null() {
              AliasId::from(
                &OutputId::new(
                  tx_payload.id(),
                  index
                    .try_into()
                    .map_err(|_| Error::OutputIdConversionError(format!("output index {index} must fit into a u16")))?,
                )
                .map_err(|err| Error::OutputIdConversionError(err.to_string()))?,
              )
            } else {
              alias_output.alias_id().to_owned()
            };

            let did: IotaDID = IotaDID::new(alias_id.deref(), network);
            documents.push(IotaDocument::unpack_from_output(&did, alias_output, true)?);
          }
        }
      }

      Ok(documents)
    }
  }
}

impl Document for IotaDocument {
  type D = IotaDID;
  type U = Object;
  type V = Object;

  fn id(&self) -> &Self::D {
    IotaDocument::id(self)
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

  use super::IotaDocument;

  impl IotaDocument {
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

impl From<IotaDocument> for IotaCoreDocument {
  fn from(document: IotaDocument) -> Self {
    document.document
  }
}

impl From<IotaDocument> for CoreDocument {
  fn from(document: IotaDocument) -> Self {
    document.document.map_unchecked(Into::into, |id| id)
  }
}

impl From<(IotaCoreDocument, IotaDocumentMetadata)> for IotaDocument {
  fn from((document, metadata): (IotaCoreDocument, IotaDocumentMetadata)) -> Self {
    Self { document, metadata }
  }
}

impl Display for IotaDocument {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_json(f)
  }
}

impl TryMethod for IotaDocument {
  const TYPE: MethodUriType = MethodUriType::Absolute;
}

#[cfg(test)]
mod tests {
  use identity_core::common::Timestamp;
  use identity_core::convert::FromJson;
  use identity_core::convert::ToJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_did::verifiable::VerifiableProperties;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodType;
  use identity_did::DID;
  use iota_types::block::protocol::ProtocolParameters;

  use crate::block::address::Address;
  use crate::block::address::AliasAddress;
  use crate::block::output::unlock_condition::GovernorAddressUnlockCondition;
  use crate::block::output::unlock_condition::StateControllerAddressUnlockCondition;
  use crate::block::output::AliasId;
  use crate::block::output::AliasOutput;
  use crate::block::output::AliasOutputBuilder;
  use crate::block::output::UnlockCondition;

  use super::*;

  fn valid_did() -> IotaDID {
    "did:iota:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      .parse()
      .unwrap()
  }

  fn generate_method(controller: &IotaDID, fragment: &str) -> IotaVerificationMethod {
    VerificationMethod::builder(Default::default())
      .id(controller.to_url().join(fragment).unwrap())
      .controller(controller.clone())
      .type_(MethodType::Ed25519VerificationKey2018)
      .data(MethodData::new_multibase(fragment.as_bytes()))
      .build()
      .unwrap()
  }

  fn generate_document(id: &IotaDID) -> IotaDocument {
    let mut metadata: IotaDocumentMetadata = IotaDocumentMetadata::new();
    metadata.created = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());
    metadata.updated = Some(Timestamp::parse("2020-01-02T00:00:00Z").unwrap());

    let document: IotaCoreDocument = IotaCoreDocument::builder(Object::default())
      .id(id.clone())
      .controller(id.clone())
      .verification_method(generate_method(id, "#key-1"))
      .verification_method(generate_method(id, "#key-2"))
      .verification_method(generate_method(id, "#key-3"))
      .authentication(generate_method(id, "#auth-key"))
      .authentication(id.to_url().join("#key-3").unwrap())
      .build()
      .unwrap();

    IotaDocument::from((document, metadata))
  }

  #[test]
  fn test_new() {
    // VALID new().
    let network: NetworkName = NetworkName::try_from("test").unwrap();
    let placeholder: IotaDID = IotaDID::placeholder(&network);
    let doc1: IotaDocument = IotaDocument::new(&network);
    assert_eq!(doc1.id().network_str(), network.as_ref());
    assert_eq!(doc1.id().tag(), placeholder.tag());
    assert_eq!(doc1.id(), &placeholder);
    assert_eq!(doc1.methods(None).len(), 0);
    assert!(doc1.service().is_empty());

    // VALID new_with_id().
    let did: IotaDID = valid_did();
    let doc2: IotaDocument = IotaDocument::new_with_id(did.clone());
    assert_eq!(doc2.id(), &did);
    assert_eq!(doc2.methods(None).len(), 0);
    assert!(doc2.service().is_empty());
  }

  #[test]
  fn test_methods() {
    let controller: IotaDID = valid_did();
    let document: IotaDocument = generate_document(&controller);
    let expected: Vec<IotaVerificationMethod> = vec![
      generate_method(&controller, "#key-1"),
      generate_method(&controller, "#key-2"),
      generate_method(&controller, "#key-3"),
      generate_method(&controller, "#auth-key"),
    ];

    let mut methods = document.methods(None).into_iter();
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

    let mut document: IotaDocument = IotaDocument::new_with_id(valid_did());

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
      let method_new: IotaVerificationMethod = IotaVerificationMethod::new(
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
    let mut document: IotaDocument = IotaDocument::new_with_id(valid_did());
    let url1: IotaDIDUrl = document.id().to_url().join("#linked-domain").unwrap();
    let service1: IotaService = Service::from_json(&format!(
      r#"{{
      "id":"{}",
      "type": "LinkedDomains",
      "serviceEndpoint": "https://bar.example.com"
    }}"#,
      url1
    ))
    .unwrap();
    assert!(document.insert_service(service1.clone()).is_ok());
    assert_eq!(1, document.service().len());
    assert_eq!(document.resolve_service(&url1), Some(&service1));
    assert_eq!(document.resolve_service("#linked-domain"), Some(&service1));
    assert_eq!(document.resolve_service("linked-domain"), Some(&service1));
    assert_eq!(document.resolve_service(""), None);
    assert_eq!(document.resolve_service("#other"), None);

    // VALID: add two services.
    let url2: IotaDIDUrl = document.id().to_url().join("#revocation").unwrap();
    let service2: IotaService = Service::from_json(&format!(
      r#"{{
      "id":"{}",
      "type": "RevocationBitmap2022",
      "serviceEndpoint": "data:,blah"
    }}"#,
      url2
    ))
    .unwrap();
    assert!(document.insert_service(service2.clone()).is_ok());
    assert_eq!(2, document.service().len());
    assert_eq!(document.resolve_service(&url2), Some(&service2));
    assert_eq!(document.resolve_service("#revocation"), Some(&service2));
    assert_eq!(document.resolve_service("revocation"), Some(&service2));
    assert_eq!(document.resolve_service(""), None);
    assert_eq!(document.resolve_service("#other"), None);

    // INVALID: insert service with duplicate fragment fails.
    let duplicate: IotaService = Service::from_json(&format!(
      r#"{{
      "id":"{}",
      "type": "DuplicateService",
      "serviceEndpoint": "data:,duplicate"
    }}"#,
      url1
    ))
    .unwrap();
    assert!(document.insert_service(duplicate.clone()).is_err());
    assert_eq!(2, document.service().len());
    let resolved: &IotaService = document.resolve_service(&url1).unwrap();
    assert_eq!(resolved, &service1);
    assert_ne!(resolved, &duplicate);

    // VALID: remove services.
    assert_eq!(service1, document.remove_service(&url1).unwrap());
    assert_eq!(1, document.service().len());
    let last_service: &IotaService = document.resolve_service(&url2).unwrap();
    assert_eq!(last_service, &service2);

    assert_eq!(service2, document.remove_service(&url2).unwrap());
    assert_eq!(0, document.service().len());
  }

  #[test]
  fn test_document_equality() {
    let mut original_doc: IotaDocument = IotaDocument::new_with_id(valid_did());
    let keypair1: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let method1: IotaVerificationMethod = IotaVerificationMethod::new(
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
    let method2: IotaVerificationMethod =
      IotaVerificationMethod::new(doc1.id().to_owned(), keypair2.type_(), keypair2.public(), "test-0").unwrap();

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
    let method3: IotaVerificationMethod =
      IotaVerificationMethod::new(doc1.id().to_owned(), keypair3.type_(), keypair3.public(), "test-0").unwrap();

    let insertion_result = doc2.insert_method(method3, MethodScope::capability_invocation());

    // Nothing was inserted, because a method with the same fragment already existed.
    assert!(insertion_result.is_err());
    assert_eq!(doc1, doc2);
  }

  #[test]
  fn test_unpack_empty() {
    let mock_token_supply: u64 = ProtocolParameters::default().token_supply();
    let controller_did: IotaDID = valid_did();

    // VALID: unpack empty, deactivated document.
    let did: IotaDID = "did:iota:0xBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"
      .parse()
      .unwrap();
    let alias_output: AliasOutput = AliasOutputBuilder::new_with_amount(1, AliasId::from(&did))
      .unwrap()
      .add_unlock_condition(UnlockCondition::StateControllerAddress(
        StateControllerAddressUnlockCondition::new(Address::Alias(AliasAddress::new(AliasId::from(&controller_did)))),
      ))
      .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
        Address::Alias(AliasAddress::new(AliasId::from(&controller_did))),
      )))
      .finish(mock_token_supply)
      .unwrap();
    let document: IotaDocument = IotaDocument::unpack_from_output(&did, &alias_output, true).unwrap();
    assert_eq!(document.id(), &did);
    assert_eq!(document.metadata.deactivated, Some(true));

    // Ensure no other fields are injected.
    let json: String = format!(
      r#"{{"doc":{{"id":"{did}","controller":"{controller_did}"}},"meta":{{"deactivated":true,"governorAddress":"iota1pz424242424242424242424242424242424242424242424242425ryaqzy","stateControllerAddress":"iota1pz424242424242424242424242424242424242424242424242425ryaqzy"}}}}"#
    );
    assert_eq!(document.to_json().unwrap(), json);

    // INVALID: reject empty document.
    assert!(IotaDocument::unpack_from_output(&did, &alias_output, false).is_err());

    // Ensure re-packing removes the controller, state controller address, and governor address.
    let packed: Vec<u8> = document.pack_with_encoding(StateMetadataEncoding::Json).unwrap();
    let state_metadata_document: StateMetadataDocument = StateMetadataDocument::unpack(&packed).unwrap();
    let unpacked_document: IotaDocument = state_metadata_document.into_iota_document(&did).unwrap();
    assert!(unpacked_document.document.controller().is_none());
    assert!(unpacked_document.metadata.state_controller_address.is_none());
    assert!(unpacked_document.metadata.governor_address.is_none());
  }

  #[test]
  fn test_json_roundtrip() {
    let document: IotaDocument = generate_document(&valid_did());

    let ser: String = document.to_json().unwrap();
    let de: IotaDocument = IotaDocument::from_json(&ser).unwrap();
    assert_eq!(document, de);
  }

  #[test]
  fn test_json_fieldnames() {
    // Changing the serialization is a breaking change!
    let document: IotaDocument = IotaDocument::new_with_id(valid_did());
    let serialization: String = document.to_json().unwrap();
    assert_eq!(
      serialization,
      format!("{{\"doc\":{},\"meta\":{}}}", document.document, document.metadata)
    );
  }
}
