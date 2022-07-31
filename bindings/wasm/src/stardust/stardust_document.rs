// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::OneOrMany;
use identity_iota::core::OrderedSet;
use identity_iota::core::Timestamp;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::Presentation;
use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::verifiable::VerifiableProperties;
use identity_iota::did::Document;
use identity_stardust::NetworkName;
use identity_stardust::StardustDID;
use identity_stardust::StardustDocument;
use identity_stardust::StateMetadataEncoding;
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
use crate::crypto::WasmProofOptions;
use crate::did::WasmMethodRelationship;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use crate::stardust::WasmStardustDID;
use crate::stardust::WasmStardustDIDUrl;
use crate::stardust::WasmStardustDocumentMetadata;
use crate::stardust::WasmStardustService;
use crate::stardust::WasmStateMetadataEncoding;

// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = StardustDocument, inspectable)]
pub struct WasmStardustDocument(pub(crate) StardustDocument);

#[wasm_bindgen(js_class = StardustDocument)]
impl WasmStardustDocument {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs an empty DID Document with a {@link StardustDID.placeholder} identifier
  /// for the given `network`.
  #[wasm_bindgen(constructor)]
  pub fn new(network: String) -> Result<WasmStardustDocument> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    Ok(WasmStardustDocument::from(StardustDocument::new(&network_name)))
  }

  /// Constructs an empty DID Document with the given identifier.
  #[wasm_bindgen(js_name = newWithId)]
  pub fn new_with_id(id: &WasmStardustDID) -> WasmStardustDocument {
    let did: StardustDID = id.0.clone();
    WasmStardustDocument::from(StardustDocument::new_with_id(did))
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns a copy of the DID Document `id`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmStardustDID {
    WasmStardustDID::from(self.0.id().clone())
  }

  /// Returns a copy of the list of document controllers.
  ///
  /// NOTE: controllers are determined by the `state_controller` unlock condition of the output
  /// during resolution and are omitted when publishing.
  #[wasm_bindgen]
  pub fn controller(&self) -> ArrayStardustDID {
    match self.0.controller() {
      Some(controllers) => controllers
        .iter()
        .cloned()
        .map(WasmStardustDID::from)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<ArrayStardustDID>(),
      None => js_sys::Array::new().unchecked_into::<ArrayStardustDID>(),
    }
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

  /// Returns a copy of the custom DID Document properties.
  #[wasm_bindgen]
  pub fn properties(&mut self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
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

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all {@link StardustService StardustServices} in the document.
  #[wasm_bindgen]
  pub fn service(&self) -> ArrayStardustService {
    self
      .0
      .service()
      .iter()
      .cloned()
      .map(WasmStardustService)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayStardustService>()
  }

  /// Add a new {@link StardustService} to the document.
  ///
  /// Returns `true` if the service was added.
  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &WasmStardustService) -> bool {
    self.0.insert_service(service.0.clone())
  }

  /// Remove a {@link StardustService} identified by the given {@link DIDUrl} from the document.
  ///
  /// Returns `true` if a service was removed.
  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: &WasmStardustDIDUrl) -> bool {
    self.0.remove_service(&did.0)
  }

  /// Returns the first {@link StardustService} with an `id` property matching the provided `query`,
  /// if present.
  #[wasm_bindgen(js_name = resolveService)]
  pub fn resolve_service(&self, query: &UStardustDIDUrlQuery) -> Option<WasmStardustService> {
    let service_query: String = query.into_serde().ok()?;
    self
      .0
      .resolve_service(&service_query)
      .cloned()
      .map(WasmStardustService::from)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  // /// Returns a list of all {@link VerificationMethod} in the DID Document.
  // #[wasm_bindgen]
  // pub fn methods(&self) -> ArrayVerificationMethods {
  //   self
  //     .0
  //     .methods()
  //     .cloned()
  //     .map(WasmVerificationMethod::from)
  //     .map(JsValue::from)
  //     .collect::<js_sys::Array>()
  //     .unchecked_into::<ArrayVerificationMethods>()
  // }
  //
  // /// Adds a new Verification Method to the DID Document.
  // #[wasm_bindgen(js_name = insertMethod)]
  // pub fn insert_method(&mut self, method: &WasmVerificationMethod, scope: &WasmMethodScope) -> Result<()> {
  //   self.0.insert_method(method.0.clone(), scope.0).wasm_result()?;
  //   Ok(())
  // }
  //

  /// Removes all references to the specified Verification Method.
  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &WasmStardustDIDUrl) -> Result<()> {
    self.0.remove_method(&did.0).wasm_result()
  }

  // /// Returns a copy of the first `VerificationMethod` with an `id` property
  // /// matching the provided `query`.
  // ///
  // /// Throws an error if the method is not found.
  // #[wasm_bindgen(js_name = resolveMethod)]
  // pub fn resolve_method(
  //   &self,
  //   query: &UDIDUrlQuery,
  //   scope: Option<RefMethodScope>,
  // ) -> Result<Option<WasmVerificationMethod>> {
  //   let method_query: String = query.into_serde().wasm_result()?;
  //   let method_scope: Option<MethodScope> = scope.map(|js| js.into_serde().wasm_result()).transpose()?;
  //
  //   let method: Option<&IotaVerificationMethod> = if let Some(scope) = method_scope {
  //     self.0.resolve_method(&method_query, Some(scope))
  //   } else {
  //     self.0.resolve_method(&method_query, None)
  //   };
  //   match method {
  //     None => Ok(None),
  //     Some(method) => Ok(Some(WasmVerificationMethod(method.clone()))),
  //   }
  // }

  /// Attaches the relationship to the given method, if the method exists.
  ///
  /// Note: The method needs to be in the set of verification methods,
  /// so it cannot be an embedded one.
  #[wasm_bindgen(js_name = attachMethodRelationship)]
  pub fn attach_method_relationship(
    &mut self,
    did_url: &WasmStardustDIDUrl,
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
    did_url: &WasmStardustDIDUrl,
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

  /// Creates a signature for the given `Credential` with the specified DID Document
  /// Verification Method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = signCredential)]
  pub fn sign_credential(
    &self,
    credential: &WasmCredential,
    privateKey: Vec<u8>,
    methodQuery: &UStardustDIDUrlQuery,
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
    methodQuery: &UStardustDIDUrlQuery,
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
    methodQuery: &UStardustDIDUrlQuery,
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

  // ===========================================================================
  // Publishing
  // ===========================================================================

  /// Serializes the document for inclusion in an Alias Output's state metadata
  /// with the default {@link StateMetadataEncoding}.
  #[wasm_bindgen]
  pub fn pack(&self) -> Result<Vec<u8>> {
    self.0.clone().pack().wasm_result()
  }

  /// Serializes the document for inclusion in an Alias Output's state metadata.
  #[wasm_bindgen(js_name = packWithEncoding)]
  pub fn pack_with_encoding(&self, encoding: WasmStateMetadataEncoding) -> Result<Vec<u8>> {
    self
      .0
      .clone()
      .pack_with_encoding(StateMetadataEncoding::from(encoding))
      .wasm_result()
  }

  /// Deserializes the document from the state metadata bytes of an Alias Output.
  ///
  /// NOTE: `did` is required since it is omitted from the serialized DID Document and
  /// cannot be inferred from the state metadata. It also indicates the network, which is not
  /// encoded in the `AliasId` alone.
  #[allow(non_snake_case)]
  #[wasm_bindgen]
  pub fn unpack(did: &WasmStardustDID, stateMetadata: &[u8]) -> Result<WasmStardustDocument> {
    StardustDocument::unpack(&did.0, stateMetadata)
      .map(WasmStardustDocument)
      .wasm_result()
  }

  // TODO: unpack_from_output/unpackFromOutput ? Feature-gated method, do we need an equivalent?

  // ===========================================================================
  // Metadata
  // ===========================================================================

  /// Returns a copy of the metadata associated with this document.
  ///
  /// NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
  /// `metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.
  #[wasm_bindgen]
  pub fn metadata(&self) -> WasmStardustDocumentMetadata {
    WasmStardustDocumentMetadata::from(self.0.metadata.clone())
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

  // ===========================================================================
  // Revocation
  // ===========================================================================

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// revoke all specified `indices`.
  #[wasm_bindgen(js_name = revokeCredentials)]
  #[allow(non_snake_case)]
  pub fn revoke_credentials(&mut self, serviceQuery: &UStardustDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.revoke_credentials(&query, indices.as_slice()).wasm_result()
  }

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// unrevoke all specified `indices`.
  #[wasm_bindgen(js_name = unrevokeCredentials)]
  #[allow(non_snake_case)]
  pub fn unrevoke_credentials(&mut self, serviceQuery: &UStardustDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.unrevoke_credentials(&query, indices.as_slice()).wasm_result()
  }
}

impl_wasm_json!(WasmStardustDocument, StardustDocument);
impl_wasm_clone!(WasmStardustDocument, StardustDocument);

impl From<StardustDocument> for WasmStardustDocument {
  fn from(document: StardustDocument) -> Self {
    Self(document)
  }
}

impl From<WasmStardustDocument> for StardustDocument {
  fn from(wasm_document: WasmStardustDocument) -> Self {
    wasm_document.0
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "StardustDIDUrl | string")]
  pub type UStardustDIDUrlQuery;

  // #[wasm_bindgen(typescript_type = "StardustDID | StardustDID[] | null")]
  // pub type OptionOneOrManyStardustDID;

  #[wasm_bindgen(typescript_type = "StardustDID[]")]
  pub type ArrayStardustDID;

  #[wasm_bindgen(typescript_type = "StardustService[]")]
  pub type ArrayStardustService;
  //
  // #[wasm_bindgen(typescript_type = "VerificationMethod[]")]
  // pub type ArrayVerificationMethods;
}
