// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Object;
use identity::core::OneOrMany;
use identity::core::SerdeInto;
use identity::core::Timestamp;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::wasm_document::WasmDocument;

#[wasm_bindgen(inspectable)]
#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential(pub(crate) Credential);

#[wasm_bindgen]
impl VerifiableCredential {
  #[wasm_bindgen]
  pub fn extend(value: &JsValue) -> Result<VerifiableCredential, JsValue> {
    let mut base: Object = value.into_serde().map_err(wasm_error)?;

    if !base.contains_key("credentialSubject") {
      return Err("Missing property: `credentialSubject`".into());
    }

    if !base.contains_key("issuer") {
      return Err("Missing property: `issuer`".into());
    }

    if !base.contains_key("@context") {
      base.insert(
        "@context".into(),
        Credential::<()>::base_context().serde_into().map_err(wasm_error)?,
      );
    }

    let mut types: Vec<String> = match base.remove("type") {
      Some(value) => value.serde_into().map(OneOrMany::into_vec).map_err(wasm_error)?,
      None => Vec::new(),
    };

    types.insert(0, Credential::<()>::base_type().into());
    base.insert("type".into(), types.serde_into().map_err(wasm_error)?);

    if !base.contains_key("issuanceDate") {
      base.insert("issuanceDate".into(), Timestamp::now_utc().to_string().into());
    }

    base.serde_into().map_err(wasm_error).map(Self)
  }

  #[wasm_bindgen]
  pub fn issue(
    issuer_doc: &WasmDocument,
    subject_data: &JsValue,
    credential_type: Option<String>,
    credential_id: Option<String>,
  ) -> Result<VerifiableCredential, JsValue> {
    let subjects: OneOrMany<Subject> = subject_data.into_serde().map_err(wasm_error)?;
    let issuer_url: Url = Url::parse(issuer_doc.0.id().as_str()).map_err(wasm_error)?;
    let mut builder: CredentialBuilder = CredentialBuilder::default().issuer(issuer_url);

    for subject in subjects.into_vec() {
      builder = builder.subject(subject);
    }

    if let Some(credential_type) = credential_type {
      builder = builder.type_(credential_type);
    }

    if let Some(credential_id) = credential_id {
      builder = builder.id(Url::parse(credential_id).map_err(wasm_error)?);
    }

    builder.build().map(Self).map_err(wasm_error)
  }

  /// Serializes a `VerifiableCredential` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    JsValue::from_serde(&self.0).map_err(wasm_error)
  }

  /// Deserializes a `VerifiableCredential` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<VerifiableCredential, JsValue> {
    json.into_serde().map_err(wasm_error).map(Self)
  }
}
