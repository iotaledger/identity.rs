// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! log {
  ($($tt:tt)*) => {
    web_sys::console::log_1(&format!($($tt)*).into());
  }
}

#[macro_export]
macro_rules! impl_wasm_clone {
  ($wasm_class:ident, $js_class:ident) => {
    #[wasm_bindgen(js_class = $js_class)]
    impl $wasm_class {
      /// Deep clones the object.
      #[wasm_bindgen(js_name = clone)]
      pub fn deep_clone(&self) -> $wasm_class {
        return $wasm_class(self.0.clone());
      }
    }
  };
}
