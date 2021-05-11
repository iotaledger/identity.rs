// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

macro_rules! impl_wasm_accessors {
  ($name:ident, { $( $field:ident => $ty:ty ),+ $(,)* }) => {
    paste::paste! {
      #[wasm_bindgen]
      impl $name {
        $(
          #[wasm_bindgen(getter = [<$field:camel>])]
          pub fn [<$field>](&self) -> $ty {
            $crate::message::IntoWasm::into_wasm(self.0.$field())
          }

          #[wasm_bindgen(setter = [<$field:camel>])]
          pub fn [<set_ $field>](&mut self, value: $ty) {
            self.0.[<set_ $field>]($crate::message::IntoRust::into_rust(&value));
          }
        )+

        #[wasm_bindgen(js_name = toJSON)]
        pub fn to_json(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
          ::wasm_bindgen::JsValue::from_serde(&self.0).map_err($crate::utils::err)
        }

        #[wasm_bindgen(js_name = fromJSON)]
        pub fn from_json(value: &::wasm_bindgen::JsValue) -> Result<$name, ::wasm_bindgen::JsValue> {
          value.into_serde().map_err($crate::utils::err).map($name)
        }
      }
    }
  };
}
