// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::FromJson;
use identity::core::Object;
use identity::core::OneOrMany;
use identity::core::Timestamp;
use identity::core::ToJson;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::CredentialBuilder;
use identity::credential::Subject;
use identity::did::DID;
use wasm_bindgen::prelude::*;

use crate::did::WasmDocument;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Credential, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmCredential(pub(crate) Credential);

#[wasm_bindgen(js_class = Credential)]
impl WasmCredential {
  #[wasm_bindgen]
  pub fn extend(value: &JsValue) -> Result<WasmCredential> {
    let mut base: Object = value.into_serde().wasm_result()?;

    if !base.contains_key("credentialSubject") {
      return Err("Missing property: `credentialSubject`".into());
    }

    if !base.contains_key("issuer") {
      return Err("Missing property: `issuer`".into());
    }

    if !base.contains_key("@context") {
      base.insert(
        "@context".into(),
        serde_into(Credential::<()>::base_context()).wasm_result()?,
      );
    }

    let mut types: Vec<String> = match base.remove("type") {
      Some(value) => serde_into(value).map(OneOrMany::into_vec).wasm_result()?,
      None => Vec::new(),
    };

    types.insert(0, Credential::<()>::base_type().into());
    base.insert("type".into(), serde_into(types).wasm_result()?);

    if !base.contains_key("issuanceDate") {
      base.insert("issuanceDate".into(), Timestamp::now_utc().to_string().into());
    }

    serde_into(base).map(Self).wasm_result()
  }

  #[wasm_bindgen]
  pub fn issue(
    issuer_doc: &WasmDocument,
    subject_data: &JsValue,
    credential_type: Option<String>,
    credential_id: Option<String>,
  ) -> Result<WasmCredential> {
    let subjects: OneOrMany<Subject> = subject_data.into_serde().wasm_result()?;
    let issuer_url: Url = Url::parse(issuer_doc.0.id().as_str()).wasm_result()?;
    let mut builder: CredentialBuilder = CredentialBuilder::default().issuer(issuer_url);

    for subject in subjects.into_vec() {
      builder = builder.subject(subject);
    }

    if let Some(credential_type) = credential_type {
      builder = builder.type_(credential_type);
    }

    if let Some(credential_id) = credential_id {
      builder = builder.id(Url::parse(credential_id).wasm_result()?);
    }

    builder.build().map(Self).wasm_result()
  }

  /// Serializes a `Credential` object as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Credential` object from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmCredential> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmCredential, Credential);

impl From<Credential> for WasmCredential {
  fn from(credential: Credential) -> WasmCredential {
    Self(credential)
  }
}

/// Converts `T` to `U` by converting to/from JSON.
///
/// An escape-hatch for converting between types that represent the same JSON
/// structure.
fn serde_into<T, U>(obj: T) -> identity::core::Result<U>
where
  T: ToJson,
  U: FromJson,
{
  obj.to_json_value().and_then(U::from_json_value)
}
