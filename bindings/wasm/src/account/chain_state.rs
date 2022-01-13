use wasm_bindgen::prelude::*;

use identity::account::ChainState as ChainState_;

use crate::tangle::WasmMessageId;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WasmChainState(pub(crate) ChainState_);

#[wasm_bindgen]
impl WasmChainState {
  #[wasm_bindgen]
  pub fn new() -> Self {
    WasmChainState(ChainState_::new())
  }

  /// Returns the integration message id of the last published update.
  ///
  /// Note: [`MessageId`] has a built-in `null` variant that needs to be checked for.
  #[wasm_bindgen]
  pub fn last_integration_message_id(&self) -> WasmMessageId {
    (*self.0.last_integration_message_id()).into()
  }

  /// Returns the diff message id of the last published update.
  ///
  /// Note: [`MessageId`] has a built-in `null` variant that needs to be checked for.
  #[wasm_bindgen]
  pub fn last_diff_message_id(&self) -> WasmMessageId {
    (*self.0.last_diff_message_id()).into()
  }

  /// Sets the last integration message id and resets the
  /// last diff message id to [`MessageId::null()`].
  #[wasm_bindgen]
  pub fn set_last_integration_message_id(&mut self, message: WasmMessageId) {
    self.0.set_last_integration_message_id(message.into())
  }

  /// Sets the last diff message id.
  #[wasm_bindgen]
  pub fn set_last_diff_message_id(&mut self, message: WasmMessageId) {
    self.0.set_last_diff_message_id(message.into())
  }

  /// Returns whether the identity has been published before.
  #[wasm_bindgen]
  pub fn is_new_identity(&self) -> bool {
    self.0.is_new_identity()
  }
}

impl Default for WasmChainState {
  fn default() -> Self {
    Self::new()
  }
}

impl From<ChainState_> for WasmChainState {
  fn from(chain_state: ChainState_) -> Self {
    WasmChainState(chain_state)
  }
}
