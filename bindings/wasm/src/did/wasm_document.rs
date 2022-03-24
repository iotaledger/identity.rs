// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity::core::OneOrMany;
use identity::core::OneOrSet;
use identity::core::OrderedSet;
use identity::core::Url;
use identity::crypto::PrivateKey;
use identity::crypto::SignatureOptions;
use identity::did::verifiable::VerifiableProperties;
use identity::did::MethodScope;
use identity::iota_core::IotaDID;
use identity::iota_core::IotaDocument;
use identity::iota_core::IotaVerificationMethod;
use identity::iota_core::MessageId;
use identity::iota_core::NetworkName;
use wasm_bindgen::prelude::*;

use crate::common::WasmTimestamp;
use crate::credential::WasmCredential;
use crate::credential::WasmPresentation;
use crate::crypto::WasmKeyPair;
use crate::crypto::WasmSignatureOptions;
use crate::did::wasm_method_relationship::WasmMethodRelationship;
use crate::did::OptionMethodScope;
use crate::did::WasmDID;
use crate::did::WasmDIDUrl;
use crate::did::WasmDiffMessage;
use crate::did::WasmDocumentMetadata;
use crate::did::WasmMethodScope;
use crate::did::WasmMethodType;
use crate::did::WasmService;
use crate::did::WasmVerificationMethod;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::JsCast;

// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = Document, inspectable)]
#[derive(Clone, Debug)]
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
  pub fn id(&self) -> WasmDID {
    WasmDID(self.0.id().clone())
  }

  /// Sets the controllers of the DID Document.
  ///
  /// Note: Duplicates will be ignored.
  /// Use `null` to remove all controllers.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, controllers: &UOneOrManyDID) -> Result<()> {
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

  /// Returns a list of document controllers.
  #[wasm_bindgen]
  pub fn controller(&self) -> ArrayDID {
    match self.0.controller() {
      Some(controllers) => controllers
        .iter()
        .cloned()
        .map(WasmDID::from)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<ArrayDID>(),
      None => js_sys::Array::new().unchecked_into::<ArrayDID>(),
    }
  }

  /// Sets the `alsoKnownAs` property in the DID document.
  #[wasm_bindgen(js_name = setAlsoKnownAs)]
  pub fn set_also_known_as(&mut self, urls: &UOneOrManyUrl) -> Result<()> {
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

  /// Returns a set of the document's `alsoKnownAs`.
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
  pub fn properties(&mut self) -> Result<MapStringAny> {
    let properties_map = js_sys::Map::new();
    for (key, value) in self.0.properties().iter() {
      properties_map.set(&JsValue::from(key), &JsValue::from_serde(&value).wasm_result()?);
    }
    Ok(properties_map.unchecked_into::<MapStringAny>())
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
  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &WasmService) -> Result<bool> {
    Ok(self.0.insert_service(service.0.clone()))
  }

  /// Remove a {@link Service} identified by the given {@link DIDUrl} from the document.
  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: &WasmDIDUrl) -> Result<()> {
    self.0.remove_service(&did.0).wasm_result()
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
  /// Adds a new Verification Method to the DID Document.
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

  /// Returns a copy of the first `VerificationMethod` with an `id` property
  /// matching the provided `query`.
  ///
  /// Throws an error if the method is not found.
  #[wasm_bindgen(js_name = resolveMethod)]
  pub fn resolve_method(
    &self,
    query: &UDIDUrlQuery,
    scope: OptionMethodScope,
  ) -> Result<Option<WasmVerificationMethod>> {
    let method_query: String = query.into_serde().wasm_result()?;
    let method_scope: Option<MethodScope> = scope.into_serde().wasm_result()?;

    let method: Option<&IotaVerificationMethod> = if let Some(scope) = method_scope {
      self.0.resolve_method(&method_query, Some(scope))
    } else {
      self.0.resolve_method(&method_query, None)
    };
    match method {
      None => Ok(None),
      Some(method) => Ok(Some(WasmVerificationMethod(method.clone()))),
    }
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
  #[wasm_bindgen(js_name = attachMethodRelationship)]
  pub fn attach_method_relationship(
    &mut self,
    did_url: &WasmDIDUrl,
    relationship: WasmMethodRelationship,
  ) -> Result<bool> {
    self
      .0
      .attach_method_relationship(&did_url.0, relationship.into())
      .wasm_result()
  }

  /// Detaches the given relationship from the given method, if the method exists.
  #[wasm_bindgen(js_name = detachMethodRelationship)]
  pub fn detach_method_relationship(
    &mut self,
    did_url: &WasmDIDUrl,
    relationship: WasmMethodRelationship,
  ) -> Result<bool> {
    self
      .0
      .detach_method_relationship(&did_url.0, relationship.into())
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
  /// verification method. See {@link Document.verifyDocument}.
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
        SignatureOptions::default(),
      )
      .wasm_result()
  }

  /// Creates a signature for the given `Credential` with the specified DID Document
  /// Verification Method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = signCredential)]
  pub fn sign_credential(
    &self,
    data: &JsValue,
    privateKey: Vec<u8>,
    methodQuery: &UDIDUrlQuery,
    options: &WasmSignatureOptions,
  ) -> Result<WasmCredential> {
    let json: JsValue = self.sign_data(data, privateKey, methodQuery, options)?;
    let data: WasmCredential = WasmCredential::from_json(&json)?;

    Ok(data)
  }

  /// Creates a signature for the given `Presentation` with the specified DID Document
  /// Verification Method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = signPresentation)]
  pub fn sign_presentation(
    &self,
    data: &JsValue,
    privateKey: Vec<u8>,
    methodQuery: &UDIDUrlQuery,
    options: &WasmSignatureOptions,
  ) -> Result<WasmPresentation> {
    let json: JsValue = self.sign_data(data, privateKey, methodQuery, options)?;
    let data: WasmPresentation = WasmPresentation::from_json(&json)?;

    Ok(data)
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
    options: &WasmSignatureOptions,
  ) -> Result<JsValue> {
    let mut data: VerifiableProperties = data.into_serde().wasm_result()?;
    let private_key: PrivateKey = privateKey.into();
    let method_query: String = methodQuery.into_serde().wasm_result()?;
    let options: SignatureOptions = options.0.clone();

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
          .map_err(identity::iota_core::Error::InvalidMessage)
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
  #[wasm_bindgen(js_name = verifyDiff)]
  pub fn verify_diff(&self, diff: &WasmDiffMessage) -> Result<()> {
    self.0.verify_diff(&diff.0).wasm_result()
  }

  /// Verifies a `DiffMessage` signature and attempts to merge the changes into `self`.
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
  #[wasm_bindgen(js_name = diffIndex)]
  pub fn diff_index(message_id: &str) -> Result<String> {
    let message_id = MessageId::from_str(message_id)
      .map_err(identity::iota_core::Error::InvalidMessage)
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
  pub fn metadata_created(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.metadata.created)
  }

  /// Sets the timestamp of when the DID document was created.
  #[wasm_bindgen(js_name = setMetadataCreated)]
  pub fn set_metadata_created(&mut self, timestamp: &WasmTimestamp) {
    self.0.metadata.created = timestamp.0;
  }

  /// Returns a copy of the timestamp of the last DID document update.
  #[wasm_bindgen(js_name = metadataUpdated)]
  pub fn metadata_updated(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.metadata.updated)
  }

  /// Sets the timestamp of the last DID document update.
  #[wasm_bindgen(js_name = setMetadataUpdated)]
  pub fn set_metadata_updated(&mut self, timestamp: &WasmTimestamp) {
    self.0.metadata.updated = timestamp.0;
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
      .map_err(identity::iota_core::Error::InvalidMessage)
      .wasm_result()?;
    self.0.metadata.previous_message_id = message_id;
    Ok(())
  }

  /// Returns a copy of the `proof` object.
  #[wasm_bindgen]
  pub fn proof(&self) -> Result<JsValue> {
    // TODO: implement proper bindings for the proof.
    match &self.0.proof {
      Some(proof) => JsValue::from_serde(proof).wasm_result(),
      None => Ok(JsValue::NULL),
    }
  }

  // ===========================================================================
  // JSON
  // ===========================================================================

  /// Serializes a `Document` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Document` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmDocument> {
    json.into_serde().map(Self).wasm_result()
  }
}

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
/// Duck-typed union to pass either a string or WasmDIDUrl as a parameter.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "DIDUrl | string")]
  pub type UDIDUrlQuery;

  #[wasm_bindgen(typescript_type = "string | string[] | null")]
  pub type UOneOrManyUrl;

  #[wasm_bindgen(typescript_type = "DID | DID[] | null")]
  pub type UOneOrManyDID;

  #[wasm_bindgen(typescript_type = "DID[]")]
  pub type ArrayDID;

  #[wasm_bindgen(typescript_type = "Service[]")]
  pub type ArrayService;

  #[wasm_bindgen(typescript_type = "Array<string>")]
  pub type ArrayString;

  #[wasm_bindgen(typescript_type = "VerificationMethod[]")]
  pub type ArrayVerificationMethods;

  #[wasm_bindgen(typescript_type = "Map<string, any>")]
  pub type MapStringAny;
}
