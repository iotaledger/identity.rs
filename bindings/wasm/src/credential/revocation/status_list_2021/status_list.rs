use crate::error::Result;
use identity_iota::credential::status_list_2021::StatusList2021;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = StatusList2021, inspectable)]
pub struct WasmStatusList2021(pub(crate) StatusList2021);

impl_wasm_clone!(WasmStatusList2021, StatusList2021);

#[wasm_bindgen(js_class = StatusList2021)]
impl WasmStatusList2021 {
  /// Creates a new {@link StatusList2021} of `size` entries
  #[wasm_bindgen(constructor)]
  pub fn new(size: usize) -> Self {
    Self(StatusList2021::new(size))
  }

  /// Returns the number of entries in this {@link StatusList2021}
  #[wasm_bindgen]
  #[allow(clippy::len_without_is_empty)]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Returns whether the entry at `index` is set
  #[wasm_bindgen]
  pub fn get(&self, index: usize) -> Option<bool> {
    self.0.get(index)
  }

  /// Sets the value of the `index`-th entry
  #[wasm_bindgen]
  pub fn set(&mut self, index: usize, value: bool) {
    self.0.set(index, value)
  }

  /// Encodes this {@link StatusList2021} into its compressed
  /// base64 string representation
  #[wasm_bindgen(js_name = "intoEncodedStr")]
  pub fn into_encoded_str(self) -> String {
    self.0.into_encoded_str()
  }

  #[wasm_bindgen(js_name = "fromEncodedStr")]
  pub fn from_encoded_str(s: &str) -> Result<WasmStatusList2021> {
    StatusList2021::try_from_encoded_str(s)
      .map(Self)
      .map_err(|e| JsError::new(&e.to_string()).into())
  }
}
