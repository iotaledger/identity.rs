use wasm_bindgen::prelude::*;

use identity::account::Generation as Generation_;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen]
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct WasmGeneration(pub(crate) Generation_);

#[wasm_bindgen]
impl WasmGeneration {
  /// Creates a new `WasmGeneration`.
  #[wasm_bindgen]
  pub fn new() -> Self {
    WasmGeneration(Generation_::new())
  }

  /// Creates a new `WasmGeneration` from a 32-bit integer.
  #[wasm_bindgen]
  pub fn from_u32(value: u32) -> Self {
    WasmGeneration(Generation_::from_u32(value))
  }

  /// Returns the `WasmGeneration` as a 32-bit integer.
  #[wasm_bindgen]
  pub fn to_u32(self) -> u32 {
    self.0.to_u32()
  }

  /// Increments the `WasmGeneration`.
  ///
  /// # Errors
  ///
  /// Fails in case of overflows.
  #[wasm_bindgen]
  pub fn try_increment(self) -> Result<WasmGeneration> {
    self.0.try_increment().map(|x| x.into()).wasm_result()
  }

  /// Decrements the `WasmGeneration`.
  ///
  /// # Errors
  ///
  /// Fails in case of underflow.
  #[wasm_bindgen]
  pub fn try_decrement(self) -> Result<WasmGeneration> {
    self.0.try_decrement().map(|x| x.into()).wasm_result()
  }

  /// Returns a `WasmGeneration` of minimum value.
  #[wasm_bindgen]
  pub fn min() -> WasmGeneration {
    WasmGeneration(Generation_::MIN)
  }

  /// Returns a `WasmGeneration` of maximum value.
  #[wasm_bindgen]
  pub fn max() -> WasmGeneration {
    WasmGeneration(Generation_::MAX)
  }
}

impl From<Generation_> for WasmGeneration {
  fn from(generation: Generation_) -> Self {
    Self(generation)
  }
}

impl From<WasmGeneration> for Generation_ {
  fn from(wasm_generation: WasmGeneration) -> Self {
    wasm_generation.0
  }
}
