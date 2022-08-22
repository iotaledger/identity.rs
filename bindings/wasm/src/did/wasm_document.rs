// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_iota::core::OneOrMany;
use identity_iota::core::OneOrSet;
use identity_iota::core::OrderedSet;
use identity_iota::core::Timestamp;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::Presentation;
use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::verifiable::VerifiableProperties;
use identity_iota::did::Document;
use identity_iota::did::MethodScope;
use identity_iota::iota_core::IotaDID;
use identity_iota::iota_core::IotaDocument;
use identity_iota::iota_core::IotaVerificationMethod;
use identity_iota::iota_core::MessageId;
use identity_iota::iota_core::NetworkName;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::common::OptionOneOrManyString;
use crate::common::OptionTimestamp;
use crate::common::UOneOrManyNumber;
use crate::common::WasmTimestamp;
use crate::credential::WasmCredential;
use crate::credential::WasmPresentation;
use crate::crypto::WasmKeyPair;
use crate::crypto::WasmProof;
use crate::crypto::WasmProofOptions;
use crate::did::RefMethodScope;
use crate::did::WasmDIDUrl;
use crate::did::WasmDiffMessage;
use crate::did::WasmDocumentMetadata;
use crate::did::WasmIotaDID;
use crate::did::WasmMethodRelationship;
use crate::did::WasmMethodScope;
use crate::did::WasmMethodType;
use crate::did::WasmService;
use crate::did::WasmVerificationMethod;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;

// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = Document, inspectable)]
pub struct WasmDocument(pub(crate) IotaDocument);

#[wasm_bindgen(js_class = Document)]
impl WasmDocument {
  /// Creates a new DID Document from the given `KeyPair`, network, and verification method
  /// fragment name.
  ///
  /// The DID Document will be pre-populated with a single verification method
  /// derived from the provided `KeyPair` embedded as a capability invocation
  /// verification relationship. This method will have the DID URL fragment
  /// `#sign-0` by default and can be easily retrieved with `Document::defaultSigningMethod`.
  ///
  /// NOTE: the generated document is unsigned, see `Document::signSelf`.
  ///
  /// Arguments:
  ///
  /// * keypair: the initial verification method is derived from the public key with this keypair.
  /// * network: Tangle network to use for the DID, default `Network::mainnet`.
  /// * fragment: name of the initial verification method, default "sign-0".
  #[wasm_bindgen(constructor)]
  pub fn new(keypair: &WasmKeyPair, network: Option<String>, fragment: Option<String>) -> Result<WasmDocument> {
    let network_name = network.map(NetworkName::try_from).transpose().wasm_result()?;
    IotaDocument::new_with_options(&keypair.0, network_name, fragment.as_deref())
      .map(Self)
      .wasm_result()
  }

  /// Creates a new DID Document from the given `VerificationMethod`.
  ///
  /// NOTE: the generated document is unsigned, see `Document::signSelf`.
  #[wasm_bindgen(js_name = fromVerificationMethod)]
  pub fn from_verification_method(method: &WasmVerificationMethod) -> Result<WasmDocument> {
    IotaDocument::from_verification_method(method.0.clone())
      .map(Self)
      .wasm_result()
  }

  /// Returns whether the given {@link MethodType} can be used to sign document updates.
  #[wasm_bindgen(js_name = isSigningMethodType)]
  pub fn is_signing_method_type(method_type: &WasmMethodType) -> bool {
    IotaDocument::is_signing_method_type(method_type.0)
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns a copy of the DID Document `id`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmIotaDID {
    WasmIotaDID(self.0.id().clone())
  }

  /// Sets the controllers of the DID Document.
  ///
  /// Note: Duplicates will be ignored.
  /// Use `null` to remove all controllers.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, controllers: &OptionOneOrManyIotaDID) -> Result<()> {
    let controllers: Option<OneOrMany<IotaDID>> = controllers.into_serde().wasm_result()?;
    let controller_set: Option<OneOrSet<IotaDID>> = if let Some(controllers) = controllers.map(OneOrMany::into_vec) {
      if controllers.is_empty() {
        None
      } else {
        Some(OneOrSet::try_from(OrderedSet::from_iter(controllers)).wasm_result()?)
      }
    } else {
      None
    };
    *self.0.controller_mut() = controller_set;
    Ok(())
  }

  /// Returns a copy of the list of document controllers.
  #[wasm_bindgen]
  pub fn controller(&self) -> ArrayIotaDID {
    match self.0.controller() {
      Some(controllers) => controllers
        .iter()
        .cloned()
        .map(WasmIotaDID::from)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<ArrayIotaDID>(),
      None => js_sys::Array::new().unchecked_into::<ArrayIotaDID>(),
    }
  }

  /// Sets the `alsoKnownAs` property in the DID document.
  #[wasm_bindgen(js_name = setAlsoKnownAs)]
  pub fn set_also_known_as(&mut self, urls: &OptionOneOrManyString) -> Result<()> {
    let urls: Option<OneOrMany<String>> = urls.into_serde().wasm_result()?;
    let mut urls_set: OrderedSet<Url> = OrderedSet::new();
    if let Some(urls) = urls {
      for url in urls.into_vec() {
        urls_set.append(Url::parse(url).wasm_result()?);
      }
    }
    *self.0.also_known_as_mut() = urls_set;
    Ok(())
  }

  /// Returns a copy of the document's `alsoKnownAs` set.
  #[wasm_bindgen(js_name = alsoKnownAs)]
  pub fn also_known_as(&self) -> ArrayString {
    self
      .0
      .also_known_as()
      .iter()
      .map(|url| url.to_string())
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  /// Adds a custom property to the DID Document.
  /// If the value is set to `null`, the custom property will be removed.
  ///
  /// ### WARNING
  /// This method can overwrite existing properties like `id` and result in an invalid document.
  #[wasm_bindgen(js_name = setPropertyUnchecked)]
  pub fn set_property_unchecked(&mut self, key: String, value: &JsValue) -> Result<()> {
    let value: Option<serde_json::Value> = value.into_serde().wasm_result()?;
    match value {
      Some(value) => {
        self.0.properties_mut().insert(key, value);
      }
      None => {
        self.0.properties_mut().remove(&key);
      }
    }
    Ok(())
  }

  /// Returns a copy of the custom DID Document properties.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all {@link Service Services} in the document.
  #[wasm_bindgen]
  pub fn service(&self) -> ArrayService {
    self
      .0
      .service()
      .iter()
      .cloned()
      .map(WasmService)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayService>()
  }

  /// Add a new {@link Service} to the document.
  ///
  /// Returns `true` if the service was added.
  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &WasmService) -> bool {
    self.0.insert_service(service.0.clone())
  }

  /// Remove a {@link Service} identified by the given {@link DIDUrl} from the document.
  ///
  /// Returns `true` if a service was removed.
  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: &WasmDIDUrl) -> bool {
    self.0.remove_service(&did.0)
  }

  /// Returns the first {@link Service} with an `id` property matching the provided `query`,
  /// if present.
  #[wasm_bindgen(js_name = resolveService)]
  pub fn resolve_service(&self, query: &UDIDUrlQuery) -> Option<WasmService> {
    let service_query: String = query.into_serde().ok()?;
    self.0.resolve_service(&service_query).cloned().map(WasmService::from)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns a list of all {@link VerificationMethod} in the DID Document.
  #[wasm_bindgen]
  pub fn methods(&self) -> ArrayVerificationMethods {
    self
      .0
      .methods()
      .cloned()
      .map(WasmVerificationMethod::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayVerificationMethods>()
  }
  /// Adds a new `method` to the document in the given `scope`.
  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &WasmVerificationMethod, scope: &WasmMethodScope) -> Result<()> {
    self.0.insert_method(method.0.clone(), scope.0).wasm_result()?;
    Ok(())
  }

  /// Removes all references to the specified Verification Method.
  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &WasmDIDUrl) -> Result<()> {
    self.0.remove_method(&did.0).wasm_result()
  }

  /// Returns a copy of the first `VerificationMethod` with a capability invocation relationship
  /// capable of signing this DID document.
  ///
  /// Throws an error if no signing method is present.
  #[wasm_bindgen(js_name = defaultSigningMethod)]
  pub fn default_signing_method(&self) -> Result<WasmVerificationMethod> {
    self
      .0
      .default_signing_method()
      .map(Clone::clone)
      .map(WasmVerificationMethod::from)
      .wasm_result()
  }

  /// Returns a copy of the first verification method with an `id` property
  /// matching the provided `query` and the verification relationship
  /// specified by `scope`, if present.
  #[wasm_bindgen(js_name = resolveMethod)]
  pub fn resolve_method(
    &self,
    query: &UDIDUrlQuery,
    scope: Option<RefMethodScope>,
  ) -> Result<Option<WasmVerificationMethod>> {
    let method_query: String = query.into_serde().wasm_result()?;
    let method_scope: Option<MethodScope> = scope.map(|js| js.into_serde().wasm_result()).transpose()?;

    let method: Option<&IotaVerificationMethod> = self.0.resolve_method(&method_query, method_scope);
    Ok(method.cloned().map(WasmVerificationMethod))
  }

  /// Attempts to resolve the given method query into a method capable of signing a document update.
  #[wasm_bindgen(js_name = resolveSigningMethod)]
  pub fn resolve_signing_method(&mut self, query: &UDIDUrlQuery) -> Result<WasmVerificationMethod> {
    let method_query: String = query.into_serde().wasm_result()?;
    Ok(WasmVerificationMethod(
      self.0.resolve_signing_method(&method_query).wasm_result()?.clone(),
    ))
  }

  /// Attaches the relationship to the given method, if the method exists.
  ///
  /// Note: The method needs to be in the set of verification methods,
  /// so it cannot be an embedded one.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = attachMethodRelationship)]
  pub fn attach_method_relationship(
    &mut self,
    didUrl: &WasmDIDUrl,
    relationship: WasmMethodRelationship,
  ) -> Result<bool> {
    self
      .0
      .attach_method_relationship(&didUrl.0, relationship.into())
      .wasm_result()
  }

  /// Detaches the given relationship from the given method, if the method exists.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = detachMethodRelationship)]
  pub fn detach_method_relationship(
    &mut self,
    didUrl: &WasmDIDUrl,
    relationship: WasmMethodRelationship,
  ) -> Result<bool> {
    self
      .0
      .detach_method_relationship(&didUrl.0, relationship.into())
      .wasm_result()
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

  /// Signs the DID document with the verification method specified by `method_query`.
  /// The `method_query` may be the full `DIDUrl` of the method or just its fragment,
  /// e.g. "#sign-0".
  ///
  /// NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
  /// verification method. See `Document::verifySelfSigned`.
  #[wasm_bindgen(js_name = signSelf)]
  pub fn sign_self(&mut self, key_pair: &WasmKeyPair, method_query: &UDIDUrlQuery) -> Result<()> {
    let method_query: String = method_query.into_serde().wasm_result()?;
    self.0.sign_self(key_pair.0.private(), &method_query).wasm_result()
  }

  /// Signs another DID document using the verification method specified by `method_query`.
  /// The `method_query` may be the full `DIDUrl` of the method or just its fragment,
  /// e.g. "#sign-0".
  ///
  /// `Document.signSelf` should be used in general, this throws an error if trying to operate
  /// on the same document. This is intended for signing updates to a document where a sole
  /// capability invocation method is rotated or replaced entirely.
  ///
  /// NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
  /// verification method. See [Document.verifyDocument](#Document+verifyDocument).
  #[wasm_bindgen(js_name = signDocument)]
  pub fn sign_document(
    &self,
    document: &mut WasmDocument,
    key_pair: &WasmKeyPair,
    method_query: &UDIDUrlQuery,
  ) -> Result<()> {
    let method_query: String = method_query.into_serde().wasm_result()?;
    self
      .0
      .sign_data(
        &mut document.0,
        key_pair.0.private(),
        &method_query,
        ProofOptions::default(),
      )
      .wasm_result()
  }

  /// Creates a signature for the given `Credential` with the specified DID Document
  /// Verification Method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = signCredential)]
  pub fn sign_credential(
    &self,
    credential: &WasmCredential,
    privateKey: Vec<u8>,
    methodQuery: &UDIDUrlQuery,
    options: &WasmProofOptions,
  ) -> Result<WasmCredential> {
    let mut data: Credential = credential.0.clone();
    let private_key: PrivateKey = privateKey.into();
    let method_query: String = methodQuery.into_serde().wasm_result()?;
    let options: ProofOptions = options.0.clone();

    self
      .0
      .sign_data(&mut data, &private_key, &method_query, options)
      .wasm_result()?;
    Ok(WasmCredential::from(data))
  }

  /// Creates a signature for the given `Presentation` with the specified DID Document
  /// Verification Method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = signPresentation)]
  pub fn sign_presentation(
    &self,
    presentation: &WasmPresentation,
    privateKey: Vec<u8>,
    methodQuery: &UDIDUrlQuery,
    options: &WasmProofOptions,
  ) -> Result<WasmPresentation> {
    let mut data: Presentation = presentation.0.clone();
    let private_key: PrivateKey = privateKey.into();
    let method_query: String = methodQuery.into_serde().wasm_result()?;
    let options: ProofOptions = options.0.clone();

    self
      .0
      .sign_data(&mut data, &private_key, &method_query, options)
      .wasm_result()?;
    Ok(WasmPresentation::from(data))
  }

  /// Creates a signature for the given `data` with the specified DID Document
  /// Verification Method.
  ///
  /// NOTE: use `signSelf` or `signDocument` for DID Documents.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = signData)]
  pub fn sign_data(
    &self,
    data: &JsValue,
    privateKey: Vec<u8>,
    methodQuery: &UDIDUrlQuery,
    options: &WasmProofOptions,
  ) -> Result<JsValue> {
    let mut data: VerifiableProperties = data.into_serde().wasm_result()?;
    let private_key: PrivateKey = privateKey.into();
    let method_query: String = methodQuery.into_serde().wasm_result()?;
    let options: ProofOptions = options.0.clone();

    self
      .0
      .sign_data(&mut data, &private_key, &method_query, options)
      .wasm_result()?;

    JsValue::from_serde(&data).wasm_result()
  }

  // ===========================================================================
  // Verification
  // ===========================================================================

  /// Verifies the authenticity of `data` using the target verification method.
  #[wasm_bindgen(js_name = verifyData)]
  pub fn verify_data(&self, data: &JsValue, options: &WasmVerifierOptions) -> Result<bool> {
    let data: VerifiableProperties = data.into_serde().wasm_result()?;
    Ok(self.0.verify_data(&data, &options.0).is_ok())
  }

  /// Verifies that the signature on the DID document `signed` was generated by a valid method from
  /// this DID document.
  ///
  /// # Errors
  ///
  /// Fails if:
  /// - The signature proof section is missing in the `signed` document.
  /// - The method is not found in this document.
  /// - An unsupported verification method is used.
  /// - The signature verification operation fails.
  #[wasm_bindgen(js_name = verifyDocument)]
  pub fn verify_document(&self, signed: &WasmDocument) -> Result<()> {
    self.0.verify_document(&signed.0).wasm_result()
  }

  /// Verifies whether `document` is a valid root DID document according to the IOTA DID method
  /// specification.
  ///
  /// It must be signed using a verification method with a public key whose BLAKE2b-256 hash matches
  /// the DID tag.
  #[wasm_bindgen(js_name = verifyRootDocument)]
  pub fn verify_root_document(document: &WasmDocument) -> Result<()> {
    IotaDocument::verify_root_document(&document.0).wasm_result()
  }

  // ===========================================================================
  // Diffs
  // ===========================================================================

  /// Generate a `DiffMessage` between two DID Documents and sign it using the specified
  /// `key` and `method`.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen]
  pub fn diff(
    &self,
    other: &WasmDocument,
    message_id: &str,
    key: &WasmKeyPair,
    method_query: &UDIDUrlQuery,
  ) -> Result<WasmDiffMessage> {
    let method_query: String = method_query.into_serde().wasm_result()?;
    self
      .0
      .diff(
        &other.0,
        MessageId::from_str(message_id)
          .map_err(identity_iota::iota_core::Error::InvalidMessage)
          .wasm_result()?,
        key.0.private(),
        &method_query,
      )
      .map(WasmDiffMessage::from)
      .wasm_result()
  }

  /// Verifies the signature of the `diff` was created using a capability invocation method
  /// in this DID Document.
  ///
  /// # Errors
  ///
  /// Fails if an unsupported verification method is used or the verification operation fails.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = verifyDiff)]
  pub fn verify_diff(&self, diff: &WasmDiffMessage) -> Result<()> {
    self.0.verify_diff(&diff.0).wasm_result()
  }

  /// Verifies a `DiffMessage` signature and attempts to merge the changes into `self`.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = mergeDiff)]
  pub fn merge_diff(&mut self, diff: &WasmDiffMessage) -> Result<()> {
    self.0.merge_diff(&diff.0).wasm_result()
  }

  // ===========================================================================
  // Publishing
  // ===========================================================================

  /// Returns the Tangle index of the integration chain for this DID.
  ///
  /// This is simply the tag segment of the `DID`.
  /// E.g.
  /// For a document with DID: did:iota:1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI,
  /// `doc.integration_index()` == "1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI"
  #[wasm_bindgen(js_name = integrationIndex)]
  pub fn integration_index(&self) -> String {
    self.0.integration_index().to_owned()
  }

  /// Returns the Tangle index of the DID diff chain. This should only be called on documents
  /// published on the integration chain.
  ///
  /// This is the Base58-btc encoded SHA-256 digest of the hex-encoded message id.
  ///
  /// @deprecated since 0.5.0, diff chain features are slated for removal.
  #[wasm_bindgen(js_name = diffIndex)]
  pub fn diff_index(message_id: &str) -> Result<String> {
    let message_id = MessageId::from_str(message_id)
      .map_err(identity_iota::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    IotaDocument::diff_index(&message_id).wasm_result()
  }

  // ===========================================================================
  // Metadata
  // ===========================================================================

  /// Returns a copy of the metadata associated with this document.
  ///
  /// NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
  /// `metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.
  #[wasm_bindgen]
  pub fn metadata(&self) -> WasmDocumentMetadata {
    WasmDocumentMetadata::from(self.0.metadata.clone())
  }

  /// Returns a copy of the timestamp of when the DID document was created.
  #[wasm_bindgen(js_name = metadataCreated)]
  pub fn metadata_created(&self) -> Option<WasmTimestamp> {
    self.0.metadata.created.map(WasmTimestamp::from)
  }

  /// Sets the timestamp of when the DID document was created.
  #[wasm_bindgen(js_name = setMetadataCreated)]
  pub fn set_metadata_created(&mut self, timestamp: OptionTimestamp) -> Result<()> {
    let timestamp: Option<Timestamp> = timestamp.into_serde().wasm_result()?;
    self.0.metadata.created = timestamp;
    Ok(())
  }

  /// Returns a copy of the timestamp of the last DID document update.
  #[wasm_bindgen(js_name = metadataUpdated)]
  pub fn metadata_updated(&self) -> Option<WasmTimestamp> {
    self.0.metadata.updated.map(WasmTimestamp::from)
  }

  /// Sets the timestamp of the last DID document update.
  #[wasm_bindgen(js_name = setMetadataUpdated)]
  pub fn set_metadata_updated(&mut self, timestamp: OptionTimestamp) -> Result<()> {
    let timestamp: Option<Timestamp> = timestamp.into_serde().wasm_result()?;
    self.0.metadata.updated = timestamp;
    Ok(())
  }

  /// Returns a copy of the previous integration chain message id.
  #[wasm_bindgen(js_name = metadataPreviousMessageId)]
  pub fn metadata_previous_message_id(&self) -> String {
    self.0.metadata.previous_message_id.to_string()
  }

  /// Sets the previous integration chain message id.
  #[wasm_bindgen(js_name = setMetadataPreviousMessageId)]
  pub fn set_metadata_previous_message_id(&mut self, value: &str) -> Result<()> {
    let message_id: MessageId = MessageId::from_str(value)
      .map_err(identity_iota::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.metadata.previous_message_id = message_id;
    Ok(())
  }

  /// Sets a custom property in the document metadata.
  /// If the value is set to `null`, the custom property will be removed.
  #[wasm_bindgen(js_name = setMetadataPropertyUnchecked)]
  pub fn set_metadata_property_unchecked(&mut self, key: String, value: &JsValue) -> Result<()> {
    let value: Option<serde_json::Value> = value.into_serde().wasm_result()?;
    match value {
      Some(value) => {
        self.0.metadata.properties.insert(key, value);
      }
      None => {
        self.0.metadata.properties.remove(&key);
      }
    }
    Ok(())
  }

  /// Returns a copy of the proof.
  #[wasm_bindgen]
  pub fn proof(&self) -> Option<WasmProof> {
    self.0.proof.clone().map(WasmProof::from)
  }

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// revoke all specified `indices`.
  #[wasm_bindgen(js_name = revokeCredentials)]
  #[allow(non_snake_case)]
  pub fn revoke_credentials(&mut self, serviceQuery: &UDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.revoke_credentials(&query, indices.as_slice()).wasm_result()
  }

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// unrevoke all specified `indices`.
  #[wasm_bindgen(js_name = unrevokeCredentials)]
  #[allow(non_snake_case)]
  pub fn unrevoke_credentials(&mut self, serviceQuery: &UDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.unrevoke_credentials(&query, indices.as_slice()).wasm_result()
  }
}

impl_wasm_json!(WasmDocument, Document);
impl_wasm_clone!(WasmDocument, Document);

impl From<IotaDocument> for WasmDocument {
  fn from(document: IotaDocument) -> Self {
    Self(document)
  }
}

impl From<WasmDocument> for IotaDocument {
  fn from(wasm_document: WasmDocument) -> Self {
    wasm_document.0
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DIDUrl | string")]
  pub type UDIDUrlQuery;

  #[wasm_bindgen(typescript_type = "IotaDID | IotaDID[] | null")]
  pub type OptionOneOrManyIotaDID;

  #[wasm_bindgen(typescript_type = "IotaDID[]")]
  pub type ArrayIotaDID;

  #[wasm_bindgen(typescript_type = "Service[]")]
  pub type ArrayService;

  #[wasm_bindgen(typescript_type = "VerificationMethod[]")]
  pub type ArrayVerificationMethods;
}
