use super::WasmStatusPurpose;
use crate::error::Result;
use identity_iota::core::Url;
use identity_iota::credential::status_list_2021::StatusList2021Entry;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = StatusList2021Entry, inspectable)]
pub struct WasmStatusList2021Entry(pub(crate) StatusList2021Entry);

#[wasm_bindgen(js_class = StatusList2021Entry)]
impl WasmStatusList2021Entry {
  /// Creates a new {@link StatusList2021Entry}
  #[wasm_bindgen(constructor)]
  pub fn new(status_list: &str, purpose: WasmStatusPurpose, index: usize) -> Result<WasmStatusList2021Entry> {
    let status_list = Url::parse(status_list).map_err(|e| JsValue::from(JsError::new(&e.to_string())))?;
    Ok(Self(StatusList2021Entry::new(status_list, purpose.into(), index)))
  }
  /// Returns this `credentialStatus`'s `id`
  #[wasm_bindgen]
  pub fn id(&self) -> String {
    self.0.id().to_string()
  }
  /// Returns the purpose of this entry
  #[wasm_bindgen]
  pub fn purpose(&self) -> WasmStatusPurpose {
    self.0.purpose().into()
  }
  /// Returns the index of this entry
  #[wasm_bindgen]
  pub fn index(&self) -> usize {
    self.0.index()
  }
  /// Returns the referenced {@link StatusList2021Credential}'s url
  #[wasm_bindgen]
  pub fn credential(&self) -> String {
    self.0.credential().to_string()
  }
}

impl_wasm_clone!(WasmStatusList2021Entry, StatusList2021Entry);
impl_wasm_json!(WasmStatusList2021Entry, StatusList2021Entry);
