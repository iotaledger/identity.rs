use std::ops::Deref;

use identity_iota::credential::status_list_2021::StatusList2021Credential;
use identity_iota::credential::status_list_2021::StatusPurpose;
use wasm_bindgen::prelude::*;

use crate::credential::WasmCredential;
use crate::error::Result;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WasmStatusPurpose {
  Revocation = "revocation",
  Suspension = "suspension",
}

impl From<StatusPurpose> for WasmStatusPurpose {
  fn from(value: StatusPurpose) -> Self {
    match value {
      StatusPurpose::Revocation => Self::Revocation,
      StatusPurpose::Suspension => Self::Suspension,
    }
  }
}

impl From<WasmStatusPurpose> for StatusPurpose {
  fn from(value: WasmStatusPurpose) -> Self {
    match value {
      WasmStatusPurpose::Revocation => Self::Revocation,
      WasmStatusPurpose::Suspension => Self::Suspension,
      _ => panic!("Unsupported StatusPurpose"),
    }
  }
}

#[wasm_bindgen(js_name = "StatusList2021Credential")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(from = "StatusList2021Credential", into = "StatusList2021Credential")]
pub struct WasmStatusList2021Credential {
  pub(crate) inner: StatusList2021Credential,
  wasm_credential: WasmCredential,
}

impl Deref for WasmStatusList2021Credential {
  type Target = WasmCredential;
  fn deref(&self) -> &Self::Target {
    &self.wasm_credential
  }
}

impl From<StatusList2021Credential> for WasmStatusList2021Credential {
  fn from(value: StatusList2021Credential) -> Self {
    Self {
      wasm_credential: WasmCredential(value.clone().into_inner()),
      inner: value,
    }
  }
}

impl From<WasmStatusList2021Credential> for StatusList2021Credential {
  fn from(value: WasmStatusList2021Credential) -> Self {
    value.inner
  }
}

#[wasm_bindgen(js_class = StatusList2021Credential)]
impl WasmStatusList2021Credential {
  /// Creates a new {@link StatusList2021Credential}
  #[wasm_bindgen(constructor)]
  pub fn new(credential: WasmCredential) -> Result<WasmStatusList2021Credential> {
    StatusList2021Credential::try_from(credential.0)
      .map(Into::into)
      .map_err(|e| JsValue::from(JsError::new(&e.to_string())))
  }
  /// Returns the {@link StatusPurpose} of this {@link StatusList2021Credential}
  #[wasm_bindgen]
  pub fn purpose(&self) -> WasmStatusPurpose {
    self.inner.purpose().into()
  }
  /// Returns the state of the `index`-th entry, if any
  #[wasm_bindgen]
  pub fn entry(&self, index: usize) -> Result<Option<bool>> {
    self
      .inner
      .entry(index)
      .map_err(|e| JsValue::from(JsError::new(&e.to_string())))
  }
  /// Sets the `index`-th entry to `state`
  #[wasm_bindgen(js_name = "setEntry")]
  pub fn set_entry(&mut self, index: usize, state: bool) -> Result<()> {
    self
      .inner
      .update_status_list(|list| list.set(index, state))
      .map_err(|e| JsValue::from(JsError::new(&e.to_string())))?;

    self.wasm_credential = WasmCredential(self.inner.clone().into_inner());
    Ok(())
  }
  #[wasm_bindgen(js_name = "clone")]
  pub fn wasm_clone(&self) -> WasmStatusList2021Credential {
    self.clone()
  }
  #[wasm_bindgen(js_name = "fromJSON")]
  pub fn from_json(json: JsValue) -> Result<WasmStatusList2021Credential> {
    use crate::error::WasmResult;
    json.into_serde::<WasmStatusList2021Credential>().wasm_result()
  }
  #[wasm_bindgen(js_name = "toJSON")]
  pub fn to_json(&self) -> Result<JsValue> {
    use crate::error::WasmResult;
    JsValue::from_serde(self).wasm_result()
  }
}
