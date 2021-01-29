// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::{
  convert::{FromJson as _, SerdeInto as _},
  did_doc::{DIDKey, Document, DocumentBuilder, MethodIndex, MethodScope, Service, ServiceBuilder, VerifiableDocument},
};
use identity_iota::{
  did::{DocumentDiff, IotaDocument, Properties},
  tangle::MessageId,
};
use wasm_bindgen::prelude::*;

use crate::{
  did::DID,
  js_err,
  key::Key,
  pubkey::{PubKey, DEFAULT_KEY},
};

#[wasm_bindgen(inspectable)]
pub struct NewDoc {
  key: Key,
  doc: Doc,
}

#[wasm_bindgen]
impl NewDoc {
  #[wasm_bindgen(getter)]
  pub fn key(&self) -> Key {
    self.key.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn doc(&self) -> Doc {
    self.doc.clone()
  }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct Doc(pub(crate) IotaDocument);

#[wasm_bindgen]
impl Doc {
  #[wasm_bindgen(constructor)]
  pub fn new(authentication: &PubKey) -> Result<Doc, JsValue> {
    let mut did = authentication.0.id().clone();
    did.set_fragment(None);

    let base: VerifiableDocument<Properties> = DocumentBuilder::new(Properties::new())
      .id(did)
      // Note: We use a reference to the verification method due to
      // upstream limitations.
      .authentication(authentication.0.id().clone())
      .verification_method(authentication.0.clone())
      .build()
      .map(VerifiableDocument::new)
      .map_err(js_err)?;

    IotaDocument::try_from_document(base.serde_into().map_err(js_err)?)
      .map_err(js_err)
      .map(Self)
  }

  /// Generates a keypair and DID Document, supported key_type is "Ed25519VerificationKey2018"
  #[wasm_bindgen(js_name = generateRandom)]
  pub fn generate_random(key_type: &str, network: Option<String>, tag: Option<String>) -> Result<NewDoc, JsValue> {
    let key: Key = Key::new(key_type)?;
    let did: DID = DID::new(&key, network)?;
    let pkey: PubKey = PubKey::new(&did, key_type, &key.public(), tag)?;

    Ok(NewDoc {
      doc: Self::new(&pkey)?,
      key,
    })
  }

  /// Generates an Ed25519 keypair and DID Document
  #[wasm_bindgen(js_name = generateEd25519)]
  pub fn generate_ed25519(network: Option<String>, tag: Option<String>) -> Result<NewDoc, JsValue> {
    Self::generate_random(DEFAULT_KEY, network, tag)
  }

  #[wasm_bindgen(getter)]
  pub fn id(&self) -> String {
    self.0.id().to_string()
  }

  #[wasm_bindgen(getter, js_name = authChain)]
  pub fn auth_chain(&self) -> String {
    self.0.id().address()
  }

  #[wasm_bindgen(getter, js_name = diffChain)]
  pub fn diff_chain(&self) -> String {
    String::new() // TODO: FIXME
  }

  #[wasm_bindgen(getter)]
  pub fn proof(&self) -> Result<JsValue, JsValue> {
    self
      .0
      .proof()
      .map(|proof| JsValue::from_serde(proof))
      .transpose()
      .map_err(js_err)
      .map(|option| option.unwrap_or(JsValue::NULL))
  }

  #[wasm_bindgen]
  pub fn sign(&mut self, key: &Key) -> Result<JsValue, JsValue> {
    self.0.sign(key.0.secret()).map_err(js_err).map(|_| JsValue::NULL)
  }

  /// Verify the signature with the authentication_key
  #[wasm_bindgen]
  pub fn verify(&self) -> bool {
    self.0.verify().is_ok()
  }

  /// Generate the difference between two DID Documents and sign it
  #[wasm_bindgen]
  pub fn diff(&self, other: &Doc, key: &Key, prev_msg: String) -> Result<JsValue, JsValue> {
    let doc: IotaDocument = other.0.clone();

    let diff: DocumentDiff = self
      .0
      .diff(&doc, key.0.secret(), MessageId::new(prev_msg))
      .map_err(js_err)?;

    JsValue::from_serde(&diff).map_err(js_err)
  }

  /// Verify the signature in a diff with the authentication_key
  #[wasm_bindgen(js_name = verifyDiff)]
  pub fn verify_diff(&self, diff: String) -> bool {
    match DocumentDiff::from_json(&diff) {
      Ok(diff) => self.0.verify_data(&diff).is_ok(),
      Err(_) => false,
    }
  }

  #[wasm_bindgen(js_name = updateService)]
  pub fn update_service(&mut self, did: DID, url: String, service_type: String) -> Result<(), JsValue> {
    let service: Service = ServiceBuilder::default()
      .id(did.0.into())
      .type_(service_type)
      .service_endpoint(url.parse().map_err(js_err)?)
      .build()
      .map_err(js_err)?;

    Self::mutate(self, |doc| doc.service_mut().update(DIDKey::new(service)))?;

    Ok(())
  }

  #[wasm_bindgen(js_name = clearServices)]
  pub fn clear_services(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.service_mut().clear())
  }

  #[wasm_bindgen(js_name = updateAuth)]
  pub fn update_auth(&mut self, public_key: &PubKey) -> Result<bool, JsValue> {
    Self::mutate(self, |doc| {
      doc
        .authentication_mut()
        .update(DIDKey::new(public_key.0.clone().into()))
    })
  }

  #[wasm_bindgen(js_name = clearAuth)]
  pub fn clear_auth(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.authentication_mut().clear())
  }

  #[wasm_bindgen(js_name = updateAssert)]
  pub fn update_assert(&mut self, public_key: &PubKey) -> Result<bool, JsValue> {
    Self::mutate(self, |doc| {
      doc
        .assertion_method_mut()
        .update(DIDKey::new(public_key.0.clone().into()))
    })
  }

  #[wasm_bindgen(js_name = clearAssert)]
  pub fn clear_assert(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.assertion_method_mut().clear())
  }

  #[wasm_bindgen(js_name = updateVerification)]
  pub fn update_verification(&mut self, public_key: &PubKey) -> Result<bool, JsValue> {
    Self::mutate(self, |doc| {
      doc.verification_method_mut().update(DIDKey::new(public_key.0.clone()))
    })
  }

  #[wasm_bindgen(js_name = clearVerification)]
  pub fn clear_verification(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.verification_method_mut().clear())
  }

  #[wasm_bindgen(js_name = updateDelegation)]
  pub fn update_delegation(&mut self, public_key: &PubKey) -> Result<bool, JsValue> {
    Self::mutate(self, |doc| {
      doc
        .capability_delegation_mut()
        .update(DIDKey::new(public_key.0.clone().into()))
    })
  }

  #[wasm_bindgen(js_name = clearDelegation)]
  pub fn clear_delegation(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.capability_delegation_mut().clear())
  }

  #[wasm_bindgen(js_name = updateInvocation)]
  pub fn update_invocation(&mut self, public_key: &PubKey) -> Result<bool, JsValue> {
    Self::mutate(self, |doc| {
      doc
        .capability_invocation_mut()
        .update(DIDKey::new(public_key.0.clone().into()))
    })
  }

  #[wasm_bindgen(js_name = clearInvocation)]
  pub fn clear_invocation(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.capability_invocation_mut().clear())
  }

  #[wasm_bindgen(js_name = updateAgreement)]
  pub fn update_agreement(&mut self, public_key: &PubKey) -> Result<bool, JsValue> {
    Self::mutate(self, |doc| {
      doc.key_agreement_mut().update(DIDKey::new(public_key.0.clone().into()))
    })
  }

  #[wasm_bindgen(js_name = clearAgreement)]
  pub fn clear_agreement(&mut self) -> Result<(), JsValue> {
    Self::mutate(self, |doc| doc.key_agreement_mut().clear())
  }

  #[wasm_bindgen(js_name = resolveKey)]
  pub fn resolve_key(&mut self, ident: JsValue, scope: Option<String>) -> Result<PubKey, JsValue> {
    let borrow: String;

    let ident: MethodIndex = if let Some(number) = ident.as_f64() {
      MethodIndex::Index(number.to_string().parse().map_err(js_err)?)
    } else if let Some(ident) = ident.as_string() {
      borrow = ident;
      MethodIndex::Ident(&borrow)
    } else {
      return Err("Invalid Key Identifier".into());
    };

    let scope: MethodScope = scope
      .map(|scope| scope.parse::<MethodScope>())
      .transpose()
      .map_err(js_err)?
      .unwrap_or(MethodScope::Authentication);

    self
      .0
      .resolve((ident, scope))
      .map(|wrap| wrap.into_method().clone())
      .map(PubKey)
      .ok_or_else(|| "Key Not Found".into())
  }

  /// Serializes a `Doc` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(js_err)
  }

  /// Deserializes a `Doc` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<Doc, JsValue> {
    json.into_serde().map_err(js_err).map(Self)
  }

  // Bypass IotaDocument Deref limitations and allow modifications to the
  // core DID Document type.
  //
  // Uses `serde` for conversions and re-validates the document after mutation.
  fn mutate<T>(this: &mut Self, f: impl FnOnce(&mut Document) -> T) -> Result<T, JsValue> {
    let mut document: Document = this.0.serde_into().map_err(js_err)?;
    let output: T = f(&mut document);

    this.0 = IotaDocument::try_from_document(document).map_err(js_err)?;

    Ok(output)
  }
}
