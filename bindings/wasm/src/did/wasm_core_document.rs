// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::WasmCoreDID;
use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::common::OptionOneOrManyString;
use crate::common::UOneOrManyNumber;
use crate::crypto::WasmProofOptions;
use crate::did::wasm_core_service::WasmCoreService;
use crate::did::wasm_core_url::WasmCoreDIDUrl;
use crate::did::wasm_core_verification_method::WasmCoreVerificationMethod;
use crate::did::RefMethodScope;
use crate::did::WasmMethodRelationship;
use crate::did::WasmMethodScope;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::core::Object;
use identity_iota::core::OneOrMany;
use identity_iota::core::OneOrSet;
use identity_iota::core::OrderedSet;
use identity_iota::core::Url;
use identity_iota::credential::RevocationDocumentExt;
use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::CoreDID;
use identity_iota::document::verifiable::VerifiableProperties;
use identity_iota::document::CoreDocument;
use identity_iota::document::Document;
use identity_iota::document::Service;
use identity_iota::verification::MethodRef;
use identity_iota::verification::MethodScope;
use identity_iota::verification::VerificationMethod;

use proc_typescript::typescript;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// A method-agnostic DID Document.
#[wasm_bindgen(js_name = CoreDocument, inspectable)]
pub struct WasmCoreDocument(pub(crate) CoreDocument);

#[wasm_bindgen(js_class = CoreDocument)]
impl WasmCoreDocument {
  /// Creates a new `CoreDocument` with the given properties.
  #[wasm_bindgen(constructor)]
  pub fn new(values: ICoreDocument) -> Result<WasmCoreDocument> {
    let core_doc: CoreDocument = values.into_serde().wasm_result()?;
    Ok(WasmCoreDocument(core_doc))
  }

  /// Returns a copy of the DID Document `id`.
  #[wasm_bindgen]
  pub fn id(&self) -> WasmCoreDID {
    WasmCoreDID::from(self.0.id().clone())
  }

  /// Sets the DID of the document.
  ///
  /// ### Warning
  ///
  /// Changing the identifier can drastically alter the results of
  /// [`Self::resolve_method`](CoreDocument::resolve_method()),
  /// [`Self::resolve_service`](CoreDocument::resolve_service()) and the related [DID URL dereferencing](https://w3c-ccg.github.io/did-resolution/#dereferencing) algorithm.
  #[wasm_bindgen(js_name = setId)]
  pub fn set_id(&mut self, id: &WasmCoreDID) {
    *self.0.id_mut_unchecked() = id.0.clone();
  }

  /// Returns a copy of the document controllers.
  #[wasm_bindgen]
  pub fn controller(&self) -> ArrayCoreDID {
    match self.0.controller() {
      Some(controllers) => controllers
        .iter()
        .cloned()
        .map(WasmCoreDID::from)
        .map(JsValue::from)
        .collect::<js_sys::Array>()
        .unchecked_into::<ArrayCoreDID>(),
      None => js_sys::Array::new().unchecked_into::<ArrayCoreDID>(),
    }
  }

  /// Sets the controllers of the DID Document.
  ///
  /// Note: Duplicates will be ignored.
  /// Use `null` to remove all controllers.
  #[wasm_bindgen(js_name = setController)]
  pub fn set_controller(&mut self, controllers: &OptionOneOrManyCoreDID) -> Result<()> {
    let controllers: Option<OneOrMany<CoreDID>> = controllers.into_serde().wasm_result()?;
    let controller_set: Option<OneOrSet<CoreDID>> = if let Some(controllers) = controllers.map(OneOrMany::into_vec) {
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

  /// Returns a copy of the document's `verificationMethod` set.
  #[wasm_bindgen(js_name = verificatonMethod)]
  pub fn verification_method(&self) -> ArrayCoreVerificationMethod {
    self
      .0
      .verification_method()
      .iter()
      .cloned()
      .map(WasmCoreVerificationMethod::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreVerificationMethod>()
  }

  /// Returns a copy of the document's `authentication` set.
  #[wasm_bindgen]
  pub fn authentication(&self) -> ArrayCoreMethodRef {
    self
      .0
      .authentication()
      .iter()
      .cloned()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(verification_method) => JsValue::from(WasmCoreVerificationMethod(verification_method)),
        MethodRef::Refer(did_url) => JsValue::from(WasmCoreDIDUrl(did_url)),
      })
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreMethodRef>()
  }

  /// Returns a copy of the document's `assertionMethod` set.
  #[wasm_bindgen(js_name = assertionMethod)]
  pub fn assertion_method(&self) -> ArrayCoreMethodRef {
    self
      .0
      .assertion_method()
      .iter()
      .cloned()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(verification_method) => JsValue::from(WasmCoreVerificationMethod(verification_method)),
        MethodRef::Refer(did_url) => JsValue::from(WasmCoreDIDUrl(did_url)),
      })
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreMethodRef>()
  }

  /// Returns a copy of the document's `keyAgreement` set.
  #[wasm_bindgen(js_name = keyAgreement)]
  pub fn key_agreement(&self) -> ArrayCoreMethodRef {
    self
      .0
      .key_agreement()
      .iter()
      .cloned()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(verification_method) => JsValue::from(WasmCoreVerificationMethod(verification_method)),
        MethodRef::Refer(did_url) => JsValue::from(WasmCoreDIDUrl(did_url)),
      })
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreMethodRef>()
  }

  /// Returns a copy of the document's `capabilityDelegation` set.
  #[wasm_bindgen(js_name = capabilityDelegation)]
  pub fn capability_delegation(&self) -> ArrayCoreMethodRef {
    self
      .0
      .capability_delegation()
      .iter()
      .cloned()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(verification_method) => JsValue::from(WasmCoreVerificationMethod(verification_method)),
        MethodRef::Refer(did_url) => JsValue::from(WasmCoreDIDUrl(did_url)),
      })
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreMethodRef>()
  }

  /// Returns a copy of the document's `capabilityInvocation` set.
  #[wasm_bindgen(js_name = capabilityInvocation)]
  pub fn capability_invocation(&self) -> ArrayCoreMethodRef {
    self
      .0
      .capability_invocation()
      .iter()
      .cloned()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(verification_method) => JsValue::from(WasmCoreVerificationMethod(verification_method)),
        MethodRef::Refer(did_url) => JsValue::from(WasmCoreDIDUrl(did_url)),
      })
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreMethodRef>()
  }

  /// Returns a copy of the custom DID Document properties.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    MapStringAny::try_from(self.0.properties())
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
        self.0.properties_mut_unchecked().insert(key, value);
      }
      None => {
        self.0.properties_mut_unchecked().remove(&key);
      }
    }
    Ok(())
  }

  // ===========================================================================
  // Services
  // ===========================================================sdfs================

  /// Returns a set of all {@link CoreService} in the document.
  #[wasm_bindgen]
  pub fn service(&self) -> ArrayCoreService {
    self
      .0
      .service()
      .iter()
      .cloned()
      .map(WasmCoreService)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreService>()
  }

  /// Add a new {@link CoreService} to the document.
  ///
  /// Errors if there already exists a service or verification method with the same id.
  #[wasm_bindgen(js_name = insertService)]
  pub fn insert_service(&mut self, service: &WasmCoreService) -> Result<()> {
    self.0.insert_service(service.0.clone()).wasm_result()
  }

  /// Remoce a {@link CoreService} identified by the given {@link CoreDIDUrl} from the document.
  ///
  /// Returns `true` if the service was removed.
  #[wasm_bindgen(js_name = removeService)]
  #[allow(non_snake_case)]
  pub fn remove_service(&mut self, didUrl: &WasmCoreDIDUrl) -> Option<WasmCoreService> {
    self.0.remove_service(&didUrl.0.clone()).map(Into::into)
  }

  /// Returns the first {@link CoreService} with an `id` property matching the provided `query`,
  /// if present.
  #[wasm_bindgen(js_name = resolveService)]
  pub fn resolve_service(&self, query: &UCoreDIDUrlQuery) -> Option<WasmCoreService> {
    let service_query: String = query.into_serde().ok()?;
    self
      .0
      .resolve_service(&service_query)
      .cloned()
      .map(WasmCoreService::from)
  }

  // ===========================================================================
  // Verification Methods
  // ===========================================================================

  /// Returns a list of all {@link CoreVerificationMethod} in the DID Document,
  /// whose verification relationship matches `scope`.
  ///
  /// If `scope` is not set, a list over the **embedded** methods is returned.
  #[wasm_bindgen]
  pub fn methods(&self, scope: Option<RefMethodScope>) -> Result<ArrayCoreVerificationMethod> {
    let scope: Option<MethodScope> = scope.map(|js| js.into_serde().wasm_result()).transpose()?;
    let methods = self
      .0
      .methods(scope)
      .into_iter()
      .cloned()
      .map(WasmCoreVerificationMethod::from)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreVerificationMethod>();
    Ok(methods)
  }

  /// Returns an array of all verification relationships.
  #[wasm_bindgen(js_name = verificationRelationships)]
  pub fn verification_relationships(&self) -> ArrayCoreMethodRef {
    self
      .0
      .verification_relationships()
      .cloned()
      .map(|method_ref| match method_ref {
        MethodRef::Embed(verification_method) => JsValue::from(WasmCoreVerificationMethod(verification_method)),
        MethodRef::Refer(did_url) => JsValue::from(WasmCoreDIDUrl(did_url)),
      })
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayCoreMethodRef>()
  }

  /// Adds a new `method` to the document in the given `scope`.
  #[wasm_bindgen(js_name = insertMethod)]
  pub fn insert_method(&mut self, method: &WasmCoreVerificationMethod, scope: &WasmMethodScope) -> Result<()> {
    self.0.insert_method(method.0.clone(), scope.0).wasm_result()
  }

  /// Removes all references to the specified Verification Method.
  #[wasm_bindgen(js_name = removeMethod)]
  pub fn remove_method(&mut self, did: &WasmCoreDIDUrl) -> Option<WasmCoreVerificationMethod> {
    self.0.remove_method(&did.0).map(Into::into)
  }

  /// Returns a copy of the first verification method with an `id` property
  /// matching the provided `query` and the verification relationship
  /// specified by `scope`, if present.
  #[wasm_bindgen(js_name = resolveMethod)]
  pub fn resolve_method(
    &self,
    query: &UCoreDIDUrlQuery,
    scope: Option<RefMethodScope>,
  ) -> Result<Option<WasmCoreVerificationMethod>> {
    let method_query: String = query.into_serde().wasm_result()?;
    let method_scope: Option<MethodScope> = scope.map(|js| js.into_serde().wasm_result()).transpose()?;

    let method: Option<&VerificationMethod> = self.0.resolve_method(&method_query, method_scope);
    Ok(method.cloned().map(WasmCoreVerificationMethod))
  }

  /// Attaches the relationship to the given method, if the method exists.
  ///
  /// Note: The method needs to be in the set of verification methods,
  /// so it cannot be an embedded one.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = attachMethodRelationship)]
  pub fn attach_method_relationship(
    &mut self,
    didUrl: &WasmCoreDIDUrl,
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
    didUrl: &WasmCoreDIDUrl,
    relationship: WasmMethodRelationship,
  ) -> Result<bool> {
    self
      .0
      .detach_method_relationship(&didUrl.0, relationship.into())
      .wasm_result()
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
  // Revocation
  // ===========================================================================

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// revoke all specified `indices`.
  #[wasm_bindgen(js_name = revokeCredentials)]
  #[allow(non_snake_case)]
  pub fn revoke_credentials(&mut self, serviceQuery: &UCoreDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.revoke_credentials(&query, indices.as_slice()).wasm_result()
  }

  /// If the document has a `RevocationBitmap` service identified by `serviceQuery`,
  /// unrevoke all specified `indices`.
  #[wasm_bindgen(js_name = unrevokeCredentials)]
  #[allow(non_snake_case)]
  pub fn unrevoke_credentials(&mut self, serviceQuery: &UCoreDIDUrlQuery, indices: UOneOrManyNumber) -> Result<()> {
    let query: String = serviceQuery.into_serde().wasm_result()?;
    let indices: OneOrMany<u32> = indices.into_serde().wasm_result()?;

    self.0.unrevoke_credentials(&query, indices.as_slice()).wasm_result()
  }

  // ===========================================================================
  // Signatures
  // ===========================================================================

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
    methodQuery: &UCoreDIDUrlQuery,
    options: &WasmProofOptions,
  ) -> Result<JsValue> {
    let mut data: VerifiableProperties = data.into_serde().wasm_result()?;
    let private_key: PrivateKey = privateKey.into();
    let method_query: String = methodQuery.into_serde().wasm_result()?;
    let options: ProofOptions = options.0.clone();

    let signer = self.0.signer(&private_key);
    let signer = signer.options(options);
    let signer = signer.method(&method_query);
    signer.sign(&mut data).wasm_result()?;
    JsValue::from_serde(&data).wasm_result()
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ICoreDocument")]
  pub type ICoreDocument;

  #[wasm_bindgen(typescript_type = "CoreDID[]")]
  pub type ArrayCoreDID;

  #[wasm_bindgen(typescript_type = "CoreVerificationMethod[]")]
  pub type ArrayCoreVerificationMethod;

  #[wasm_bindgen(typescript_type = "Array<CoreDIDUrl | CoreVerificationMethod>")]
  pub type ArrayCoreMethodRef;

  #[wasm_bindgen(typescript_type = "CoreDIDUrl | string")]
  pub type UCoreDIDUrlQuery;

  #[wasm_bindgen(typescript_type = "CoreDID | CoreDID[] | null")]
  pub type OptionOneOrManyCoreDID;

  #[wasm_bindgen(typescript_type = "CoreService[]")]
  pub type ArrayCoreService;
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[typescript(name = "ICoreDocument", readonly, optional)]
#[allow(non_snake_case, dead_code)]
struct ICoreDocumentHelper {
  #[typescript(optional = false, type = "string | CoreDID | IotaDID")]
  id: Option<CoreDID>,

  #[typescript(type = "(string | CoreDID | IotaDID)[]")]
  controller: Option<OneOrSet<CoreDID>>,

  #[typescript(type = "string[]")]
  alsoKnownAs: Option<OrderedSet<Url>>,

  #[typescript(type = "(CoreVerificationMethod | IotaVerificationMethod)[]")]
  verificationMethod: Option<OrderedSet<VerificationMethod<CoreDID, Object>>>,

  #[typescript(type = "(CoreVerificationMethod | CoreDIDUrl | IotaVerificationMethod | IotaDIDUrl)[]")]
  authentication: Option<OrderedSet<VerificationMethod<CoreDID, Object>>>,

  #[typescript(type = "(CoreVerificationMethod | CoreDIDUrl | IotaVerificationMethod | IotaDIDUrl)[]")]
  assertionMethod: Option<OrderedSet<VerificationMethod<CoreDID, Object>>>,

  #[typescript(type = "(CoreVerificationMethod | CoreDIDUrl | IotaVerificationMethod | IotaDIDUrl)[]")]
  keyAgreement: Option<OrderedSet<VerificationMethod<CoreDID, Object>>>,

  #[typescript(type = "(CoreVerificationMethod | CoreDIDUrl | IotaVerificationMethod | IotaDIDUrl)[]")]
  capabilityDelegation: Option<OrderedSet<VerificationMethod<CoreDID, Object>>>,

  #[typescript(type = "(CoreVerificationMethod | CoreDIDUrl | IotaVerificationMethod | IotaDIDUrl)[]")]
  capabilityInvocation: Option<OrderedSet<VerificationMethod<CoreDID, Object>>>,

  #[typescript(type = "(CoreService | IotaService)[]")]
  service: Option<OrderedSet<Service<CoreDID, Object>>>,

  #[serde(flatten)]
  #[typescript(optional = false, name = "[properties: string]", type = "unknown")]
  properties: Object,
}

impl_wasm_json!(WasmCoreDocument, CoreDocument);
impl_wasm_clone!(WasmCoreDocument, CoreDocument);

impl From<CoreDocument> for WasmCoreDocument {
  fn from(doc: CoreDocument) -> Self {
    Self(doc)
  }
}
