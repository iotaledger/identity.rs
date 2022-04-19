// Copyright 2020-2022 IOTA Stiftung
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
use wasm_bindgen::JsCast;

use crate::common::ArrayString;
use crate::common::MapStringAny;
use crate::common::WasmTimestamp;
use crate::credential::ArrayContext;
use crate::credential::ArrayEvidence;
use crate::credential::ArrayPolicy;
use crate::credential::ArrayRefreshService;
use crate::credential::ArraySchema;
use crate::credential::ArrayStatus;
use crate::credential::ArraySubject;
use crate::credential::UrlOrIssuer;
use crate::crypto::WasmProof;
use crate::did::WasmDocument;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Credential, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmCredential(pub(crate) Credential);

#[wasm_bindgen(js_class = Credential)]
impl WasmCredential {
  /// Returns a copy of the JSON-LD context(s) applicable to the `Credential`.
  #[wasm_bindgen]
  pub fn context(&self) -> Result<ArrayContext> {
    self
      .0
      .context
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayContext>())
  }

  /// Returns a copy of the unique `URI` referencing the subject of the `Credential`.
  #[wasm_bindgen]
  pub fn id(&self) -> Option<String> {
    self.0.id.as_ref().map(|url| url.to_string())
  }

  /// Returns a copy of the URIs defining the type of the `Credential`.
  #[wasm_bindgen]
  pub fn types(&self) -> ArrayString {
    self
      .0
      .types
      .iter()
      .map(|s| s.as_str())
      .map(JsValue::from_str)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  /// Returns a copy of the `Credential` subject(s).
  #[wasm_bindgen(js_name = credentialSubject)]
  pub fn credential_subject(&self) -> Result<ArraySubject> {
    self
      .0
      .credential_subject
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArraySubject>())
  }

  /// Returns a copy of the issuer of the `Credential`.
  #[wasm_bindgen]
  pub fn issuer(&self) -> Result<UrlOrIssuer> {
    JsValue::from_serde(&self.0.issuer)
      .map(|value| value.unchecked_into::<UrlOrIssuer>())
      .wasm_result()
  }

  /// Returns a copy of the timestamp of when the `Credential` becomes valid.
  #[wasm_bindgen(js_name = "issuanceDate")]
  pub fn issuance_date(&self) -> WasmTimestamp {
    WasmTimestamp::from(self.0.issuance_date)
  }

  /// Returns a copy of the timestamp of when the `Credential` should no longer be considered valid.
  #[wasm_bindgen(js_name = "expirationDate")]
  pub fn expiration_date(&self) -> Option<WasmTimestamp> {
    self.0.expiration_date.map(WasmTimestamp::from)
  }

  /// Returns a copy of the information used to determine the current status of the `Credential`.
  #[wasm_bindgen(js_name = "credentialStatus")]
  pub fn credential_status(&self) -> Result<ArrayStatus> {
    self
      .0
      .credential_status
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayStatus>())
  }

  /// Returns a copy of the information used to assist in the enforcement of a specific `Credential` structure.
  #[wasm_bindgen(js_name = "credentialSchema")]
  pub fn credential_schema(&self) -> Result<ArraySchema> {
    self
      .0
      .credential_schema
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArraySchema>())
  }

  /// Returns a copy of the service(s) used to refresh an expired `Credential`.
  #[wasm_bindgen(js_name = "refreshService")]
  pub fn refresh_service(&self) -> Result<ArrayRefreshService> {
    self
      .0
      .refresh_service
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayRefreshService>())
  }

  /// Returns a copy of the terms-of-use specified by the `Credential` issuer.
  #[wasm_bindgen(js_name = "termsOfUse")]
  pub fn terms_of_use(&self) -> Result<ArrayPolicy> {
    self
      .0
      .terms_of_use
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayPolicy>())
  }

  /// Returns a copy of the human-readable evidence used to support the claims within the `Credential`.
  #[wasm_bindgen]
  pub fn evidence(&self) -> Result<ArrayEvidence> {
    self
      .0
      .evidence
      .iter()
      .map(JsValue::from_serde)
      .collect::<std::result::Result<js_sys::Array, _>>()
      .wasm_result()
      .map(|value| value.unchecked_into::<ArrayEvidence>())
  }

  /// Returns whether or not the `Credential` must only be contained within a {@link Presentation}
  /// with a proof issued from the `Credential` subject.
  #[wasm_bindgen(js_name = "nonTransferable")]
  pub fn non_transferable(&self) -> Option<bool> {
    self.0.non_transferable
  }

  /// Returns a copy of the proof used to verify the `Credential`.
  #[wasm_bindgen]
  pub fn proof(&self) -> Option<WasmProof> {
    self.0.proof.clone().map(WasmProof)
  }

  /// Returns a copy of the miscellaneous properties on the `Credential`.
  #[wasm_bindgen]
  pub fn properties(&self) -> Result<MapStringAny> {
    let map: js_sys::Map = js_sys::Map::new();
    for (key, value) in self.0.properties.iter() {
      map.set(
        &JsValue::from_str(key.as_str()),
        &JsValue::from_serde(&value).wasm_result()?,
      );
    }
    Ok(map.unchecked_into::<MapStringAny>())
  }

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

  /// Serializes a `Credential` to a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `Credential` from a JSON object.
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
