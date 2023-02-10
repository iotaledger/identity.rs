// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::rc::Rc;

use identity_iota::core::OneOrMany;
use identity_iota::core::OrderedSet;
use identity_iota::core::Timestamp;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::Presentation;
use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::ProofOptions;
use identity_iota::document::verifiable::VerifiableProperties;
use identity_iota::iota::block::output::dto::AliasOutputDto;
use identity_iota::iota::block::output::AliasOutput;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::NetworkName;
use identity_iota::iota::StateMetadataEncoding;
use identity_iota::verification::MethodScope;
use identity_iota::verification::VerificationMethod;
use iota_types::block::protocol::dto::ProtocolParametersDto;
use iota_types::block::protocol::ProtocolParameters;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::common::ArrayService;
use crate::common::ArrayString;
use crate::common::ArrayVerificationMethod;
use crate::common::MapStringAny;
use crate::common::OptionOneOrManyString;
use crate::common::OptionTimestamp;
use crate::common::UDIDUrlQuery;
use crate::common::UOneOrManyNumber;
use crate::common::WasmTimestamp;
use crate::credential::WasmCredential;
use crate::credential::WasmPresentation;
use crate::crypto::WasmProofOptions;
use crate::did::CoreDocumentLock;
use crate::did::RefMethodScope;
use crate::did::WasmDIDUrl;
use crate::did::WasmCoreDocument;
use crate::did::WasmMethodRelationship;
use crate::did::WasmMethodScope;
use crate::did::WasmService;
use crate::did::WasmVerificationMethod;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use crate::iota::identity_client_ext::IAliasOutput;
use crate::iota::WasmIotaDID;
use crate::iota::WasmIotaDocumentMetadata;
use crate::iota::WasmStateMetadataEncoding;

type IotaDocumentLock = tokio::sync::RwLock<IotaDocument>; 
// =============================================================================
// =============================================================================

#[wasm_bindgen(js_name = IotaDocument, inspectable)]
pub struct WasmIotaDocument(pub(crate) Rc<IotaDocumentLock>);

#[wasm_bindgen(js_class = IotaDocument)]
impl WasmIotaDocument {
  // ===========================================================================
  // Constructors
  // ===========================================================================

  /// Constructs an empty DID Document with a {@link IotaDID.placeholder} identifier
  /// for the given `network`.
  #[wasm_bindgen(constructor)]
  pub fn new(network: String) -> Result<WasmIotaDocument> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    Ok(WasmIotaDocument::from(IotaDocument::new(&network_name)))
  }

  /// Constructs an empty DID Document with the given identifier.
  #[wasm_bindgen(js_name = newWithId)]
  pub fn new_with_id(id: &WasmIotaDID) -> WasmIotaDocument {
    let did: IotaDID = id.0.clone();
    WasmIotaDocument::from(IotaDocument::new_with_id(did))
  }

  // ===========================================================================
  // Properties
  // ===========================================================================

  /// Returns a copy of the DID Document `id`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmIotaDID {
    WasmIotaDID::from(self.0.blocking_read().id().clone())
  }

  /// Returns a copy of the list of document controllers.
  ///
  /// NOTE: controllers are determined by the `state_controller` unlock condition of the output
  /// during resolution and are omitted when publishing.
  #[wasm_bindgen]
  pub fn controller(&self) -> ArrayIotaDID {
    self
      .0
      .blocking_read()
      .controller()
      .cloned()
      .map(WasmIotaDID::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayIotaDID>()
  }

  /// Returns a copy of the document's `alsoKnownAs` set.
  #[wasm_bindgen(js_name = alsoKnownAs)]
  pub fn also_known_as(&self) -> ArrayString {
    self
      .0
      .blocking_read()
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
    *self.0.blocking_write().also_known_as_mut() = urls_set;
    Ok(())
  }

  /// Returns a copy of the custom DID Document properties.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.blocking_read().properties())
  }

  /// Sets a custom property in the DID Document.
  /// If the value is set to `null`, the custom property will be removed.
  ///
  /// ### WARNING
  ///
  /// This method can overwrite existing properties like `id` and result in an invalid document.
  #[wasm_bindgen(js_name = setPropertyUnchecked)]
  pub fn set_property_unchecked(&mut self, key: String, value: &JsValue) -> Result<()> {
    let value: Option<serde_json::Value> = value.into_serde().wasm_result()?;
    match value {
      Some(value) => {
        self.0.blocking_write().properties_mut_unchecked().insert(key, value);
      }
      None => {
        self.0.blocking_write().properties_mut_unchecked().remove(&key);
      }
    }
    Ok(())
  }

  // ===========================================================================
  // Services
  // ===========================================================================

  /// Return a set of all {@link Service} in the document.
  #[wasm_bindgen]
  pub fn service(&self) -> ArrayService {
    self
      .0
      .blocking_read()
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
  pub fn insert_service(&mut self, service: &WasmService) -> Result<()> {
    self.0.blocking_write().insert_service(service.0.clone()).wasm_result()
  }

  /// Remove a {@link Service} identified by the given {@link DIDUrl} from the document.
  ///
  /// Returns `true` if a service was removed.
  #[wasm_bindgen(js_name = removeService)]
  pub fn remove_service(&mut self, did: &WasmDIDUrl) -> Option<WasmService> {
    self.0.blocking_write().remove_service(&did.0).map(Into::into)
  }

  /// Returns the first {@link Service} with an `id` property matching the provided `query`,
  /// if present.
  #[wasm_bindgen(js_name = resolveService)]
  pub fn resolve_service(&self, query: &UDIDUrlQuery) -> Option<WasmService> {
    let service_query: String = query.into_serde().ok()?;
    self.0.blocking_read().resolve_service(&service_query).cloned().map(WasmService::from)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns a list of all {@link VerificationMethod} in the DID Document,
  /// whose verification relationship matches `scope`.
  ///
  /// If `scope` is not set, a list over the **embedded** methods is returned.
  #[wasm_bindgen]
  pub fn methods(&self, scope: Option<RefMethodScope>) -> Result<ArrayVerificationMethod> {
    let scope: Option<MethodScope> = scope.map(|js| js.into_serde().wasm_result()).transpose()?;
    let methods = self
      .0
      .blocking_read()
      .methods(scope)
      .into_iter()
      .cloned()
      .map(WasmVerificationMethod::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayVerificationMethod>();
    Ok(methods)
  }

  /// Adds a new `method` to the document in the given `scope`.
  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &WasmVerificationMethod, scope: &WasmMethodScope) -> Result<()> {
    self.0.blocking_write().insert_method(method.0.clone(), scope.0).wasm_result()?;
    Ok(())
  }

  /// Removes all references to the specified Verification Method.
  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &WasmDIDUrl) -> Option<WasmVerificationMethod> {
    self.0.blocking_write().remove_method(&did.0).map(Into::into)
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

    let guard = self.0.blocking_read();
    let method: Option<&VerificationMethod> = guard.resolve_method(&method_query, method_scope);
    Ok(method.cloned().map(WasmVerificationMethod))
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
      .blocking_write()
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
      .blocking_write()
      .detach_method_relationship(&didUrl.0, relationship.into())
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
    methodQuery: &UDIDUrlQuery,
    options: &WasmProofOptions,
  ) -> Result<WasmCredential> {
    let mut data: Credential = credential.0.clone();
    let private_key: PrivateKey = privateKey.into();
    let method_query: String = methodQuery.into_serde().wasm_result()?;
    let options: ProofOptions = options.0.clone();

    self
      .0
      .blocking_read()
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
      .blocking_read()
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
      .blocking_read()
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
    Ok(self.0.blocking_read().verify_data(&data, &options.0).is_ok())
  }

  // ===========================================================================
  // Publishing
  // ===========================================================================

  /// Serializes the document for inclusion in an Alias Output's state metadata
  /// with the default {@link StateMetadataEncoding}.
  #[wasm_bindgen]
  pub fn pack(&self) -> Result<Vec<u8>> {
    self.0.blocking_read().clone().pack().wasm_result()
  }

  /// Serializes the document for inclusion in an Alias Output's state metadata.
  #[wasm_bindgen(js_name = packWithEncoding)]
  pub fn pack_with_encoding(&self, encoding: WasmStateMetadataEncoding) -> Result<Vec<u8>> {
    self
      .0
      .blocking_read()
      .clone()
      .pack_with_encoding(StateMetadataEncoding::from(encoding))
      .wasm_result()
  }

  /// Deserializes the document from an Alias Output.
  ///
  /// If `allowEmpty` is true, this will return an empty DID document marked as `deactivated`
  /// if `stateMetadata` is empty.
  ///
  /// The `tokenSupply` must be equal to the token supply of the network the DID is associated with.  
  ///
  /// NOTE: `did` is required since it is omitted from the serialized DID Document and
  /// cannot be inferred from the state metadata. It also indicates the network, which is not
  /// encoded in the `AliasId` alone.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = unpackFromOutput)]
  pub fn unpack_from_output(
    did: &WasmIotaDID,
    aliasOutput: IAliasOutput,
    allowEmpty: bool,
    tokenSupply: u64,
  ) -> Result<WasmIotaDocument> {
    let alias_dto: AliasOutputDto = aliasOutput.into_serde().wasm_result()?;
    let alias_output: AliasOutput = AliasOutput::try_from_dto(&alias_dto, tokenSupply)
      .map_err(|err| {
        identity_iota::iota::Error::JsError(format!("get_alias_output failed to convert AliasOutputDto: {err}"))
      })
      .wasm_result()?;
    IotaDocument::unpack_from_output(&did.0, &alias_output, allowEmpty)
      .map(WasmIotaDocument::from)
      .wasm_result()
  }

  /// Returns all DID documents of the Alias Outputs contained in the block's transaction payload
  /// outputs, if any.
  ///
  /// Errors if any Alias Output does not contain a valid or empty DID Document.
  ///
  /// `protocolResponseJson` can be obtained from a `Client`.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = unpackFromBlock)]
  pub fn unpack_from_block(
    network: String,
    block: &IBlock,
    protocol_parameters: &INodeInfoProtocol,
  ) -> Result<ArrayIotaDocument> {
    let network_name: NetworkName = NetworkName::try_from(network).wasm_result()?;
    let block_dto: iota_types::block::BlockDto = block
      .into_serde()
      .map_err(|err| {
        identity_iota::iota::Error::JsError(format!("unpackFromBlock failed to deserialize BlockDto: {err}"))
      })
      .wasm_result()?;

    let protocol_parameters_dto: ProtocolParametersDto = protocol_parameters
      .into_serde()
      .map_err(|err| identity_iota::iota::Error::JsError(format!("could not obtain protocolParameters: {err}")))
      .wasm_result()?;

    let protocol_parameters: ProtocolParameters = ProtocolParameters::try_from(protocol_parameters_dto)
      .map_err(|err| identity_iota::iota::Error::JsError(format!("could not obtain protocolParameters: {err}")))
      .wasm_result()?;

    let block: iota_types::block::Block = iota_types::block::Block::try_from_dto(&block_dto, &protocol_parameters)
      .map_err(|err| identity_iota::iota::Error::JsError(format!("unpackFromBlock failed to convert BlockDto: {err}")))
      .wasm_result()?;

    Ok(
      IotaDocument::unpack_from_block(&network_name, &block)
        .wasm_result()?
        .into_iter()
        .map(WasmIotaDocument::from)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<ArrayIotaDocument>(),
    )
  }

  // ===========================================================================
  // Metadata
  // ===========================================================================

  /// Returns a copy of the metadata associated with this document.
  ///
  /// NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
  /// `metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.
  #[wasm_bindgen]
  pub fn metadata(&self) -> WasmIotaDocumentMetadata {
    WasmIotaDocumentMetadata::from(self.0.blocking_read().metadata.clone())
  }

  /// Returns a copy of the timestamp of when the DID document was created.
  #[wasm_bindgen(js_name = metadataCreated)]
  pub fn metadata_created(&self) -> Option<WasmTimestamp> {
    self.0.blocking_read().metadata.created.map(WasmTimestamp::from)
  }

  /// Sets the timestamp of when the DID document was created.
  #[wasm_bindgen(js_name = setMetadataCreated)]
  pub fn set_metadata_created(&mut self, timestamp: OptionTimestamp) -> Result<()> {
    let timestamp: Option<Timestamp> = timestamp.into_serde().wasm_result()?;
    self.0.blocking_write().metadata.created = timestamp;
    Ok(())
  }

  /// Returns a copy of the timestamp of the last DID document update.
  #[wasm_bindgen(js_name = metadataUpdated)]
  pub fn metadata_updated(&self) -> Option<WasmTimestamp> {
    self.0.blocking_read().metadata.updated.map(WasmTimestamp::from)
  }

  /// Sets the timestamp of the last DID document update.
  #[wasm_bindgen(js_name = setMetadataUpdated)]
  pub fn set_metadata_updated(&mut self, timestamp: OptionTimestamp) -> Result<()> {
    let timestamp: Option<Timestamp> = timestamp.into_serde().wasm_result()?;
    self.0.blocking_write().metadata.updated = timestamp;
    Ok(())
  }

  /// Returns a copy of the deactivated status of the DID document.
  #[wasm_bindgen(js_name = metadataDeactivated)]
  pub fn metadata_deactivated(&self) -> Option<bool> {
    self.0.blocking_read().metadata.deactivated
  }

  /// Sets the deactivated status of the DID document.
  #[wasm_bindgen(js_name = setMetadataDeactivated)]
  pub fn set_metadata_deactivated(&mut self, deactivated: Option<bool>) {
    self.0.blocking_write().metadata.deactivated = deactivated;
  }

  /// Returns a copy of the Bech32-encoded state controller address, if present.
  #[wasm_bindgen(js_name = metadataStateControllerAddress)]
  pub fn metadata_state_controller_address(&self) -> Option<String> {
    self.0.blocking_read().metadata.state_controller_address.clone()
  }

  /// Returns a copy of the Bech32-encoded governor address, if present.
  #[wasm_bindgen(js_name = metadataGovernorAddress)]
  pub fn metadata_governor_address(&self) -> Option<String> {
    self.0.blocking_read().metadata.governor_address.clone()
  }

  /// Sets a custom property in the document metadata.
  /// If the value is set to `null`, the custom property will be removed.
  #[wasm_bindgen(js_name = setMetadataPropertyUnchecked)]
  pub fn set_metadata_property_unchecked(&mut self, key: String, value: &JsValue) -> Result<()> {
    let value: Option<serde_json::Value> = value.into_serde().wasm_result()?;
    match value {
      Some(value) => {
        self.0.blocking_write().metadata.properties.insert(key, value);
      }
      None => {
        self.0.blocking_write().metadata.properties.remove(&key);
      }
    }
    Ok(())
  }

  // ===========================================================================
  // Revocation
  // ===========================================================================

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// revoke all specified `indices`.
  #[wasm_bindgen(js_name = revokeCredentials)]
  #[allow(non_snake_case)]
  pub fn revoke_credentials(&mut self, serviceQuery: &UDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.blocking_write().revoke_credentials(&query, indices.as_slice()).wasm_result()
  }

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// unrevoke all specified `indices`.
  #[wasm_bindgen(js_name = unrevokeCredentials)]
  #[allow(non_snake_case)]
  pub fn unrevoke_credentials(&mut self, serviceQuery: &UDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.blocking_write().unrevoke_credentials(&query, indices.as_slice()).wasm_result()
  }

  // ===========================================================================
  // Cloning
  // ===========================================================================

  #[wasm_bindgen(js_name = clone)]
  /// Returns a deep clone of the `IotaDocument`. 
  pub fn deep_clone(&self) -> WasmIotaDocument {
    WasmIotaDocument(Rc::new(IotaDocumentLock::new(self.0.blocking_read().clone())))
  }

  #[wasm_bindgen(js_name = shallowClone, skip_typescript)]
  pub fn shallow_clone(&self) -> WasmIotaDocument {
    WasmIotaDocument(self.0.clone())
  }

  // ===========================================================================
  // Serialization
  // ===========================================================================
    
  /// Serializes this to a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> crate::error::Result<JsValue> {
    use crate::error::WasmResult;
    JsValue::from_serde(&self.0.blocking_read().as_ref()).wasm_result()
  }

  /// Deserializes an instance from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> crate::error::Result<WasmIotaDocument> {
    use crate::error::WasmResult;
    json
      .into_serde()
      .map(|value| Self(Rc::new(IotaDocumentLock::new(value))))
      .wasm_result()
  }

  // ===========================================================================
  // "AsRef<CoreDocument>"
  // ===========================================================================
  /// Transforms the `IotaDocument` to its `CoreDocument` representation. 
  #[wasm_bindgen(js_name = asCoreDocument)]
  pub fn as_core_document(&self) -> WasmCoreDocument {
    WasmCoreDocument(Rc::new(CoreDocumentLock::new(self.0.blocking_read().core_document().clone())))
  }
}


impl From<IotaDocument> for WasmIotaDocument {
  fn from(document: IotaDocument) -> Self {
    Self(Rc::new(IotaDocumentLock::new(document)))
  }
}

/*
impl From<WasmIotaDocument> for IotaDocument {
  fn from(wasm_document: WasmIotaDocument) -> Self {
    wasm_document.0
  }
}
 */

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IotaDID[]")]
  pub type ArrayIotaDID;

  #[wasm_bindgen(typescript_type = "IotaDocument[]")]
  pub type ArrayIotaDocument;

  // External interface from `@iota/types`, must be deserialized via BlockDto.
  #[wasm_bindgen(typescript_type = "IBlock")]
  pub type IBlock;

  // External interface from `@iota/types`, must be deserialized via ProtocolParameters.
  #[wasm_bindgen(typescript_type = "INodeInfoProtocol")]
  pub type INodeInfoProtocol;
}

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_IMPORTS: &'static str = r#"import type { IBlock, INodeInfoProtocol } from '@iota/types';"#;
