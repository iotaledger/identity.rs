// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::JsValue;

/// Convert errors to strings so they are readable in JS
pub fn err<T>(error: T) -> JsValue
where
  T: ToString,
{
  error.to_string().into()
}

// Alternative to implementing From for JsValue when deriving FlatEnum
// /// Convert errors that derive FlatEnum into JS
// pub fn flat_err<T, V>(error: T) -> JsValue
// where
//   T: flat_enum::IntoFlatEnum<V>,
//   V: flat_enum::FlatEnum,
// {
//   // TODO: check that unwrap is infallible here?
//   JsValue::from_serde(&error.into_flat_enum()).unwrap()
// }

#[cfg(test)]
mod tests {
  use flat_enum::IntoFlatEnum;

  #[test]
  fn test_js_error() {
    let err = identity::credential::Error::DIDError(
      identity::did::Error::CoreError(
        identity::core::Error::DecodeBitmap(
          std::io::Error::new(std::io::ErrorKind::Other, "something went wrong!")
        )
      )
    );
    println!("{}", err.to_string()); // Failed to decode roaring bitmap: something went wrong!
    let json_str = serde_json::to_string(&err.to_string()).unwrap();
    println!("{}", json_str); // "Failed to decode roaring bitmap: something went wrong!"
    let json = serde_json::to_string(&err.into_flat_enum()).unwrap();
    println!("{}", json); // {"code":"DIDError","description":"Failed to decode roaring bitmap: something went wrong!"}
  }
}
